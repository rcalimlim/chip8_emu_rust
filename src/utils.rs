use num::traits::Unsigned;
use rand::{thread_rng, Rng};
use std::convert::TryInto;
use std::ops::{BitAnd, ShrAssign};

/// Opcode variable struct
/// - nibbles - All 4-bit values of the 16-bit word
/// - nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
/// - x - A 4-bit value, the lower 4 bits of the high byte of the instruction
/// - y - A 4-bit value, the upper 4 bits of the low byte of the instruction
/// - kk or byte - An 8-bit value, the lowest 8 bits of the instruction
pub struct OpcodeVariables {
    pub nibbles: [usize; 4],
    pub nnn: u16,
    pub x: usize,
    pub y: usize,
    pub kk: u8,
}

impl OpcodeVariables {
    fn new(nibbles: [usize; 4], nnn: u16, x: usize, y: usize, kk: u8) -> Self {
        OpcodeVariables {
            nibbles,
            nnn,
            x,
            y,
            kk,
        }
    }
}

/// Helper function that translates opcodes to instruction variables
pub fn opcode_to_variables(word: &u16) -> OpcodeVariables {
    let nibbles = [
        ((word & 0xF000) >> 12) as usize,
        ((word & 0x0F00) >> 8) as usize,
        ((word & 0x00F0) >> 4) as usize,
        (word & 0x000F) as usize,
    ];

    OpcodeVariables::new(
        nibbles,
        word & 0x0FFF,
        nibbles[1],
        nibbles[2],
        (word & 0x00FF) as u8,
    )
}

/// Generate a random integer within [0, 256)
pub fn gen_rand_u8() -> u8 {
    let result: u8 = thread_rng().gen();
    result
}

/// Translate unsigned int into bit array
pub fn into_bit_vec<T>(num: T) -> Vec<u8>
where
    T: Copy + PartialOrd + BitAnd + ShrAssign + Unsigned,
    <T as BitAnd>::Output: TryInto<u8>,
{
    let mut num = num;
    let mut bits = Vec::new();
    while num > T::zero() {
        let bit: u8 = match (num & T::one()).try_into() {
            Ok(val) => val,
            Err(_) => panic!("Failed to cast input."),
        };
        bits.push(bit);
        num >>= T::one();
    }
    bits.reverse();
    bits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opcode_to_variables() {
        let word = 0xABCD;
        let vars = opcode_to_variables(&word);

        assert_eq!(
            [0xA, 0xB, 0xC, 0xD],
            vars.nibbles,
            "`nibbles` should be correct"
        );
        assert_eq!(0xBCD, vars.nnn, "`nnn` (addr) should be correct");
        assert_eq!(0xB, vars.x, "`x` should be correct");
        assert_eq!(0xC, vars.y, "`y` should be correct");
        assert_eq!(0xCD, vars.kk, "`kk` (byte) should be correct");
    }

    #[test]
    fn test_into_bit_vec_u8() {
        let val: u8 = 0b10100011;
        let expected: Vec<u8> = vec![1, 0, 1, 0, 0, 0, 1, 1];
        assert_eq!(expected, into_bit_vec(val));
    }

    #[test]
    fn test_into_bit_vec_u16() {
        let val: u16 = 0b1010001110101010;
        let expected: Vec<u8> = vec![1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0];
        assert_eq!(expected, into_bit_vec(val));
    }

    #[test]
    fn test_into_bit_vec_u32() {
        let val: u32 = 0b10100011101010101010001110101010;
        let expected: Vec<u8> = vec![
            1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1,
            0, 1, 0,
        ];
        assert_eq!(expected, into_bit_vec(val));
    }

    #[test]
    fn test_into_bit_vec_u64() {
        let val: u64 = 0b1010001110101010101000111010101010100011101010101010001110101010;
        let expected: Vec<u8> = vec![
            1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1,
            0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0,
            1, 0, 1, 0, 1, 0,
        ];
        assert_eq!(expected, into_bit_vec(val));
    }

    #[test]
    fn test_into_bit_vec_u128() {
        let val: u128 = 0b10100011101010101010001110101010101000111010101010100011101010101010001110101010101000111010101010100011101010101010001110101010;
        let expected: Vec<u8> = vec![
            1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1,
            0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1,
            1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0,
        ];
        assert_eq!(expected, into_bit_vec(val));
    }

    #[test]
    fn test_into_bit_vec_usize() {
        let val: usize = 0b10100011101010101010001110101010;
        let expected: Vec<u8> = vec![
            1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1,
            0, 1, 0,
        ];
        assert_eq!(expected, into_bit_vec(val));
    }
}
