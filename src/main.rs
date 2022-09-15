
extern crate win32;

use win32::X86;

use anyhow::bail;

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let exe = match args.as_slice() {
        [_, exe] => exe,
        _ => bail!("specify path"),
    };

    let mut x86 = X86::new();
    let buf = std::fs::read(exe)?;
    win32::load_exe(&mut x86, &buf)?;

    let decoder = iced_x86::Decoder::with_ip(
        32,
        &x86.mem[x86.regs.eip as usize..],
        x86.regs.eip as u64,
        iced_x86::DecoderOptions::NONE,
    );

    let mut i = 0;
    for instruction in decoder {
        print!("{:08X} ", instruction.ip());
        let start_index = instruction.ip() as usize;
        let instr_bytes = &x86.mem[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02x}", b);
        }
        if instr_bytes.len() < 10 {
            for _ in 0..10 - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", instruction);
        i += 1;
        if i > 20 {
            break;
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
