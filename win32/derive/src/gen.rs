use super::parse;
use crate::parse::DllExport;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate the wrapper function that calls a win32api function by taking arguments using from_x86.
///
/// The caller of winapi functions is responsible for pushing/popping the
/// return address, because some callers actually 'jmp' directly.
///
/// This macro generates shim wrappers of functions, taking their
/// input args off the stack and forwarding their return values via eax.
pub fn fn_wrapper(module: TokenStream, dllexport: &DllExport) -> (TokenStream, TokenStream) {
    let name = &dllexport.func.sig.ident;
    let name_str = name.to_string();

    let mut fetch_args = TokenStream::new();
    fetch_args.extend(quote!(let mem = machine.mem().detach();));
    let mut stack_offset = 4u32; // return address
    for parse::Argument { name, ty, stack } in dllexport.args.iter() {
        // We expect all the stack_offset math to be inlined by the compiler into plain constants.
        // TODO: reading the args in reverse would produce fewer bounds checks...
        fetch_args.extend(quote! {
            let #name = <#ty>::from_stack(mem, esp + #stack_offset);
        });
        stack_offset += stack.consumed();
    }

    let stack_consumed = dllexport.stack_consumed();

    // If the function is async, we need to handle the return value a bit differently.
    let is_async = dllexport.func.sig.asyncness.is_some();
    let args = dllexport
        .args
        .iter()
        .map(|arg| arg.name)
        .collect::<Vec<_>>();
    let (func, body) = if dllexport.func.sig.asyncness.is_some() {
        (
            quote!(shims::Handler::Async(impls::#name)),
            quote! {
                #fetch_args
                let machine: *mut Machine = machine;
                Box::pin(async move {
                    let machine = unsafe { &mut *machine };
                    #module::#name(machine, #(#args),*).await.to_raw()
                })
            },
        )
    } else {
        (
            quote!(shims::Handler::Sync(impls::#name)),
            quote! {
                #fetch_args
                #module::#name(machine, #(#args),*).to_raw()
            },
        )
    };

    let retn = if is_async {
        quote!(crate::shims::BoxFuture<u32>)
    } else {
        quote!(u32)
    };
    (
        quote!(pub unsafe fn #name(machine: &mut Machine, esp: u32) -> #retn { #body }),
        quote!(pub const #name: shims::Shim = shims::Shim {
            name: #name_str,
            func: #func,
            stack_consumed: #stack_consumed,
        };),
    )
}
