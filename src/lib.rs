// Opcode variable struct
// - nibbles - All 4-bit values of the 16-bit word
// - nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// - x - A 4-bit value, the lower 4 bits of the high byte of the instruction
// - y - A 4-bit value, the upper 4 bits of the low byte of the instruction
// - kk or byte - An 8-bit value, the lowest 8 bits of the instruction
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

// Helper function that translates opcodes to instrution variables
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

#[cfg(tests)]
mod test {
    use super::*;

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
}
