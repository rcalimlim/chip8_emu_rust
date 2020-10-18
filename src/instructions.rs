use crate::chip8::Chip8;

/// 0nnn - Jump to a machine code routine at nnn.
pub fn sys_addr(chip8: &mut Chip8) {}

/// 00E0 - Clear the display.
pub fn cls(chip8: &mut Chip8) {}

/// 00EE - Return from a subroutine.
pub fn ret(chip8: &mut Chip8) {}

/// `1nnn` - Jump to location nnn.
pub fn jp_addr(chip8: &mut Chip8) {
    chip8.pc = chip8.opcode & 0x0FFF;
}

/// `2nnn` - Call subroutine at nnn.
pub fn call_addr(chip8: &mut Chip8) {}

/// `3xkk` - Skip next instruction if Vx = kk.
pub fn se_vx_byte(mut chip8: Chip8) {}

/// `4xkk` - Skip next instruction if Vx != kk.
pub fn sne_vx_byte(chip8: &mut Chip8) {}

/// `5xy0` - Skip next instruction if Vx = Vy.
pub fn se_vx_vy(chip8: &mut Chip8) {}

/// `6xkk` - Set Vx = kk.
pub fn ld_vx_byte(chip8: &mut Chip8) {}

/// `7xkk` - Set Vx = Vx + kk.
pub fn add_vx_byte(chip8: &mut Chip8) {}

/// `8xy0` - Set Vx = Vy.
pub fn ld_vx_vy(chip8: &mut Chip8) {}

/// `8xy1` - Set Vx = Vx OR Vy.
pub fn or_vx_vy(chip8: &mut Chip8) {}

/// `8xy2` - Set Vx = Vx AND Vy.
pub fn and_vx_vy(chip8: &mut Chip8) {}

/// `8xy3` - Set Vx = Vx XOR Vy.
pub fn xor_vx_vy(chip8: &mut Chip8) {}

/// `8xy4` - Set Vx = Vx + Vy, set VF = carry. 
pub fn add_vx_vy(chip8: &mut Chip8) {}

/// `8xy5` - Set Vx = Vx - Vy, set VF = NOT borrow.
pub fn sub_vx_vy(chip8: &mut Chip8) {}

/// `8xy6` - Set Vx = Vx SHR 1.
pub fn shr_vx_vy(chip8: &mut Chip8) {}

/// `8xy7` - Set Vx = Vy - Vx, set VF = NOT borrow.
pub fn subn_vx_vy(chip8: &mut Chip8) {}

/// `8xyE` - Set Vx = Vx SHL 1.
pub fn shl_vx_vy(chip8: &mut Chip8) {}

/// `9xy0` - Skip next instruction if Vx != Vy.
pub fn sne_vx_vy(chip8: &mut Chip8) {}

/// `Annn` - Set I = nnn.
pub fn ld_i_addr(chip8: &mut Chip8) {}

/// `Bnnn` - Jump to location nnn + V0.
pub fn jp_v0_addr(chip8: &mut Chip8) {}

/// `Cxkk` - Set Vx = random byte AND kk.
pub fn rnd_vx_byte(chip8: &mut Chip8) {}

/// `Dxyn` - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
pub fn drw_vx_vy_nibble(chip8: &mut Chip8) {}

/// `Ex9E` - Skip next instruction if key with the value of Vx is pressed.
pub fn skp_vx(chip8: &mut Chip8) {}

/// `ExA1` - Skip next instruction if key with the value of Vx is not pressed.
pub fn sknp_vx(chip8: &mut Chip8) {}

/// `Fx07` - Set Vx = delay timer value.
pub fn ld_vx_dt(chip8: &mut Chip8) {}

/// `Fx0A` - Wait for a key press, store the value of the key in Vx.
pub fn ld_vx_k(chip8: &mut Chip8) {}

/// `Fx15` - Set delay timer = Vx.
pub fn ld_dt_vx(chip8: &mut Chip8) {}

/// `Fx18` - Set sound timer = Vx.
pub fn ld_st_vx(chip8: &mut Chip8) {}

/// `Fx1E` - Set I = I + Vx.
pub fn add_i_vx(chip8: &mut Chip8) {}

/// `Fx29` - Set I = location of sprite for digit Vx.
pub fn ld_f_vx(chip8: &mut Chip8) {}

/// `Fx33` - Store BCD representation of Vx in memory locations I, I+1, and I+2.
pub fn ld_b_vx(chip8: &mut Chip8) {}

/// `Fx55` - Store registers V0 through Vx in memory starting at location I.
pub fn ld_i_vx(chip8: &mut Chip8) {}

/// `Fx65` - Read registers V0 through Vx from memory starting at location I.
pub fn ld_vx_i(chip8: &mut Chip8) {}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Chip8 {
        let mut chip8 = Chip8::new();
        chip8.initialize();
        chip8
    }

    #[test]
    fn test_jp_addr() {
        let mut chip8 = setup();
        let test_opcode = 0x1ABC;
        chip8.opcode = test_opcode;
        jp_addr(&mut chip8);
        assert_eq!(chip8.pc, test_opcode & 0x0FFF);
    }
}
