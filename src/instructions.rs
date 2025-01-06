use crate::{
    parse_twos_complement_int, registers::{
        Mode, Register, RegisterEncoding, RegisterMemory, RegisterMemoryEncoding, SegmentRegister,
    }, Result
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
pub enum Mnemonic {
    XLAT,
    ESC,
    LOOPNE {
        short_label: isize
    },
    LOOPE {
        short_label: isize
    },
    LOOP {
        short_label: isize,
    },
    IN {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    OUT {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    INTO,
    IRET,
    AAM,
    AAD,
    SAR {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    SHR {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    SAL {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    RCR {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    RCL {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    ROR {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    ROL {
        dest: RegisterMemory,
        source: RegisterMemory
    },
    INT {
        value: isize,
    },
    LDS {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    LES {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    RET {
        segment: Option<isize>,
    },
    MOVS {
        wide: bool,
    },
    CMPS {
        wide: bool,
    },
    STOS {
        wide: bool,
    },
    LODS {
        wide: bool,
    },
    SCAS {
        wide: bool,
    },
    WAIT,
    PUSHF,
    POPF,
    SAHF,
    LAHF,
    CWD,
    CALL {
        far_proc: usize,
    },
    LEA {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    MOV {
        dest: RegisterMemory,
        source: RegisterMemory,
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
    JMP {
        label: isize
    },
    LOCK,
    REPNE,
    REP,
    HLT,
    CMC,
    JCXZ {
        label: u8,
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
    NOT {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    NEG {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    MUL {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    IMUL {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    DIV {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    IDIV {
        dest: RegisterMemory,
        source: RegisterMemory,
    },
    CLC,
    STC,
    CLI,
    CLD,
    STD
}

pub struct ImmediateModeEncoding<I> {
    mode: ImmediateMode,
    dest: RegisterMemory,
    wide: bool,
    iter: I,
}

impl<'a, I: Iterator<Item = &'a u8>> TryFrom<ImmediateModeEncoding<I>> for Mnemonic {
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
            ImmediateMode::ADD => Ok(Mnemonic::ADD {
                dest: value.dest,
                source,
            }),
            ImmediateMode::OR => Ok(Mnemonic::OR {
                dest: value.dest,
                source,
            }),
            ImmediateMode::ADC => Ok(Mnemonic::ADC {
                dest: value.dest,
                source,
            }),
            ImmediateMode::SBB => Ok(Mnemonic::SBB {
                dest: value.dest,
                source,
            }),
            ImmediateMode::AND => Ok(Mnemonic::AND {
                dest: value.dest,
                source,
            }),
            ImmediateMode::SUB => Ok(Mnemonic::SUB {
                dest: value.dest,
                source,
            }),
            ImmediateMode::XOR => Ok(Mnemonic::XOR {
                dest: value.dest,
                source,
            }),
            ImmediateMode::CMP => Ok(Mnemonic::CMP {
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
        iter,
    };
    if reversed {
        source = RegisterMemory::try_from(encoding)?;
        dest = RegisterMemory::try_from(RegisterEncoding { byte, wide })?;
    } else {
        dest = RegisterMemory::try_from(encoding)?;
        source = RegisterMemory::try_from(RegisterEncoding { byte, wide })?;
    }
    Ok((dest, source))
}

pub fn jump<'a, I: Iterator<Item = &'a u8>>(mut iter: I) -> Result<u8> {
    Ok(*iter.next().unwrap())
}

pub fn immediate_to_memory<'a, I: Iterator<Item = &'a u8>>(
    wide: bool,
    mut iter: I
) -> Result<Mnemonic> {
    let data_byte = iter.next().unwrap();
    let rm = data_byte & 7;
    let mode = get_mode(&data_byte)?;
    let rm_encoding = RegisterMemoryEncoding {
        mode,
        rm,
        wide,
        iter: &mut iter,
    };
    let dest = RegisterMemory::try_from(rm_encoding)?;
    let mut operand: isize;
    if wide {
        operand =
            u16::from_le_bytes([*iter.next().unwrap(), *iter.next().unwrap()]) as isize;
    } else {
        operand = *iter.next().unwrap() as isize;
    }
    operand = parse_twos_complement_int(operand, wide);
    Ok(Mnemonic::MOV { dest, source: RegisterMemory::Immediate(operand as isize) })
}

pub fn immediate_to_register<'a, I: Iterator<Item = &'a u8>>(
    register_wide: bool,
    immediate_wide: bool,
    mut iter: I,
) -> Result<Mnemonic> {
    let data_byte = iter.next().unwrap();
    let mode = get_mode(&data_byte)?;
    let rm = data_byte & 7;
    let byte = (data_byte >> 3) & 7;
    let immediate = ImmediateMode::try_from(byte)?;
    let rm_encoding = RegisterMemoryEncoding {
        mode,
        rm,
        wide: register_wide,
        iter: &mut iter,
    };
    let dest = RegisterMemory::try_from(rm_encoding)?;
    let im_encoding = ImmediateModeEncoding {
        dest,
        mode: immediate,
        iter,
        wide: immediate_wide,
    };
    let instruction = Mnemonic::try_from(im_encoding)?;
    Ok(instruction)
}

pub fn pop<'a, I: Iterator<Item = &'a u8>>(wide: bool, mut iter: I) -> Result<Mnemonic> {
    Ok(Mnemonic::NOP)
}

pub fn call<'a, I: Iterator<Item = &'a u8>>(mut iter: I) -> Result<Mnemonic> {
    Ok(Mnemonic::CALL { far_proc: 0 })
}
