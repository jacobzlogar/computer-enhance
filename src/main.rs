use computer_enhance::{opcodes::OPCODE_TABLE, Result};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = format!(
        "{}/listings/part1/{}",
        env!("CARGO_MANIFEST_DIR"),
        args[1]
    );
    let binary = std::fs::read(path)?;
    let mut iter = binary.iter();
    while let Some(byte) = iter.next() {
        let opcode_fn = OPCODE_TABLE[*byte as usize];
        let instruction = (opcode_fn)(&mut iter)?;
        println!("{:?}", instruction);
    }
    Ok(())
}
