use crate::chip8::Chip8;

/// 0nnn - Jump to a machine code routine at nnn.
pub fn sys_addr(chip8: &mut Chip8) {
    chip8.pc = chip8.opcode & 0x0FFF;
}

/// 00E0 - Clear the display.
pub fn cls(chip8: &mut Chip8) {}

/// 00EE - Return from a subroutine.
pub fn ret(chip8: &mut Chip8) {
    chip8.pc = chip8.stack[chip8.sp as usize];
    chip8.sp -= 1;
}

/// `1nnn` - Jump to location nnn.
pub fn jp_addr(chip8: &mut Chip8) {
    chip8.pc = chip8.opcode & 0x0FFF;
}

/// `2nnn` - Call subroutine at nnn.
pub fn call_addr(chip8: &mut Chip8) {
    chip8.sp += 1;
    chip8.stack[chip8.sp as usize] = chip8.pc;
    chip8.pc = chip8.opcode & 0x0FFF;
}

/// `3xkk` - Skip next instruction if Vx = kk.
pub fn se_vx_byte(chip8: &mut Chip8) {}

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
pub fn add_vx_vy(chip8: &mut Chip8) {
    let nibbles = to_nibbles(&chip8.opcode);

    let x = nibbles[1];
    let y = nibbles[2];

    let vx: u16 = chip8.v[x].into();
    let vy: u16 = chip8.v[y].into();
    let mut sum = vx + vy;

    if sum > 255 {
        sum = sum & 0x00FF;
        chip8.v[0xF] = 1;
    }

    chip8.v[x] = sum as u8;
    chip8.pc += 2;
}

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
pub fn ld_b_vx(chip8: &mut Chip8) {
    let nibbles = to_nibbles(&chip8.opcode);
    let v_index = nibbles[1];
    let num = chip8.v[v_index];

    let hundreds = (num / 100) as u8;
    let tens = (num % 100 / 10) as u8;
    let ones = (num % 100 % 10) as u8;

    let i = chip8.i as usize;

    chip8.memory[i] = hundreds;
    chip8.memory[i + 1] = tens;
    chip8.memory[i + 2] = ones;
    chip8.pc += 2;
}

/// `Fx55` - Store registers V0 through Vx in memory starting at location I.
pub fn ld_i_vx(chip8: &mut Chip8) {}

/// `Fx65` - Read registers V0 through Vx from memory starting at location I.
pub fn ld_vx_i(chip8: &mut Chip8) {}

/// Helper function that returns the nibble from the passed index (starting from 0) bit-shifted
/// to the right.
fn to_nibbles(word: &u16) -> [usize; 4] {
    [
        ((word & 0xF000) >> 12) as usize,
        ((word & 0x0F00) >> 8) as usize,
        ((word & 0x00F0) >> 4) as usize,
        (word & 0x000F) as usize,
    ]
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    fn setup() -> Chip8 {
        let mut chip8 = Chip8::new();
        chip8.initialize();
        chip8
    }

    #[test]
    fn test_sys_addr() {
        let mut chip8 = setup();
        let test_opcode = 0x1ABC;
        chip8.opcode = test_opcode;
        sys_addr(&mut chip8);
    }

    #[test]
    fn test_ret() {
        let mut chip8 = setup();
        let test_addr = 0x0ABC;
        let initial_sp = 1;
        chip8.sp = initial_sp;
        chip8.stack[chip8.sp as usize] = test_addr;
        chip8.opcode = 0x00EE;
        ret(&mut chip8);
        assert_eq!(
            chip8.pc, test_addr,
            "should set program counter to address at the top of the stack"
        );
        assert_eq!(
            chip8.sp,
            initial_sp - 1,
            "should decrement stack pointer by one"
        );
    }

    #[test]
    fn test_jp_addr() {
        let mut chip8 = setup();
        let test_opcode = 0x1ABC;
        chip8.opcode = test_opcode;
        jp_addr(&mut chip8);
        assert_eq!(chip8.pc, test_opcode & 0x0FFF);
    }

    #[test]
    fn test_call_addr() {
        let mut chip8 = setup();
        let test_opcode = 0x2ABC;
        let initial_sp = chip8.sp;
        let initial_pc = chip8.pc;
        chip8.opcode = test_opcode;
        call_addr(&mut chip8);

        assert_eq!(
            chip8.sp,
            initial_sp + 1,
            "should increment the stack pointer by one"
        );
        assert_eq!(
            chip8.stack[chip8.sp as usize], initial_pc,
            "should move the current program counter to the stack"
        );
        assert_eq!(
            chip8.pc,
            test_opcode & 0x0FFF,
            "should set the program counter to `nnn` of opcode"
        );
    }

    #[test]
    fn test_add_vx_vy_no_carry() {
        let mut chip8 = setup();
        let test_opcode = 0x8454;
        let initial_pc = chip8.pc;
        let initial_v4 = 100;
        let initial_v5 = 50;
        chip8.opcode = test_opcode;
        chip8.v[4] = initial_v4;
        chip8.v[5] = initial_v5;
        chip8.v[0xF] = 0; // make sure vf is 0
        add_vx_vy(&mut chip8);

        assert_eq!(
            chip8.v[0x4],
            initial_v4 + initial_v5,
            "should add `vx` and `vy` and store value in `vx`"
        );
        assert_eq!(chip8.v[0xF], 0, "should not set `vf` when there's no carry");
        assert_eq!(
            chip8.pc,
            initial_pc + 2,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_add_vx_vy_carry() {
        let mut chip8 = setup();
        let test_opcode = 0x8454;
        let initial_pc = chip8.pc;
        let initial_v4: u16 = 200;
        let initial_v5: u16 = 200;
        chip8.opcode = test_opcode;
        chip8.v[4] = initial_v4 as u8;
        chip8.v[5] = initial_v5 as u8;
        chip8.v[0xF] = 0; // make sure vf is 0
        add_vx_vy(&mut chip8);

        assert_eq!(
            chip8.v[0x4],
            ((initial_v4 + initial_v5) & 0x00FF) as u8,
            "should add `vx` and `vy` and store lowest 8 bits in `vx`"
        );
        assert_eq!(
            chip8.v[0xF], 1,
            "should set `vf` when `vx` + `vy` overflows"
        );
        assert_eq!(
            chip8.pc,
            initial_pc + 2,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_ld_b_vx() {
        let mut chip8 = setup();
        let test_opcode = 0xFB33;
        let initial_i: usize = 100;
        let initial_pc = 5;
        chip8.opcode = test_opcode;
        chip8.v[0xB] = 123;
        chip8.i = initial_i as u16;
        chip8.pc = initial_pc;
        ld_b_vx(&mut chip8);

        assert_eq!(
            chip8.memory[initial_i..(initial_i + 3)],
            [1, 2, 3],
            "should store correct values in memory"
        );
        assert_eq!(
            chip8.pc,
            initial_pc + 2,
            "should increment program counter by 2"
        );
    }
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn test_to_nibbles() {
        let word: u16 = 0xA0CD;
        let nibbles = to_nibbles(&word);

        assert_eq!(
            [0xA, 0x0, 0xC, 0xD],
            nibbles,
            "should return correct array of nibbles"
        );
    }
}
