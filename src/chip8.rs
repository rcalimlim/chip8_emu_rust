extern crate rand;

use crate::instructions::*;
use crate::utils::*;
use std::fs::File;
use std::io::Read;

pub struct Chip8 {
    pub opcode: u16,        // current opcode
    pub memory: [u8; 4096], // 4K memory
    pub v: [u8; 16],        // V0-VE registers
    pub i: u16,             // index register
    pub pc: u16,            // program counter
    pub gfx: [u8; 64 * 32], // graphics
    pub stack: [u16; 16],   // opcode stack
    pub sp: u16,            // stack pointer
    pub key: [bool; 16],    // hex keypad to store key state
    pub delay_timer: u8,    // counter register at 60Hz, counts down to 0
    pub sound_timer: u8,    // counter plays sound at 0, counts down to 0
    pub should_draw: bool,  // draw flag
}

impl Chip8 {
    pub fn initialize() -> Self {
        let mut chip8 = Chip8 {
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
            key: [false; 16],
            should_draw: false,
        };

        for (i, font_byte) in FONTS.iter().enumerate() {
            chip8.memory[i] = *font_byte;
        }

        chip8
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
        let opcode_nibbles = opcode_to_variables(&self.opcode).nibbles;

        // closure that panics on invalid opcode
        let panic_invalid_opcode = || {
            panic!("Not a valid opcode: {:#X?}", self.opcode);
        };

        println!("{:#X?}", self.opcode);

        // match opcodes to instructions
        match opcode_nibbles {
            [0x0, 0x0, 0xE, 0x0] => cls(self),                // 0x00E0
            [0x0, 0x0, 0xE, 0xE] => ret(self),                // 0x00EE
            [0x0, _, _, _] => sys_addr(self),                 // 0x0nnn
            [0x1, _, _, _] => jp_addr(self),                  // 0x1nnn
            [0x2, _, _, _] => call_addr(self),                // 0x2nnn
            [0x3, _, _, _] => se_vx_byte(self),               // 0x3xkk
            [0x4, _, _, _] => sne_vx_byte(self),              // 0x4xkk
            [0x5, _, _, 0x0] => se_vx_vy(self),               // 0x5xy0
            [0x6, _, _, _] => ld_vx_byte(self),               // 0x6xkk
            [0x7, _, _, _] => add_vx_byte(self),              // 0x7xkk
            [0x8, _, _, 0] => ld_vx_vy(self),                 // 0x8xy0
            [0x8, _, _, 1] => or_vx_vy(self),                 // 0x8xy1
            [0x8, _, _, 2] => and_vx_vy(self),                // 0x8xy2
            [0x8, _, _, 3] => xor_vx_vy(self),                // 0x8xy3
            [0x8, _, _, 4] => add_vx_vy(self),                // 0x8xy4
            [0x8, _, _, 5] => sub_vx_vy(self),                // 0x8xy5
            [0x8, _, _, 6] => shr_vx_vy(self),                // 0x8xy6
            [0x8, _, _, 7] => subn_vx_vy(self),               // 0x8xy7
            [0x8, _, _, 0xE] => shl_vx_vy(self),              // 0x8xyE
            [0x9, _, _, 0x0] => sne_vx_vy(self),              // 0x9xy0
            [0xA, _, _, _] => ld_i_addr(self),                // 0xAnnn
            [0xB, _, _, _] => jp_v0_addr(self),               // 0xBnnn
            [0xC, _, _, _] => rnd_vx_byte(self, gen_rand_u8), // 0xCnnn
            [0xD, _, _, _] => drw_vx_vy_nibble(self),         // 0xDxyn
            [0xE, _, 0x9, 0xE] => skp_vx(self),               // 0xEx9E
            [0xE, _, 0xA, 0x1] => sknp_vx(self),              // 0xExA1
            [0xF, _, 0x0, 0x7] => ld_vx_dt(self),             // 0xFx07
            [0xF, _, 0x0, 0xA] => ld_vx_k(self),              // 0xFx0A
            [0xF, _, 0x1, 0x5] => ld_dt_vx(self),             // Fx15
            [0xF, _, 0x1, 0x8] => ld_st_vx(self),             // Fx18
            [0xF, _, 0x1, 0xE] => add_i_vx(self),             // Fx1E
            [0xF, _, 0x2, 0x9] => ld_f_vx(self),              // Fx29
            [0xF, _, 0x3, 0x3] => ld_b_vx(self),              // Fx33
            [0xF, _, 0x5, 0x5] => ld_i_vx(self),              // Fx55
            [0xF, _, 0x6, 0x5] => ld_vx_i(self),              // Fx65
            _ => panic_invalid_opcode(),
        }

        // update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        // 'beep' if sound timer is greater than 0
        if self.sound_timer > 0 {
            println!("*Beep*");
            self.sound_timer -= 1;
        }
    }

    pub fn should_draw(&self) -> bool {
        true
    }

    pub fn set_key(&self, key: usize, value: bool) {
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
        let mut chip8 = Chip8::initialize();
        assert_eq!(chip8.memory[..80].iter().eq(FONTS.iter()), true);
    }

    #[test]
    fn load_rom_into_memory() {
        let mut chip8 = Chip8::initialize();
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
