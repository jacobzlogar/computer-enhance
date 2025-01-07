pub mod instructions;
pub mod opcodes;
pub mod registers;
pub mod cpu;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn parse_twos_complement_int(int: isize, is_16_bit: bool) -> isize {
    if is_16_bit {
        let mut val = int as i16;
        if int >> 15 == 1 {
            val = !val + 1;
            val = -val;
        }
        return val as isize;
    } else {
        let mut val = int as i8;
        if int >> 7 == 1 {
            val = !val + 1;
            val = -val;
        }
        return val as isize;
    }
}
