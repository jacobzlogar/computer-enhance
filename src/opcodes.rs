use crate::{
    instructions::{register_memory_register, Instruction},
    parse_twos_complement_int,
    registers::{Register, RegisterMemory, SegmentRegister},
    Result,
};

type Thunk = for<'a> fn(&'a mut std::slice::Iter<'a, u8>) -> Result<Instruction>;

const OPCODE_TABLE: [Thunk; 107] = [
    // Add Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        return Ok(Instruction::ADD { dest, source });
    },
    // Add Reg16/Mem16, Reg16
    |iter| {
        let (dest,source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::ADD { dest, source });
    },
    // Add Reg8, Reg8/Mem8
    |iter| {
        let (dest,source) = register_memory_register(false, iter, true)?;
        return Ok(Instruction::ADD { dest, source });
    },
    // Add Reg16, Reg16/Mem16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        return Ok(Instruction::ADD { dest, source });
    },
    // Add AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Instruction::ADD {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // Add AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Instruction::ADD {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },

    // Push ES
    |_| Ok(Instruction::PUSHSEG(SegmentRegister::ES)),
    // Pop ES
    |_| Ok(Instruction::POPSEG(SegmentRegister::ES)),

    // Or Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        return Ok(Instruction::OR { dest, source });
    },
    // Or  Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::OR { dest, source });
    },
    // Or Reg8, Reg8/Mem8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        return Ok(Instruction::OR { dest, source });
    },
    // Or Reg16, Reg16/Mem16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::OR { dest, source });
    },
    // Or AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Instruction::OR {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // Or AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Instruction::OR {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },

    // Push CS
    |_| Ok(Instruction::PUSHSEG(SegmentRegister::CS)),
    |_| Ok(Instruction::NOP),

    // Add with carry, Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        return Ok(Instruction::ADC { dest, source });
    },
    // Add w. carry, Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::ADC { dest, source });
    },
    // Add w. carry, Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        return Ok(Instruction::ADC { dest, source });
    },
    // Add w. carry, Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        return Ok(Instruction::ADC { dest, source });
    },
    // ADC AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Instruction::ADC {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // ADC AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Instruction::ADC {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },

    // Push SS
    |_| Ok(Instruction::PUSHSEG(SegmentRegister::SS)),
    |_| Ok(Instruction::POPSEG(SegmentRegister::SS)),

    // Subtract w. borrow, Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        return Ok(Instruction::SBB { dest, source });
    },
    // Subtract w. borrow, Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::SBB { dest, source });
    },
    // Subtract w. borrow, Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        return Ok(Instruction::SBB { dest, source });
    },
    // Subtract w. borrow, Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        return Ok(Instruction::SBB { dest, source });
    },
    // SBB AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Instruction::SBB {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // SBB AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Instruction::SBB {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },

    // PUSH & POP DS
    |_| Ok(Instruction::PUSHSEG(SegmentRegister::DS)),
    |_| Ok(Instruction::POPSEG(SegmentRegister::DS)),

    // segment override prefix (ES)
    |_| Ok(Instruction::SEGMENTOVERRIDE(SegmentRegister::ES)),
    // increment adjust for add
    |_| Ok(Instruction::DAA),

    // Subtract, Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        return Ok(Instruction::SUB { dest, source });
    },
    // Subtract, Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::SUB { dest, source });
    },
    // Subtract, Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        return Ok(Instruction::SUB { dest, source });
    },
    // Subtract, Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        return Ok(Instruction::SUB { dest, source });
    },
    // Sub AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Instruction::SUB {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // SUB AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Instruction::SUB {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },

    // segment override prefix (CS)
    |_| Ok(Instruction::SEGMENTOVERRIDE(SegmentRegister::CS)),
    // increment adjust for subtract
    |_| Ok(Instruction::DAS),

    // XOR Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        return Ok(Instruction::XOR { dest, source });
    },
    // XOR Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::XOR { dest, source });
    },
    // XOR Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        return Ok(Instruction::XOR { dest, source });
    },
    // XOR Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        return Ok(Instruction::XOR { dest, source });
    },
    // XOR AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Instruction::XOR {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // XOR AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Instruction::XOR {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },

    // segment override prefix (SS)
    |_| Ok(Instruction::SEGMENTOVERRIDE(SegmentRegister::SS)),
    // ascii adjust for add
    |_| Ok(Instruction::AAA),

    // CMP Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        return Ok(Instruction::CMP { dest, source });
    },
    // CMP Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        return Ok(Instruction::CMP { dest, source });
    },
    // CMP Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        return Ok(Instruction::CMP { dest, source });
    },
    // CMP Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        return Ok(Instruction::CMP { dest, source });
    },
    // CMP AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Instruction::CMP {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // CMP AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Instruction::CMP {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    
    // segment override prefix (DS)
    |_| Ok(Instruction::SEGMENTOVERRIDE(SegmentRegister::DS)),
    // ascii adjust for subtract
    |_| Ok(Instruction::AAS),

    // Increment register
    |_| Ok(Instruction::INC(Register::AX)),
    |_| Ok(Instruction::INC(Register::CX)),
    |_| Ok(Instruction::INC(Register::DX)),
    |_| Ok(Instruction::INC(Register::BX)),
    |_| Ok(Instruction::INC(Register::SP)),
    |_| Ok(Instruction::INC(Register::BP)),
    |_| Ok(Instruction::INC(Register::SI)),
    |_| Ok(Instruction::INC(Register::DI)),

    // Decrement register
    |_| Ok(Instruction::DEC(Register::AX)),
    |_| Ok(Instruction::DEC(Register::CX)),
    |_| Ok(Instruction::DEC(Register::DX)),
    |_| Ok(Instruction::DEC(Register::BX)),
    |_| Ok(Instruction::DEC(Register::SP)),
    |_| Ok(Instruction::DEC(Register::BP)),
    |_| Ok(Instruction::DEC(Register::SI)),
    |_| Ok(Instruction::DEC(Register::DI)),

    // Push to register
    |_| Ok(Instruction::PUSH(Register::AX)),
    |_| Ok(Instruction::PUSH(Register::CX)),
    |_| Ok(Instruction::PUSH(Register::DX)),
    |_| Ok(Instruction::PUSH(Register::BX)),
    |_| Ok(Instruction::PUSH(Register::SP)),
    |_| Ok(Instruction::PUSH(Register::BP)),
    |_| Ok(Instruction::PUSH(Register::SI)),
    |_| Ok(Instruction::PUSH(Register::DI)),

    // Pop from register
    |_| Ok(Instruction::POP(Register::AX)),
    |_| Ok(Instruction::POP(Register::CX)),
    |_| Ok(Instruction::POP(Register::DX)),
    |_| Ok(Instruction::POP(Register::BX)),
    |_| Ok(Instruction::POP(Register::SP)),
    |_| Ok(Instruction::POP(Register::BP)),
    |_| Ok(Instruction::POP(Register::SI)),
    |_| Ok(Instruction::POP(Register::DI)),

    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    |_| Ok(Instruction::NOP),
    // Jump if overflow
    |iter| {
        return Ok(Instruction::JO);
    },
];

#[cfg(test)]
mod tests {
    use crate::registers::RegisterMemory;
    use crate::{instructions::Instruction, opcodes::OPCODE_TABLE, registers::Register};
    #[test]
    fn test_or_register_memory_register() {
        let binary = [0b00001000, 0b00000100];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Instruction::OR {
                dest: RegisterMemory::Register(Register::AL),
                source: RegisterMemory::Register(Register::SI),
            }
        );
    }

    #[test]
    fn test_or_register_memory_register_displacement() {
        let binary = [0b00001000, 0b01000001, 0b01000101];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Instruction::OR {
                dest: RegisterMemory::Register(Register::AL),
                source: RegisterMemory::CombineRegistersData(Register::BX, Register::DI, 69),
            }
        );
    }
    #[test]
    fn test_add_immediate() {
        let binary = [0b00000100, 0b01000101];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Instruction::ADD {
                dest: RegisterMemory::Register(Register::AL),
                source: RegisterMemory::Immediate(69),
            }
        );
    }
    #[test]
    fn test_add_immediate_wide() {
        let binary = [0b00000101, 0b00000000, 0b00000010];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        println!("{:?}", instruction);
        assert_eq!(
            instruction,
            Instruction::ADD {
                dest: RegisterMemory::Register(Register::AX),
                source: RegisterMemory::Immediate(512),
            }
        );
    }
    #[test]
    fn test_add_register_memory() {
        let binary = [0b00000001, 0b00000000];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Instruction::ADD {
                source: RegisterMemory::CombineRegisters(Register::BX, Register::SI),
                dest: RegisterMemory::Register(Register::AX)
            }
        );
    }
    #[test]
    fn test_add_register_memory_reverse_wide() {
        let binary = [0b00000011, 0b00000000];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Instruction::ADD {
                dest: RegisterMemory::CombineRegisters(Register::BX, Register::SI),
                source: RegisterMemory::Register(Register::AX)
            }
        );
    }
    #[test]
    fn test_add_register_memory_displacement() {
        let binary = [0b00000000, 0b01000000, 0b01000101];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Instruction::ADD {
                source: RegisterMemory::CombineRegistersData(Register::BX, Register::SI, 69),
                dest: RegisterMemory::Register(Register::AL)
            }
        );
    }
    #[test]
    fn test_add_register_memory_displacement_reverse() {
        let binary = [0b00000010, 0b01000000, 0b01000101];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Instruction::ADD {
                dest: RegisterMemory::CombineRegistersData(Register::BX, Register::SI, 69),
                source: RegisterMemory::Register(Register::AL)
            }
        );
    }
}
