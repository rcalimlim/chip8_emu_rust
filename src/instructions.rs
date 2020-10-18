use crate::chip8::Chip8;

/// 0nnn - Jump to a machine code routine at nnn.
pub fn sys_addr(chip8: &mut Chip8) {}

/// 00E0 - Clear the display.
pub fn cls(chip8: &mut Chip8) {}

/// 00EE - Return from a subroutine.
pub fn ret(chip8: &mut Chip8) {}

/// 1nnn - Jump to location nnn.
pub fn jp_addr(chip8: &mut Chip8) {
    chip8.pc = chip8.opcode & 0x0FFF;
}

/// 2nnn - Call subroutine at nnn.
pub fn call_addr(chip8: &mut Chip8) {}

/// 3xkk - Skip next instruction if Vx = kk.
pub fn se_vx_byte(mut chip8: Chip8) {}

/// 4xkk - Skip next instruction if Vx != kk.
pub fn sne_vx_byte(chip8: &mut Chip8) {}

/// 5xy0 - Skip next instruction if Vx = Vy.
pub fn se_vx_vy(chip8: &mut Chip8) {}