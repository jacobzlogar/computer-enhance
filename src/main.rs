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
        // for opcode in OPCODE_TABLE {
        //     if (*byte >> opcode.bit_shift) == opcode.mask {
        //         match opcode.opcode {
        //             Opcode::Mov(mnemonic) => {
        //                 let mut op = Mov::new(mnemonic.clone(), byte.clone());
        //                 op.parse(&mut iter.clone())?;
        //                 cpu.mov(op.dest.clone(), op.source.clone(), op.value);
        //                 // println!("{op}");
        //             }
        //             Opcode::Arithmetic(mnemonic) => {
        //                 println!("{:08b}", byte);
        //                 let next = iter.next().unwrap();
        //                 println!("{:08b}", next);
        //             }
        //         }
        //     }
        // }
    }
    Ok(())
}
