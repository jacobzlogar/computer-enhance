use crate::{
    registers::{
        Mode, Register, RegisterEncoding, RegisterMemory, RegisterMemoryEncoding, SegmentRegister,
    },
    Result,
};

#[derive(Debug, Eq, PartialEq)]
pub enum ImmediateMode {
    ADD,
    OR,
    ADC,
    SBB,
    AND,
    SUB,
    XOR,
    CMP,
}

impl TryFrom<u8> for ImmediateMode {
    type Error = Box<dyn std::error::Error + 'static>;
    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(ImmediateMode::ADD),
            1 => Ok(ImmediateMode::OR),
            2 => Ok(ImmediateMode::ADC),
            3 => Ok(ImmediateMode::SBB),
            4 => Ok(ImmediateMode::AND),
            5 => Ok(ImmediateMode::SUB),
            6 => Ok(ImmediateMode::XOR),
            7 => Ok(ImmediateMode::CMP),
            _ => Err("Not an immediate mode mapping".into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    WAIT,
    PUSHF,
    POPF,
    SAHF,
    LAHF,
    CWD,
    CALL {
        far_proc: usize
    },
    LEA {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    MOV {
        dest: RegisterMemory,
        source: RegisterMemory
    },
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
    XCHG {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    TEST {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    CMP {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    OR {
        dest: RegisterMemory,
        source: RegisterMemory,
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
        source: RegisterMemory,
    },
    SUB {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    XOR {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    JO {
        label: u8,
    },
    JE {
        label: u8,
    },
    JL {
        label: u8,
    },
    JLE {
        label: u8,
    },
    JB {
        label: u8,
    },
    JBE {
        label: u8,
    },
    JP {
        label: u8,
    },
    JS {
        label: u8,
    },
    JNE {
        label: u8,
    },
    JNL {
        label: u8,
    },
    JNLE {
        label: u8,
    },
    JNB {
        label: u8,
    },
    JNBE {
        label: u8,
    },
    JNP {
        label: u8,
    },
    JNO {
        label: u8,
    },
    JNS {
        label: u8,
    },
    AND {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
}

pub struct ImmediateModeEncoding<I> {
    mode: ImmediateMode,
    dest: RegisterMemory,
    wide: bool,
    iter: I
}

impl<'a, I: Iterator<Item = &'a u8>> TryFrom<ImmediateModeEncoding<I>> for Instruction {
    type Error = Box<dyn std::error::Error + 'static>;
    fn try_from(mut value: ImmediateModeEncoding<I>) -> Result<Self> {
        let source: RegisterMemory;
        if value.wide {
            let data =
                u16::from_le_bytes([*value.iter.next().unwrap(), *value.iter.next().unwrap()]);
            source = RegisterMemory::Immediate(data as isize);
        } else {
            source = RegisterMemory::Immediate(*value.iter.next().unwrap() as isize);
        }
        match value.mode {
            ImmediateMode::ADD => Ok(Instruction::ADD {
                dest: value.dest,
                source,
            }),
            ImmediateMode::OR => Ok(Instruction::OR {
                dest: value.dest,
                source,
            }),
            ImmediateMode::ADC => Ok(Instruction::ADC {
                dest: value.dest,
                source,
            }),
            ImmediateMode::SBB => Ok(Instruction::SBB {
                dest: value.dest,
                source,
            }),
            ImmediateMode::AND => Ok(Instruction::AND {
                dest: value.dest,
                source,
            }),
            ImmediateMode::SUB => Ok(Instruction::SUB {
                dest: value.dest,
                source,
            }),
            ImmediateMode::XOR => Ok(Instruction::XOR {
                dest: value.dest,
                source,
            }),
            ImmediateMode::CMP => Ok(Instruction::CMP {
                dest: value.dest,
                source,
            }),
        }
    }
}

fn get_mode(byte: &u8) -> Result<Mode> {
    let mode = Mode::try_from(byte >> 6)?;
    Ok(mode)
}

pub fn register_memory_register<'a, I: Iterator<Item = &'a u8>>(
    wide: bool,
    mut iter: I,
    reversed: bool,
) -> Result<(RegisterMemory, RegisterMemory)> {
    let data_byte = iter.next().unwrap();
    let mode = get_mode(&data_byte)?;
    let (source, dest): (RegisterMemory, RegisterMemory);
    let rm = data_byte & 7;
    let byte = (data_byte >> 3) & 7;
    let encoding = RegisterMemoryEncoding {
        mode,
        rm,
        wide,
        iter
    };
    if reversed {
        dest = RegisterMemory::try_from(encoding)?;
        source = RegisterMemory::try_from(RegisterEncoding { byte, wide })?;
    } else {
        source = RegisterMemory::try_from(encoding)?;
        dest = RegisterMemory::try_from(RegisterEncoding { byte, wide })?;
    }
    Ok((dest, source))
}


pub fn jump<'a, I: Iterator<Item = &'a u8>>(
    mut iter: I
) -> Result<u8> {
    Ok(*iter.next().unwrap())
}

pub fn immediate_to_register<'a, I: Iterator<Item = &'a u8>>(
    wide: bool,
    mut iter: I
) -> Result<Instruction> {
    println!("{wide}");
    let data_byte = iter.next().unwrap();
    let mode = get_mode(&data_byte)?;
    let rm = data_byte & 7;
    let byte = (data_byte >> 3) & 7;
    let immediate = ImmediateMode::try_from(byte)?;
    let rm_encoding = RegisterMemoryEncoding {
        mode,
        rm,
        wide,
        iter: &mut iter,
    };
    let dest = RegisterMemory::try_from(rm_encoding)?;
    let im_encoding = ImmediateModeEncoding {
        dest,
        mode: immediate,
        iter,
        wide,
    };
    let instruction = Instruction::try_from(im_encoding)?;
    Ok(instruction)
}

pub fn pop<'a, I: Iterator<Item = &'a u8>>(
    wide: bool,
    mut iter: I
) -> Result<Instruction> {
    Ok(Instruction::NOP)
}

pub fn call<'a, I: Iterator<Item = &'a u8>>(
    mut iter: I
) -> Result<Instruction> {
    Ok(Instruction::CALL { far_proc: 0 })
}
