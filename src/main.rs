use computer_enhance::{cpu::Cpu, opcodes::OPCODE_TABLE, Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: String,
    #[arg(short, long)]
    execute: bool
}

fn main() -> Result<()> {
    let args = Args::parse();
    // let args: Vec<String> = std::env::args().collect();
    let path = format!("{}/listings/part1/{}", env!("CARGO_MANIFEST_DIR"), args.filename);
    let binary = std::fs::read(path)?;
    let mut cpu = Cpu::new();
    let mut iter = binary.iter();
    while let Some(byte) = iter.next() {
        // println!("{:08b}", byte);
        let opcode_fn = OPCODE_TABLE[*byte as usize];
        let instruction = (opcode_fn)(&mut iter)?;
        println!("{:?}", instruction);
        if args.execute {
            cpu.execute(instruction)?;
        }
    }
    println!("{:?}", cpu.registers);
    Ok(())
}
