use crate::{
    instructions::{call, immediate_to_register, jump, pop, register_memory_register, Mnemonic},
    parse_twos_complement_int,
    registers::{Register, RegisterMemory, SegmentRegister},
    Result,
};

type Thunk = fn(&mut std::slice::Iter<u8>) -> Result<Mnemonic>;

pub const OPCODE_TABLE: [Thunk; 168] = [
    // Add Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::ADD { dest, source })
    },
    // Add Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::ADD { dest, source })
    },
    // Add Reg8, Reg8/Mem8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::ADD { dest, source })
    },
    // Add Reg16, Reg16/Mem16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::ADD { dest, source })
    },
    // Add AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::ADD {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // Add AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::ADD {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // Push ES
    |_| Ok(Mnemonic::PUSHSEG(SegmentRegister::ES)),
    // Pop ES
    |_| Ok(Mnemonic::POPSEG(SegmentRegister::ES)),
    // Or Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::OR { dest, source })
    },
    // Or  Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::OR { dest, source })
    },
    // Or Reg8, Reg8/Mem8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::OR { dest, source })
    },
    // Or Reg16, Reg16/Mem16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::OR { dest, source })
    },
    // Or AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::OR {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // Or AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::OR {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // Push CS
    |_| Ok(Mnemonic::PUSHSEG(SegmentRegister::CS)),
    |_| Ok(Mnemonic::NOP),
    // Add with carry, Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::ADC { dest, source })
    },
    // Add w. carry, Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::ADC { dest, source })
    },
    // Add w. carry, Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::ADC { dest, source })
    },
    // Add w. carry, Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::ADC { dest, source })
    },
    // ADC AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::ADC {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // ADC AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::ADC {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // Push SS
    |_| Ok(Mnemonic::PUSHSEG(SegmentRegister::SS)),
    |_| Ok(Mnemonic::POPSEG(SegmentRegister::SS)),
    // Subtract w. borrow, Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::SBB { dest, source })
    },
    // Subtract w. borrow, Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::SBB { dest, source })
    },
    // Subtract w. borrow, Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::SBB { dest, source })
    },
    // Subtract w. borrow, Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::SBB { dest, source })
    },
    // SBB AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::SBB {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // SBB AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::SBB {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // PUSH & POP DS
    |_| Ok(Mnemonic::PUSHSEG(SegmentRegister::DS)),
    |_| Ok(Mnemonic::POPSEG(SegmentRegister::DS)),
    // AND Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::AND { dest, source })
    },
    // AND, Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::AND { dest, source })
    },
    // AND Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::AND { dest, source })
    },
    // AND Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::AND { dest, source })
    },
    // AND AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::AND {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // AND AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::AND {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // segment override prefix (ES)
    |_| Ok(Mnemonic::SEGMENTOVERRIDE(SegmentRegister::ES)),
    // increment adjust for add
    |_| Ok(Mnemonic::DAA),
    // Subtract, Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::SUB { dest, source })
    },
    // Subtract, Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::SUB { dest, source })
    },
    // Subtract, Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::SUB { dest, source })
    },
    // Subtract, Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::SUB { dest, source })
    },
    // Sub AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::SUB {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // SUB AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::SUB {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // segment override prefix (CS)
    |_| Ok(Mnemonic::SEGMENTOVERRIDE(SegmentRegister::CS)),
    // increment adjust for subtract
    |_| Ok(Mnemonic::DAS),
    // XOR Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::XOR { dest, source })
    },
    // XOR Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::XOR { dest, source })
    },
    // XOR Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::XOR { dest, source })
    },
    // XOR Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::XOR { dest, source })
    },
    // XOR AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::XOR {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // XOR AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::XOR {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // segment override prefix (SS)
    |_| Ok(Mnemonic::SEGMENTOVERRIDE(SegmentRegister::SS)),
    // ascii adjust for add
    |_| Ok(Mnemonic::AAA),
    // CMP Reg8/Mem8, Reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::CMP { dest, source })
    },
    // CMP Reg16/Mem16, Reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::CMP { dest, source })
    },
    // CMP Reg8, Reg8/Mem8,
    |iter| {
        let (dest, source) = register_memory_register(false, iter, true)?;
        Ok(Mnemonic::CMP { dest, source })
    },
    // CMP Reg16, Reg16/Mem16,
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::CMP { dest, source })
    },
    // CMP AL, Immediate8
    |iter| {
        let data = iter.next().unwrap();
        let operand = parse_twos_complement_int(*data as isize, false);
        return Ok(Mnemonic::CMP {
            dest: RegisterMemory::Register(Register::AL),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // CMP AX, Immediate16
    |iter| {
        let data = u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]);
        let operand = parse_twos_complement_int(data as isize, true);
        return Ok(Mnemonic::CMP {
            dest: RegisterMemory::Register(Register::AX),
            source: RegisterMemory::Immediate(operand),
        });
    },
    // segment override prefix (DS)
    |_| Ok(Mnemonic::SEGMENTOVERRIDE(SegmentRegister::DS)),
    // ascii adjust for subtract
    |_| Ok(Mnemonic::AAS),
    // Increment register
    |_| Ok(Mnemonic::INC(Register::AX)),
    |_| Ok(Mnemonic::INC(Register::CX)),
    |_| Ok(Mnemonic::INC(Register::DX)),
    |_| Ok(Mnemonic::INC(Register::BX)),
    |_| Ok(Mnemonic::INC(Register::SP)),
    |_| Ok(Mnemonic::INC(Register::BP)),
    |_| Ok(Mnemonic::INC(Register::SI)),
    |_| Ok(Mnemonic::INC(Register::DI)),
    // Decrement register
    |_| Ok(Mnemonic::DEC(Register::AX)),
    |_| Ok(Mnemonic::DEC(Register::CX)),
    |_| Ok(Mnemonic::DEC(Register::DX)),
    |_| Ok(Mnemonic::DEC(Register::BX)),
    |_| Ok(Mnemonic::DEC(Register::SP)),
    |_| Ok(Mnemonic::DEC(Register::BP)),
    |_| Ok(Mnemonic::DEC(Register::SI)),
    |_| Ok(Mnemonic::DEC(Register::DI)),
    // Push to register
    |_| Ok(Mnemonic::PUSH(Register::AX)),
    |_| Ok(Mnemonic::PUSH(Register::CX)),
    |_| Ok(Mnemonic::PUSH(Register::DX)),
    |_| Ok(Mnemonic::PUSH(Register::BX)),
    |_| Ok(Mnemonic::PUSH(Register::SP)),
    |_| Ok(Mnemonic::PUSH(Register::BP)),
    |_| Ok(Mnemonic::PUSH(Register::SI)),
    |_| Ok(Mnemonic::PUSH(Register::DI)),
    // Pop from register
    |_| Ok(Mnemonic::POP(Register::AX)),
    |_| Ok(Mnemonic::POP(Register::CX)),
    |_| Ok(Mnemonic::POP(Register::DX)),
    |_| Ok(Mnemonic::POP(Register::BX)),
    |_| Ok(Mnemonic::POP(Register::SP)),
    |_| Ok(Mnemonic::POP(Register::BP)),
    |_| Ok(Mnemonic::POP(Register::SI)),
    |_| Ok(Mnemonic::POP(Register::DI)),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    // Jump if overflow
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JO { label })
    },
    // Jump not overflow
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNO { label })
    },
    // Jump on below
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JB { label })
    },
    // Jump on not below
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNB { label })
    },
    // Jump on equal
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JE { label })
    },
    // Jump on not equal
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNE { label })
    },
    // Jump on below or equal/not above
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JBE { label })
    },
    // Jump on not below or equal/above
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNBE { label })
    },
    // Jump on sign
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JS { label })
    },
    // Jump on not sign
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNS { label })
    },
    // Jump on parity
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JP { label })
    },
    // Jump on not parity
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNP { label })
    },
    // Jump on less
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JL { label })
    },
    // Jump on not less
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNL { label })
    },
    // Jump on less or equal
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JLE { label })
    },
    // Jump on not less or equal
    |iter| {
        let label = jump(iter)?;
        Ok(Mnemonic::JNLE { label })
    },
    // ADD/OR/ADC/SBB/AND/SUB/XOR/CMP Immediate to register
    |iter| {
        let instruction = immediate_to_register(false, iter)?;
        Ok(instruction)
    },
    // ADD/OR/ADC/SBB/AND/SUB/XOR/CMP Immediate to register (wide)
    |iter| {
        let instruction = immediate_to_register(true, iter)?;
        Ok(instruction)
    },
    // ADD/ADC/SBB/SUB/CMP immediate to register
    |iter| {
        let instruction = immediate_to_register(false, iter)?;
        Ok(instruction)
    },
    // ADD/ADC/SBB/SUB/CMP immediate (8bit) to register (wide)
    |iter| {
        let instruction = immediate_to_register(false, iter)?;
        Ok(instruction)
    },
    // TEST reg8/mem8, reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::TEST { dest, source })
    },
    // TEST reg16/mem16, reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::TEST { dest, source })
    },
    // XCHG reg8/mem8, reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::XCHG { dest, source })
    },
    // XCHG reg16/mem16, reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::XCHG { dest, source })
    },
    // MOV reg8/mem8, reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::MOV { dest, source })
    },
    // MOV reg16/mem16, reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::MOV { dest, source })
    },
    // MOV reg8, mem8/reg8
    |iter| {
        let (dest, source) = register_memory_register(false, iter, false)?;
        Ok(Mnemonic::MOV { dest, source })
    },
    // MOV reg16, mem16/reg16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::MOV { dest, source })
    },
    // TODO: MOV reg16/mem16, SEGREG
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::MOV { dest, source })
    },
    |_| Ok(Mnemonic::NOP),
    // LEA REG16,MEM16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, false)?;
        Ok(Mnemonic::LEA { dest, source })
    },
    // TODO: MOV SEGREG, reg16/mem16
    |iter| {
        let (dest, source) = register_memory_register(true, iter, true)?;
        Ok(Mnemonic::MOV { dest, source })
    },
    |_| Ok(Mnemonic::NOP),
    |iter| {
        pop(true, iter)
    },
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    |_| Ok(Mnemonic::NOP),
    // Exchange AX, AX ??
    |_| Ok(Mnemonic::NOP),
    // XCHG instructions
    |_| Ok(Mnemonic::XCHG { dest: RegisterMemory::Register(Register::AX), source: RegisterMemory::Register(Register::CX) }),
    |_| Ok(Mnemonic::XCHG { dest: RegisterMemory::Register(Register::AX), source: RegisterMemory::Register(Register::DX) }),
    |_| Ok(Mnemonic::XCHG { dest: RegisterMemory::Register(Register::AX), source: RegisterMemory::Register(Register::BX) }),
    |_| Ok(Mnemonic::XCHG { dest: RegisterMemory::Register(Register::AX), source: RegisterMemory::Register(Register::SP) }),
    |_| Ok(Mnemonic::XCHG { dest: RegisterMemory::Register(Register::AX), source: RegisterMemory::Register(Register::BP) }),
    |_| Ok(Mnemonic::XCHG { dest: RegisterMemory::Register(Register::AX), source: RegisterMemory::Register(Register::SI) }),
    |_| Ok(Mnemonic::XCHG { dest: RegisterMemory::Register(Register::AX), source: RegisterMemory::Register(Register::DI) }),
    |_| Ok(Mnemonic::CWD),
    |iter| {
        call(iter)
    },
    |_| Ok(Mnemonic::WAIT),
    |_| Ok(Mnemonic::PUSHF),
    |_| Ok(Mnemonic::POPF),
    |_| Ok(Mnemonic::SAHF),
    |_| Ok(Mnemonic::LAHF),
];

#[cfg(test)]
mod tests {
    use crate::registers::RegisterMemory;
    use crate::{instructions::Mnemonic, opcodes::OPCODE_TABLE, registers::Register};
    #[test]
    fn test_immediate_to_register_wide() {
        let binary = [0b10000001, 0b11001001, 0b00100110, 0b00000000];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Mnemonic::OR {
                dest: RegisterMemory::Register(Register::CX),
                source: RegisterMemory::Immediate(38)
            }
        )
    }
    #[test]
    fn test_immediate_to_register() {
        let binary = [0b10000000, 0b11000001, 0b00100110];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Mnemonic::ADD {
                dest: RegisterMemory::Register(Register::CL),
                source: RegisterMemory::Immediate(38)
            }
        )
    }
    #[test]
    fn test_or_register_memory_register() {
        let binary = [0b00001000, 0b00000100];
        let mut iter = binary.iter();
        let byte = iter.next().unwrap();
        let instruction = (OPCODE_TABLE[*byte as usize])(&mut iter).unwrap();
        assert_eq!(
            instruction,
            Mnemonic::OR {
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
            Mnemonic::OR {
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
            Mnemonic::ADD {
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
            Mnemonic::ADD {
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
            Mnemonic::ADD {
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
            Mnemonic::ADD {
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
            Mnemonic::ADD {
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
            Mnemonic::ADD {
                dest: RegisterMemory::CombineRegistersData(Register::BX, Register::SI, 69),
                source: RegisterMemory::Register(Register::AL)
            }
        );
    }
}
