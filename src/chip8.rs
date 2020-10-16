pub struct Chip8 {
    opcode: u16, // current opcode
    memory: [u8; 4096], // 4K memory
    v: [u8; 16], // V0-VE registers
    i: u16, // index register
    pc: u16, // program counter
    gfx: [u8; 64 * 32], // graphics
    delay_timer: u8, // counter register at 60Hz, counts down to 0
    sound_timer: u8, // counter plays sound at 0, counts down to 0
    stack: [u16; 16], // opcode stack
    sp: u16, // stack pointer
    key: [u8; 16], // hex keypad to store key state
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0, 
            pc: 0,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        }
    }

    pub fn initialize(&mut self) {
        // initialize:
        // - registers
        // - memory
       for (i, font_byte) in FONTS.iter().enumerate() {
          self.memory[i] = *font_byte;
       }  
    }

    pub fn load_game(&self) {
        // read game data into memory
    }

    pub fn emulate_cycle(&self) {
        // fetch opcode
        // decode opcode
        // execute opcode

        // update timers
    }

    pub fn should_draw(&self) -> bool {
        true
    }

    pub fn set_keys(&self) {
        // store key press state
    }

    pub fn should_quit(&self) -> bool {
        true
    }
}

const FONTS: [u8; 80] = [
    0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000, // "0"
    0b00100000, 0b01100000, 0b00100000, 0b00100000, 0b01110000, // "1"
    0b11110000, 0b00010000, 0b11110000, 0b10000000, 0b11110000, // "2"
    0b11110000, 0b00010000, 0b11110000, 0b00010000, 0b11110000, // "3"
    0b10010000, 0b10010000, 0b11110000, 0b00010000, 0b00010000, // "4"
    0b11110000, 0b10000000, 0b11110000, 0b00010000, 0b11110000, // "5"
    0b11110000, 0b10000000, 0b11110000, 0b10010000, 0b11110000, // "6"
    0b11110000, 0b00010000, 0b00100000, 0b01000000, 0b01000000, // "7"
    0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000, // "8"
    0b11110000, 0b10010000, 0b11110000, 0b00010000, 0b11110000, // "9"
    0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b10010000, // "A"
    0b11100000, 0b10010000, 0b11100000, 0b10010000, 0b11100000, // "B"
    0b11110000, 0b10000000, 0b10000000, 0b10000000, 0b11110000, // "C"
    0b11100000, 0b10010000, 0b10010000, 0b10010000, 0b11100000, // "D"
    0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b11110000, // "E"
    0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b10000000, // "F"
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_fonts_into_memory() {
        let mut chip8 = Chip8::new();
        chip8.initialize();
        assert_eq!(chip8.memory[..80].iter().eq(FONTS.iter()), true);
    }
}