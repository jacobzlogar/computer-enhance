use crate::parse_twos_complement_int;

const MEMORY_MODE_ENCODING: [RegisterMemory; 8] = [
    RegisterMemory::CombineRegisters(Register::BX, Register::SI),
    RegisterMemory::CombineRegisters(Register::BX, Register::DI),
    RegisterMemory::CombineRegisters(Register::BP, Register::SI),
    RegisterMemory::CombineRegisters(Register::BP, Register::DI),
    RegisterMemory::Register(Register::SI),
    RegisterMemory::Register(Register::DI),
    RegisterMemory::DirectAddress(0),
    RegisterMemory::Register(Register::BX),
];

const MEMORY_MODE_DISPLACEMENT_ENCODING: [RegisterMemory; 8] = [
    RegisterMemory::CombineRegistersData(Register::BX, Register::SI, 0),
    RegisterMemory::CombineRegistersData(Register::BX, Register::DI, 0),
    RegisterMemory::CombineRegistersData(Register::BP, Register::SI, 0),
    RegisterMemory::CombineRegistersData(Register::BP, Register::DI, 0),
    RegisterMemory::RegisterData(Register::SI, 0),
    RegisterMemory::RegisterData(Register::DI, 0),
    RegisterMemory::RegisterData(Register::BP, 0),
    RegisterMemory::RegisterData(Register::BX, 0),
];

const MEMORY_MODE_DISPLACEMENT_WIDE_ENCODING: [RegisterMemory; 8] = [
    RegisterMemory::CombineRegistersDataWide(Register::BX, Register::SI, 0),
    RegisterMemory::CombineRegistersDataWide(Register::BX, Register::DI, 0),
    RegisterMemory::CombineRegistersDataWide(Register::BP, Register::SI, 0),
    RegisterMemory::CombineRegistersDataWide(Register::BP, Register::DI, 0),
    RegisterMemory::RegisterDataWide(Register::SI, 0),
    RegisterMemory::RegisterDataWide(Register::DI, 0),
    RegisterMemory::RegisterDataWide(Register::BP, 0),
    RegisterMemory::RegisterDataWide(Register::BX, 0),
];

const REGISTERS: [Register; 8] = [
    Register::AL,
    Register::CL,
    Register::DL,
    Register::BL,
    Register::AH,
    Register::CH,
    Register::DH,
    Register::BH,
];

const WIDE_REGISTERS: [Register; 8] = [
    Register::AX,
    Register::CX,
    Register::DX,
    Register::BX,
    Register::SP,
    Register::BP,
    Register::SI,
    Register::DI,
];

#[derive(Debug)]
pub enum Mode {
    MemoryMode,
    MemoryModeDisplacement,
    MemoryModeDisplacementWide,
    RegisterMode,
}

impl TryFrom<u8> for Mode {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::MemoryMode),
            1 => Ok(Mode::MemoryModeDisplacement),
            2 => Ok(Mode::MemoryModeDisplacementWide),
            3 => Ok(Mode::RegisterMode),
            _ => Err("Not a memory mode"),
        }
    }
}

#[derive(Debug)]
pub struct RegisterMemoryEncoding<I> {
    pub rm: u8,
    pub mode: Mode,
    pub wide: bool,
    pub iter: I,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RegisterMemory {
    SegmentRegister(SegmentRegister),
    Register(Register),
    CombineRegisters(Register, Register),
    DirectAddress(isize),
    RegisterData(Register, isize),
    RegisterDataWide(Register, isize),
    CombineRegistersData(Register, Register, isize),
    CombineRegistersDataWide(Register, Register, isize),
    Immediate(isize),
}

impl<'a, I: Iterator<Item = &'a u8>> TryFrom<RegisterMemoryEncoding<I>> for RegisterMemory {
    type Error = &'static str;
    fn try_from(mut value: RegisterMemoryEncoding<I>) -> Result<Self, Self::Error> {
        match value.mode {
            Mode::MemoryMode => {
                let memory_mode = MEMORY_MODE_ENCODING[value.rm as usize];
                match memory_mode {
                    Self::DirectAddress(_) => {
                        let mut operand: isize;
                        if value.wide {
                            operand = u16::from_le_bytes([
                                *value.iter.next().unwrap(),
                                *value.iter.next().unwrap(),
                            ]) as isize;
                        } else {
                            operand = *value.iter.next().unwrap() as isize;
                        }
                        operand = parse_twos_complement_int(operand, value.wide);
                        Ok(Self::DirectAddress(operand))
                    }
                    _ => Ok(memory_mode),
                }
            }
            Mode::MemoryModeDisplacement => {
                let displacement =
                    parse_twos_complement_int(*value.iter.next().unwrap() as isize, false);
                let register_memory = MEMORY_MODE_DISPLACEMENT_ENCODING[value.rm as usize];
                match register_memory {
                    Self::RegisterData(dest, _) => {
                        return Ok(Self::RegisterData(dest, displacement));
                    }
                    Self::CombineRegistersData(dest, source, _) => {
                        return Ok(Self::CombineRegistersData(dest, source, displacement));
                    }
                    _ => Err("Failed to parse displacement"),
                }
            }
            Mode::MemoryModeDisplacementWide => {
                let data =
                    u16::from_le_bytes([*value.iter.next().unwrap(), *value.iter.next().unwrap()]);
                let displacement = parse_twos_complement_int(data as isize, true);
                let register_memory = MEMORY_MODE_DISPLACEMENT_WIDE_ENCODING[value.rm as usize];
                match register_memory {
                    Self::RegisterDataWide(dest, _) => {
                        return Ok(Self::RegisterDataWide(dest, displacement));
                    }
                    Self::CombineRegistersDataWide(dest, source, _) => {
                        return Ok(Self::CombineRegistersDataWide(dest, source, displacement));
                    }
                    _ => Err("Failed to parse wide displacement"),
                }
            }
            Mode::RegisterMode => {
                if value.wide {
                    return Ok(RegisterMemory::Register(WIDE_REGISTERS[value.rm as usize]));
                } else {
                    return Ok(RegisterMemory::Register(REGISTERS[value.rm as usize]));
                }
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum SegmentRegister {
    ES,
    CS,
    SS,
    DS,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Register {
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

pub struct RegisterEncoding {
    pub byte: u8,
    pub wide: bool,
}

impl TryFrom<RegisterEncoding> for RegisterMemory {
    type Error = &'static str;
    fn try_from(value: RegisterEncoding) -> Result<Self, Self::Error> {
        if value.wide {
            match value.byte {
                0x00 => Ok(Self::Register(Register::AX)),
                0x01 => Ok(Self::Register(Register::CX)),
                0x02 => Ok(Self::Register(Register::DX)),
                0x03 => Ok(Self::Register(Register::BX)),
                0x04 => Ok(Self::Register(Register::SP)),
                0x05 => Ok(Self::Register(Register::BP)),
                0x06 => Ok(Self::Register(Register::SI)),
                0x07 => Ok(Self::Register(Register::DI)),
                _ => Err("Not a 16-bit register"),
            }
        } else {
            match value.byte {
                0x00 => Ok(Self::Register(Register::AL)),
                0x01 => Ok(Self::Register(Register::CL)),
                0x02 => Ok(Self::Register(Register::DL)),
                0x03 => Ok(Self::Register(Register::BL)),
                0x04 => Ok(Self::Register(Register::AH)),
                0x05 => Ok(Self::Register(Register::CH)),
                0x06 => Ok(Self::Register(Register::DH)),
                0x07 => Ok(Self::Register(Register::BH)),
                _ => Err("Not an 8-bit register"),
            }
        }
    }
}
