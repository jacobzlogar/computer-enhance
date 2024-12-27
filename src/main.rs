use std::{env, error::Error, fmt::Display, slice::Iter, iter::Peekable};
use computer_enhance::parse_twos_complement_int;

type ChunkIter<'a> = &'a mut Iter<'a, u8>;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ArithOperation {
    Add = 0b000,
    Sub = 0b101,
    Cmp = 0b111
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MovMnemonic {
    ImmediateToRegisterMemory,
    RegisterMemoryToRegister,
    MemoryToAccumulator,
    AccumulatorToMemory,
    RegisterMemoryToSegmentRegister,
    SegmentRegisterToRegisterMemory,
    ImmediateToRegister,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ArithMnemonic {
    ImmediateToRegisterMemory,
}
#[derive(Debug)]
enum Opcode {
    Mov(MovMnemonic),
    Arithmetic(ArithMnemonic)
}
#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    mask: u8,
    bit_shift: u8,
}
const OPCODE_TABLE: [Instruction; 8] = [
    Instruction {
        opcode: Opcode::Arithmetic(ArithMnemonic::ImmediateToRegisterMemory),
        mask: 0b100000,
        bit_shift: 2,
    },
    Instruction {
        opcode: Opcode::Mov(MovMnemonic::RegisterMemoryToRegister),
        mask: 0b100010,
        bit_shift: 2,
    },
    Instruction {
        opcode: Opcode::Mov(MovMnemonic::ImmediateToRegisterMemory),
        mask: 0b1100011,
        bit_shift: 1,
    },
    Instruction {
        opcode: Opcode::Mov(MovMnemonic::ImmediateToRegister),
        mask: 0b1011,
        bit_shift: 4,
    },
    Instruction {
        opcode: Opcode::Mov(MovMnemonic::MemoryToAccumulator),
        mask: 0b1010000,
        bit_shift: 1,
    },
    Instruction {
        opcode: Opcode::Mov(MovMnemonic::AccumulatorToMemory),
        mask: 0b1010001,
        bit_shift: 1,
    },
    Instruction {
        opcode: Opcode::Mov(MovMnemonic::RegisterMemoryToSegmentRegister),
        mask: 0b10001110,
        bit_shift: 0,
    },
    Instruction {
        opcode: Opcode::Mov(MovMnemonic::SegmentRegisterToRegisterMemory),
        mask: 0b10001100,
        bit_shift: 0,
    },
];

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RegisterTarget {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

impl TryFrom<(bool, u8)> for RegisterTarget {
    type Error = &'static str;
    fn try_from(value: (bool, u8)) -> Result<Self, Self::Error> {
        if value.0 {
            match value.1 {
                0x00 => Ok(Self::AX),
                0x01 => Ok(Self::CX),
                0x02 => Ok(Self::DX),
                0x03 => Ok(Self::BX),
                0x04 => Ok(Self::SP),
                0x05 => Ok(Self::BP),
                0x06 => Ok(Self::SI),
                0x07 => Ok(Self::DI),
                _ => Err("Not a 16-bit register".into()),
            }
        } else {
            match value.1 {
                0x00 => Ok(Self::AL),
                0x01 => Ok(Self::CL),
                0x02 => Ok(Self::DL),
                0x03 => Ok(Self::BL),
                0x04 => Ok(Self::AH),
                0x05 => Ok(Self::CH),
                0x06 => Ok(Self::DH),
                0x07 => Ok(Self::BH),
                _ => Err("Not an 8-bit register".into()),
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Mode {
    MemoryMode,
    MemoryMode8Bit,
    MemoryMode16Bit,
    RegisterMode,
}

impl TryFrom<u8> for Mode {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::MemoryMode),
            0x01 => Ok(Self::MemoryMode8Bit),
            0x02 => Ok(Self::MemoryMode16Bit),
            0x03 => Ok(Self::RegisterMode),
            _ => Err("Not a memory mode"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum MovTarget {
    RegisterToRegister(RegisterTarget),
    ImmediateToRegister(u8),
    SourceAddressCalculation {
        first: RegisterTarget,
        second: Option<RegisterTarget>,
    },
    SourceAddressCalculation8Bit {
        first: RegisterTarget,
        second: Option<RegisterTarget>,
        displacement: Option<u8>,
    },
    SourceAddressCalculation16Bit {
        first: RegisterTarget,
        second: Option<RegisterTarget>,
        displacement: Option<u16>,
    },
}

fn source_address_display<T: std::fmt::Debug>(
    first: &RegisterTarget,
    second: &Option<RegisterTarget>,
    displacement: &Option<T>,
) -> String {
    match (second, displacement) {
        (Some(second), Some(displacement)) => {
            return format!("[{:?} + {:?} + {:?}]", first, second, displacement);
        }
        (Some(second), None) => {
            return format!("[{:?} + {:?}]", first, second);
        }
        (None, None) => {
            return format!("[{:?}]", first);
        }
        (None, Some(disp)) => {
            return format!("[{:?} + {:?}]", first, disp);
        }
    }
}

impl Display for MovTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::RegisterToRegister(reg) => {
                return write!(f, "{:?}", reg);
            }
            MovTarget::ImmediateToRegister(val) => {
                return write!(f, "{val}");
            }
            MovTarget::SourceAddressCalculation { first, second } => {
                if let Some(second) = second {
                    return write!(f, "[{:?} + {:?}]", first, second);
                }
                return write!(f, "[{:?}]", first);
            }
            MovTarget::SourceAddressCalculation8Bit {
                first,
                second,
                displacement,
            } => {
                return write!(
                    f,
                    "{}",
                    source_address_display::<u8>(first, second, displacement)
                );
            }
            MovTarget::SourceAddressCalculation16Bit {
                first,
                second,
                displacement,
            } => {
                return write!(
                    f,
                    "{}",
                    source_address_display::<u16>(first, second, displacement)
                );
            }
        }
    }
}

impl<'a> TryFrom<(u8, Mode, ChunkIter<'a>)> for MovTarget {
    type Error = &'static str;
    fn try_from(value: (u8, Mode, ChunkIter<'a>)) -> Result<Self, Self::Error> {
        let register_map = [
            (RegisterTarget::BX, Some(RegisterTarget::SI)),
            (RegisterTarget::BX, Some(RegisterTarget::DI)),
            (RegisterTarget::BP, Some(RegisterTarget::SI)),
            (RegisterTarget::BP, Some(RegisterTarget::DI)),
            (RegisterTarget::SI, None),
            (RegisterTarget::DI, None),
            (RegisterTarget::BP, None),
            (RegisterTarget::BX, None),
        ];

        let (first, second) = register_map
            .get(value.0 as usize)
            .ok_or("Not a valid register/memory field encoding")?;

        let result = match value.1 {
            Mode::RegisterMode => MovTarget::RegisterToRegister(*first),
            Mode::MemoryMode => MovTarget::SourceAddressCalculation {
                first: *first,
                second: *second,
            },
            Mode::MemoryMode8Bit => {
                let data = value.2.next().unwrap();
                MovTarget::SourceAddressCalculation8Bit {
                    first: *first,
                    second: *second,
                    displacement: Some(*data),
                }
            }
            Mode::MemoryMode16Bit => {
                let data = u16::from_le_bytes([*value.2.next().unwrap(), *value.2.next().unwrap()]);
                MovTarget::SourceAddressCalculation16Bit {
                    first: *first,
                    second: *second,
                    displacement: Some(data),
                }
            }
        };

        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Mov {
    opcode_byte: u8,
    wide: bool,
    mnemonic: MovMnemonic,
    value: isize,
    source: Option<MovTarget>,
    dest: Option<MovTarget>,
}

impl Display for Mov {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.source, &self.dest) {
            (Some(src), Some(dest)) => {
                return write!(f, "mov {}, {}", src, dest);
            }
            (None, None) => {
                panic!("this should be impossible")
            }
            (None, Some(dest)) => {
                return write!(f, "mov {}, {}", dest, self.value);
            }
            (Some(src), None) => {
                return write!(f, "mov {}, {}", src, self.value);
            }
        }
    }
}

impl Mov {
    fn new(mnemonic: MovMnemonic, opcode_byte: u8) -> Self {
        Self {
            opcode_byte,
            mnemonic,
            wide: false,
            value: 0,
            source: None,
            dest: None,
        }
    }
    fn handle_immediate_to_register<'a>(
        &mut self,
        chunk: ChunkIter<'a>
    ) -> Result<&mut Self, Box<dyn Error>> {
        self.wide = (self.opcode_byte >> 3) & 1 != 0;
        if self.wide {
            let data = u16::from_le_bytes([*chunk.next().unwrap(), *chunk.next().unwrap()]);
            self.value = parse_twos_complement_int(data as isize, true);
        } else {
            let data = chunk.next().unwrap();
            self.value = parse_twos_complement_int(*data as isize, false);
        }
        let target = RegisterTarget::try_from((self.wide, self.opcode_byte & 7))?;
        self.source = Some(MovTarget::RegisterToRegister(target));
        Ok(self)
    }
    fn handle_memory_register_to_register<'a>(
        &mut self,
        chunk: ChunkIter<'a>
    ) -> Result<&mut Self, Box<dyn Error>> {
        self.wide = self.opcode_byte & 1 == 1;
        let reversed = self.opcode_byte & 2 == 2;
        let data_byte = chunk.next().unwrap();
        let mode = Mode::try_from((data_byte) >> 6 & 3)?;
        let reg_bits = (data_byte >> 3) & 7;
        let rm_bits = data_byte & 7;
        match mode {
            Mode::RegisterMode => {
                let source = RegisterTarget::try_from((self.wide, reg_bits))?;
                let dest = RegisterTarget::try_from((self.wide, rm_bits))?;
                match reversed {
                    true => {
                        self.source = Some(MovTarget::RegisterToRegister(source));
                        self.dest = Some(MovTarget::RegisterToRegister(dest));
                    },
                    false => {
                        self.source = Some(MovTarget::RegisterToRegister(dest));
                        self.dest = Some(MovTarget::RegisterToRegister(source));
                    }
                }
            },
            _ => {
                let source = RegisterTarget::try_from((self.wide, reg_bits))?;
                let dest = MovTarget::try_from((rm_bits, mode, chunk))?;
                match reversed {
                    true => {
                        self.source = Some(MovTarget::RegisterToRegister(source));
                        self.dest = Some(dest);
                    },
                    false => {
                        self.source = Some(dest);
                        self.dest = Some(MovTarget::RegisterToRegister(source));
                    }
                }
            },
        }
        Ok(self)
    }
    fn parse<'a>(&mut self, chunk: ChunkIter<'a>) -> Result<&mut Self, Box<dyn Error>> {
        match self.mnemonic {
            MovMnemonic::ImmediateToRegister => self.handle_immediate_to_register(chunk),
            MovMnemonic::ImmediateToRegisterMemory => self.handle_immediate_to_register(chunk),
            MovMnemonic::RegisterMemoryToRegister => self.handle_memory_register_to_register(chunk),
            _ => Err("Unsupported variant".into()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let binary = std::fs::read(&args[1])?;
    let mut iter = binary.iter();
    while let Some(byte) = iter.next() {
        for opcode in OPCODE_TABLE {
            if (*byte >> opcode.bit_shift) == opcode.mask {
                match opcode.opcode {
                    Opcode::Mov(mnemonic) => {
                        let mut op = Mov::new(mnemonic.clone(), byte.clone());
                        op.parse(&mut iter.clone())?;
                        println!("{op}");
                    },
                    Opcode::Arithmetic(mnemonic) => {
                        println!("{:08b}", byte);
                        let next = iter.next().unwrap();
                        println!("{:08b}", next);
                    }
                }
            }
        }
    }
    Ok(())
}
