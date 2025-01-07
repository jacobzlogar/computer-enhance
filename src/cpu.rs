use std::collections::HashMap;

use crate::instructions::Mnemonic;
use crate::registers::{Register, RegisterMemory, SegmentRegister};
use crate::Result;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CpuFlag {
    PF,
    ZF,
    OF,
    DF,
    IF,
    TF,
    SF,
    AF,
    CF
}

#[derive(Debug)]
pub struct Cpu {
    flags: HashMap<CpuFlag, bool>,
    pub registers: HashMap<Register, isize>,
    segment_registers: HashMap<SegmentRegister, isize>
}


impl Cpu {
    pub fn new() -> Self {
        let mut segment_registers: HashMap<SegmentRegister, isize> = HashMap::new();
        segment_registers.insert(SegmentRegister::ES, 0);
        segment_registers.insert(SegmentRegister::CS, 0);
        segment_registers.insert(SegmentRegister::SS, 0);
        segment_registers.insert(SegmentRegister::DS, 0);
        let mut registers: HashMap<Register, isize> = HashMap::new();
        registers.insert(Register::AL, 0);
        registers.insert(Register::CL, 0);
        registers.insert(Register::BL, 0);
        registers.insert(Register::DL, 0);
        registers.insert(Register::AH, 0);
        registers.insert(Register::CH, 0);
        registers.insert(Register::DH, 0);
        registers.insert(Register::BH, 0);
        registers.insert(Register::AX, 0);
        registers.insert(Register::CX, 0);
        registers.insert(Register::BX, 0);
        registers.insert(Register::DX, 0);
        registers.insert(Register::SP, 0);
        registers.insert(Register::BP, 0);
        registers.insert(Register::SI, 0);
        registers.insert(Register::DI, 0);
        let mut flags: HashMap<CpuFlag, bool> = HashMap::new();
        flags.insert(CpuFlag::PF, false);
        flags.insert(CpuFlag::ZF, false);
        flags.insert(CpuFlag::OF, false);
        flags.insert(CpuFlag::DF, false);
        flags.insert(CpuFlag::IF, false);
        flags.insert(CpuFlag::TF, false);
        flags.insert(CpuFlag::SF, false);
        flags.insert(CpuFlag::AF, false);
        flags.insert(CpuFlag::CF, false);
        Self {
            flags,
            registers,
            segment_registers
        }
    }
    pub fn execute(&mut self, instruction: Mnemonic) -> Result<()> {
        match instruction {
            Mnemonic::MOV {dest, source} => {
                self.mov(dest, source)?;
            }
            _ => ()
        }
        Ok(())
    }
    fn mov(&mut self, dest: RegisterMemory, source: RegisterMemory) -> Result<()> {
        println!("{:?} {:?}", dest, source);
        match (dest, source) {
            (RegisterMemory::Register(register), RegisterMemory::Immediate(value)) => {
                self.registers.entry(register).and_modify(|v| *v = value);
            }
            (RegisterMemory::Register(dest), RegisterMemory::Register(source)) => {
                let source_value = *self.registers.get(&source).unwrap();
                *self.registers.entry(dest)
                    .or_insert(0) = source_value;
            }
            (_, _) => ()
        }
        Ok(())
    }
}
