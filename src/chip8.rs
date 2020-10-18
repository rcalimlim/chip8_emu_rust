use std::fs::File;
use std::io::Read;
use crate::instructions::*;

pub struct Chip8 {
    pub opcode: u16,        // current opcode
    pub memory: [u8; 4096], // 4K memory
    pub v: [u8; 16],        // V0-VE registers
    pub i: u16,             // index register
    pub pc: u16,            // program counter
    pub gfx: [u8; 64 * 32], // graphics
    pub stack: [u16; 16],   // opcode stack
    pub sp: u16,            // stack pointer
    pub key: [u8; 16],      // hex keypad to store key state
    pub delay_timer: u8,    // counter register at 60Hz, counts down to 0
    pub sound_timer: u8,    // counter plays sound at 0, counts down to 0
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
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

    pub fn load_rom(&mut self, rom_path: &str) {
        // read game data into memory
        let rom = match File::open(rom_path) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file: {:?}", e),
        };

        for (i, byte) in rom.bytes().enumerate() {
            self.memory[0x200 + i] = byte.unwrap();
        }
    }

    pub fn emulate_cycle(&mut self) {
        // fetch opcode
        let counter: usize = self.pc.into();
        let high_byte = self.memory[counter];
        let low_byte = self.memory[counter + 1];
        self.opcode = (high_byte as u16) << 8 | low_byte as u16;

        // decode opcode
        let opcode_nibbles = [
            self.opcode >> 12,
            self.opcode >> 8 & 0xF,
            self.opcode >> 4 & 0xF,
            self.opcode & 0xF,
        ];
        match opcode_nibbles {
            [0x1, _, _, _] => jp_addr(self),
            _ => panic!("Not a valid opcode: {:?}", self.opcode),
        }

        // update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            println!("*Beep*");
            self.sound_timer -= 1;
        }
    }

    pub fn should_draw(&self) -> bool {
        true
    }

    pub fn set_keys(&self) {
        // store key press state
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
    use std::io::Read;

    #[test]
    fn read_fonts_into_memory() {
        let mut chip8 = Chip8::new();
        chip8.initialize();
        assert_eq!(chip8.memory[..80].iter().eq(FONTS.iter()), true);
    }

    #[test]
    fn load_rom_into_memory() {
        let mut chip8 = Chip8::new();
        chip8.initialize();
        chip8.load_rom("test/test-rom.ch8");

        let mut data: Vec<u8> = Vec::new();
        let mut rom = match File::open("test/test-rom.ch8") {
            Ok(file) => file,
            Err(e) => panic!("Could not open test rom: {:?}", e),
        };
        rom.read_to_end(&mut data).unwrap();
        assert_eq!(chip8.memory[0x200..(0x200 + data.len())].to_vec(), data);
    }
}
