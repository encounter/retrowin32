# See doc/x86-64 section "Trampoline assembly".

tramp32:
    calll *%eax
    lretl   # long ret to 64-bit mode

# This code is the x86 entry point for 32->64bit calls.
# We stamp out a separate copy of it per entry point.
tramp64:
    # Argument passed to call64 trampoline, to be set to correct value:
    pushl $0x99999999 # hi 4 bytes of 64bit target
    pushl $0x11111111 # lo 4 bytes of 64bit target

    lcall *0xaaaaaaaa # 16:32 of call64

    # stack contents are now:
    #   4 bytes return addr in exe
    #   N bytes arguments passed via stdcall

    retl $20  # clean stdcall args, adjust value here as appropriate

crash:
    int3
    movl $1, %eax
    jmpl *%eax
