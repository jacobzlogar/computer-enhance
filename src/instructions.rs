use crate::{
    registers::{Mode, Register, RegisterEncoding, RegisterMemory, RegisterMemoryEncoding, SegmentRegister},
    Result,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    // MOV,
    INC(Register),
    DEC(Register),
    SEGMENTOVERRIDE(SegmentRegister),
    AAS,
    AAA,
    DAA,
    DAS,
    NOP,
    PUSH(Register),
    POP(Register),
    PUSHSEG(SegmentRegister),
    POPSEG(SegmentRegister),
    CMP {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    OR {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    ADD {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    ADC {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    SBB {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    SUB {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    XOR {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    JO,
    JE,
    JL,
    JLE,
    JB,
    JBE,
    JP,
    JS,
    JNE,
    JNL,
    JNLE,
    JNB,
    JNBE,
    JNP,
    JNO,
    JNS
    // Incrememt,
    // Subtract,
    // SubtractWithBorrow,
    // Decrement,
    // Compare,
    // Logic,
    // And,
    // Test,
    // ExclusiveOr,
    // Repeat,
    // MoveByte,
    // CompareByte,
    // ScanByte,
    // LoadByte,
    // StoreByte,
    // Call,
    // UnconditionalJump,
    // ReturnFromCall,
    // Interrupt,
    // ProcessorControl,
}

fn get_mode(byte: &u8) -> Result<Mode> {
    let mode = Mode::try_from(byte >> 6)?;
    Ok(mode)
}

pub fn register_memory_register<'a>(
    wide: bool,
    iter: &'a mut std::slice::Iter<'a, u8>,
    reversed: bool,
) -> Result<(RegisterMemory, RegisterMemory)> {
    let data_byte = iter.next().unwrap();
    let mode = get_mode(&data_byte)?;
    let (source, dest): (RegisterMemory, RegisterMemory);
    if reversed {
        let encoding = RegisterMemoryEncoding {
            mode,
            rm: data_byte & 7,
            wide,
            iter
        };
        dest = RegisterMemory::try_from(encoding)?;
        source = RegisterMemory::try_from(RegisterEncoding {
            byte: (data_byte >> 3) & 7,
            wide,
        })?;
    } else {
        let encoding = RegisterMemoryEncoding {
            mode,
            rm: data_byte & 7,
            wide,
            iter
        };
        source = RegisterMemory::try_from(encoding)?;
        dest = RegisterMemory::try_from(RegisterEncoding {
            byte: (data_byte >> 3) & 7,
            wide,
        })?;
    }
    Ok((dest, source))
}
