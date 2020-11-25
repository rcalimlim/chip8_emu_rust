extern crate rand;
use crate::chip8::Chip8;
use crate::utils::*;
use std::num::Wrapping;

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
    chip8.pc += 2;
}

/// `1nnn` - Jump to location nnn.
pub fn jp_addr(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.pc = vars.nnn;
}

/// `2nnn` - Call subroutine at nnn.
pub fn call_addr(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.sp += 1;
    chip8.stack[chip8.sp as usize] = chip8.pc;
    chip8.pc = vars.nnn;
}

/// `3xkk` - Skip next instruction if Vx = kk.
pub fn se_vx_byte(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);

    if chip8.v[vars.x] == vars.kk {
        chip8.pc += 4;
    } else {
        chip8.pc += 2;
    }
}

/// `4xkk` - Skip next instruction if Vx != kk.
pub fn sne_vx_byte(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);

    if chip8.v[vars.x] != vars.kk {
        chip8.pc += 4;
    } else {
        chip8.pc += 2;
    }
}

/// `5xy0` - Skip next instruction if Vx = Vy.
pub fn se_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);

    if chip8.v[vars.x] == chip8.v[vars.y] {
        chip8.pc += 4;
    } else {
        chip8.pc += 2;
    }
}

/// `6xkk` - Set Vx = kk.
pub fn ld_vx_byte(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.v[vars.x] = vars.kk;
    chip8.pc += 2;
}

/// `7xkk` - Set Vx = Vx + kk.
pub fn add_vx_byte(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    let vx = chip8.v[vars.x] as u16;
    chip8.v[vars.x] = (vx + vars.kk as u16) as u8;
    chip8.pc += 2;
}

/// `8xy0` - Set Vx = Vy.
pub fn ld_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.v[vars.x] = chip8.v[vars.y];
    chip8.pc += 2;
}

/// `8xy1` - Set Vx = Vx OR Vy.
pub fn or_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.v[vars.x] = chip8.v[vars.x] | chip8.v[vars.y];
    chip8.pc += 2;
}

/// `8xy2` - Set Vx = Vx AND Vy.
pub fn and_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.v[vars.x] = chip8.v[vars.x] & chip8.v[vars.y];
    chip8.pc += 2;
}

/// `8xy3` - Set Vx = Vx XOR Vy.
pub fn xor_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.v[vars.x] = chip8.v[vars.x] ^ chip8.v[vars.y];
    chip8.pc += 2;
}

/// `8xy4` - Set Vx = Vx + Vy, set VF = carry.
pub fn add_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    let vx: u16 = chip8.v[vars.x].into();
    let vy: u16 = chip8.v[vars.y].into();
    let mut sum = vx + vy;

    if sum > 255 {
        sum = sum & 0x00FF;
        chip8.v[0xF] = 1;
    }

    chip8.v[vars.x] = sum as u8;
    chip8.pc += 2;
}

/// `8xy5` - Set Vx = Vx - Vy, set VF = NOT borrow.
pub fn sub_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.v[0xF] = (chip8.v[vars.x] > chip8.v[vars.y]) as u8;

    let lhs = Wrapping(chip8.v[vars.x]);
    let rhs = Wrapping(chip8.v[vars.y]);
    chip8.v[vars.x] = (lhs - rhs).0;
    chip8.pc += 2;
}

/// `8xy6` - Set Vx = Vx SHR 1.
pub fn shr_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    match chip8.v[vars.x] & 0b1 {
        0b1 => {
            chip8.v[0xF] = 1;
            chip8.v[vars.x] = chip8.v[vars.x] >> 1;
        }
        _ => {
            chip8.v[0xF] = 0;
        }
    }
    chip8.pc += 2;
}

/// `8xy7` - Set Vx = Vy - Vx, set VF = NOT borrow.
pub fn subn_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    match chip8.v[vars.y] > chip8.v[vars.x] {
        true => {
            chip8.v[vars.x] = chip8.v[vars.y] - chip8.v[vars.x];
            chip8.v[0xF] = 1;
        }
        false => {
            chip8.v[0xF] = 0;
        }
    }
    chip8.pc += 2;
}

/// `8xyE` - Set Vx = Vx SHL 1.
pub fn shl_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    match chip8.v[vars.x] >> 7 {
        1 => {
            chip8.v[0xF] = 1;
        }
        _ => {
            chip8.v[0xF] = 0;
        }
    }
    chip8.v[vars.x] = chip8.v[vars.x] << 1;
    chip8.pc += 2;
}

/// `9xy0` - Skip next instruction if Vx != Vy.
pub fn sne_vx_vy(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    if chip8.v[vars.x] != chip8.v[vars.y] {
        chip8.pc += 4;
    } else {
        chip8.pc += 2;
    }
}

/// `Annn` - Set I = nnn.
pub fn ld_i_addr(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.i = vars.nnn;
    chip8.pc += 2;
}

/// `Bnnn` - Jump to location nnn + V0.
pub fn jp_v0_addr(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.pc = vars.nnn + chip8.v[0x0] as u16;
}

/// `Cxkk` - Set Vx = random byte AND kk.
pub fn rnd_vx_byte(chip8: &mut Chip8, rnd_fn: fn() -> u8) {
    let vars = opcode_to_variables(&chip8.opcode);
    chip8.v[vars.x] = vars.kk & rnd_fn();
    chip8.pc += 2;
}

/// `Dxyn` - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
pub fn drw_vx_vy_nibble(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    let vx = chip8.v[vars.x] as usize;
    let vy = chip8.v[vars.y] as usize;
    let n = vars.nibbles[3];
    let sprite_i = chip8.i as usize;
    for (i, &sprite) in chip8.memory[sprite_i..sprite_i + n].iter().enumerate() {
        let row = ((vy + i) % 32) * 64;
        for (j, &new_bit) in into_bit_vec(sprite).iter().enumerate() {
            let offset = (vx + j) % 64;
            let bit_index = row + offset;
            let old_bit = chip8.gfx[bit_index];

            if old_bit == 1 && new_bit == 1 {
                chip8.v[0xF] = 1;
            }

            let result = old_bit ^ new_bit;
            chip8.gfx[bit_index] = result;
        }
    }

    chip8.should_draw = true;
    chip8.pc += 2;
}

/// `Ex9E` - Skip next instruction if key with the value of Vx is pressed.
pub fn skp_vx(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    if chip8.key[vars.x] == true {
        chip8.pc += 2;
    }
}

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
    // let nibbles = to_nibbles(&chip8.opcode);
    let vars = opcode_to_variables(&chip8.opcode);
    let num = chip8.v[vars.x];

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
pub fn ld_i_vx(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    let mem_i = chip8.i as usize;
    for i in 0..(vars.x + 1) {
        chip8.memory[mem_i + i] = chip8.v[i]
    }
    chip8.pc += 2;
}

/// `Fx65` - Read registers V0 through Vx from memory starting at location I.
pub fn ld_vx_i(chip8: &mut Chip8) {
    let vars = opcode_to_variables(&chip8.opcode);
    let mem_start = chip8.i as usize;
    for (i, &val) in chip8.memory[mem_start..mem_start + vars.x]
        .iter()
        .enumerate()
    {
        chip8.v[i] = val;
    }
    chip8.pc += 2;
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Chip8 {
        Chip8::initialize()
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
            test_addr + 2,
            chip8.pc,
            "should set program counter to address at the top of the stack and add 2"
        );
        assert_eq!(
            initial_sp - 1,
            chip8.sp,
            "should decrement stack pointer by one"
        );
    }

    #[test]
    fn test_jp_addr() {
        let mut chip8 = setup();
        let test_opcode = 0x1ABC;
        let initial_pc = 512;
        chip8.opcode = test_opcode;
        chip8.pc = initial_pc;
        jp_addr(&mut chip8);
        assert_eq!(
            test_opcode & 0x0FFF,
            chip8.pc,
            "should set program counter to `nnn`"
        );
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
    fn test_se_vx_byte_eq() {
        let mut chip8 = setup();
        let test_opcode = 0x32B0;
        let initial_pc = 512;
        chip8.opcode = test_opcode;
        chip8.v[2] = 0xB0;
        chip8.pc = initial_pc;
        se_vx_byte(&mut chip8);

        assert_eq!(
            initial_pc + 4,
            chip8.pc,
            "should increment program counter by 4 when `vx` equals `kk`"
        );
    }

    #[test]
    fn test_se_vx_byte_neq() {
        let mut chip8 = setup();
        let test_opcode = 0x32B0;
        let initial_pc = 512;
        chip8.opcode = test_opcode;
        chip8.v[2] = 0xFF;
        chip8.pc = initial_pc;
        se_vx_byte(&mut chip8);

        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2 when `vx` does not equal `kk`"
        );
    }

    #[test]
    fn test_sne_vx_byte_neq() {
        let mut chip8 = setup();
        let test_opcode = 0x32B0;
        let initial_pc = 512;
        chip8.opcode = test_opcode;
        chip8.v[2] = 0xFF;
        chip8.pc = initial_pc;
        sne_vx_byte(&mut chip8);

        assert_eq!(
            initial_pc + 4,
            chip8.pc,
            "should increment program counter by 4 when `vx` does not equal `kk`"
        );
    }

    #[test]
    fn test_sne_vx_byte_eq() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x4CB0;
        chip8.v[0xC] = 0xB0;
        chip8.pc = initial_pc;
        sne_vx_byte(&mut chip8);

        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2 when `vx` equals `kk`"
        );
    }

    #[test]
    fn test_se_vx_vy_eq() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x5CE0;
        chip8.pc = initial_pc;
        chip8.v[0xC] = 0xE;
        chip8.v[0xE] = 0xE;
        se_vx_vy(&mut chip8);

        assert_eq!(
            initial_pc + 4,
            chip8.pc,
            "should increment program counter by 4 when `vx` equals `vy`"
        )
    }

    #[test]
    fn test_se_vx_vy_neq() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x5CE0;
        chip8.pc = initial_pc;
        chip8.v[0xC] = 0xE;
        chip8.v[0xE] = 0xF;
        se_vx_vy(&mut chip8);

        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2 when `vx` does not equal `vy`"
        )
    }

    #[test]
    fn test_ld_vx_byte() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x60AA;
        chip8.v[0x0] = 5;
        chip8.pc = initial_pc;
        ld_vx_byte(&mut chip8);

        assert_eq!(0xAA, chip8.v[0x0], "should load `kk` into `vx`");
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8690;
        chip8.v[0x6] = 0xDA;
        chip8.v[0x9] = 0x12;
        chip8.pc = initial_pc;
        ld_vx_vy(&mut chip8);

        assert_eq!(0x12, chip8.v[0x6], "should store the value of `vy` in `vx`");
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    pub fn test_or_vx_vy() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8DB1;
        chip8.v[0xD] = 0xA;
        chip8.v[0xB] = 0x5;
        or_vx_vy(&mut chip8);

        assert_eq!(
            0xA | 0x5,
            chip8.v[0xD],
            "should store the result of `vx` OR `vy` in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    pub fn test_and_vx_vy() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8DB1;
        chip8.v[0xD] = 0xA;
        chip8.v[0xB] = 0x5;
        and_vx_vy(&mut chip8);

        assert_eq!(
            0xA & 0x5,
            chip8.v[0xD],
            "should store the result of `vx` AND `vy` in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    pub fn test_xor_vx_vy() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8DB1;
        chip8.v[0xD] = 0xA;
        chip8.v[0xB] = 0x5;
        xor_vx_vy(&mut chip8);

        assert_eq!(
            0xA ^ 0x5,
            chip8.v[0xD],
            "should store the result of `vx` XOR `vy` in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_add_vx_byte() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x7210;
        chip8.v[0x2] = 0x6;
        chip8.pc = initial_pc;
        add_vx_byte(&mut chip8);

        assert_eq!(
            0x16, chip8.v[0x2],
            "should add `vx` to `kk` and store it in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_add_vx_byte_overflow() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x76FF;
        chip8.v[0x2] = 0x6;
        chip8.pc = initial_pc;
        add_vx_byte(&mut chip8);

        assert_eq!(
            0x6, chip8.v[0x2],
            "should add `vx` to `kk` and store it in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
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
        chip8.v[0xF] = 0;
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
        chip8.v[0xF] = 0;
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
    fn test_sub_vx_vy_greater_than() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8355;
        chip8.v[0x3] = 100;
        chip8.v[0x5] = 25;
        chip8.v[0xF] = 0;
        chip8.pc = initial_pc;
        sub_vx_vy(&mut chip8);

        assert_eq!(1, chip8.v[0xF], "should set `vf` to 1 when `vx` > `vy`");
        assert_eq!(
            75, chip8.v[0x3],
            "should subtract `vy` from `vx` and store results in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_sub_vx_vy_less_than() {
        // Revisit this, as it's possible it's wrong
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8355;
        chip8.v[0x3] = 25;
        chip8.v[0x5] = 100;
        chip8.v[0xF] = 0;
        chip8.pc = initial_pc;
        sub_vx_vy(&mut chip8);

        assert_eq!(0, chip8.v[0xF], "should not set `vf` to 1 when `vx` < `vy`");
        assert_eq!(
            181, chip8.v[0x3],
            "should not subtract `vy` from `vx` and store results in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_shr_vx_vy_is_one() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8016;
        chip8.v[0x0] = 0b1001;
        shr_vx_vy(&mut chip8);

        assert_eq!(
            1, chip8.v[0xF],
            "should set `vf` to 1 when least significant bit of `vx` is 1"
        );
        assert_eq!(
            4, chip8.v[0x0],
            "should divide `vx` by 2 and store result in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_shr_vx_vy_is_not_one() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8016;
        chip8.v[0x0] = 0b1000;
        shr_vx_vy(&mut chip8);

        assert_eq!(
            0, chip8.v[0xF],
            "should not set `vf` to 1 when least-significant bit is 0"
        );
        assert_eq!(8, chip8.v[0x0], "should not change `vx`");
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_subn_vx_vy_greater_than() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8CD7;
        chip8.v[0xD] = 200;
        chip8.v[0xC] = 160;
        chip8.v[0xF] = 0;
        subn_vx_vy(&mut chip8);

        assert_eq!(
            40, chip8.v[0xC],
            "should subtract `vx` from `vy` when `vy` > `vx` and store the result in `vx`"
        );
        assert_eq!(1, chip8.v[0xF], "should set `vf` to 1 when `vy` > `vx`");
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_subn_vx_vy_less_than() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x8CD7;
        chip8.v[0xD] = 160;
        chip8.v[0xC] = 200;
        chip8.v[0xF] = 0;
        subn_vx_vy(&mut chip8);

        assert_eq!(
            200, chip8.v[0xC],
            "should not subtract `vx` from `vy` when `vy` < `vx`"
        );
        assert_eq!(0, chip8.v[0xF], "should set `vf` to 0 when `vy` < `vx`");
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_shl_vx_vy_is_one() {
        let mut chip8 = setup();
        let initial_pc = 512;
        let initial_vx: u8 = 0b10000000;
        chip8.opcode = 0x8ABE;
        chip8.v[0xA] = initial_vx;
        shl_vx_vy(&mut chip8);

        assert_eq!(
            1, chip8.v[0xF],
            "should set `vf` to 1 when most-significant bit is 1"
        );
        assert_eq!(
            initial_vx << 1,
            chip8.v[0xA],
            "should multiply `vx` by 2 and store result in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_shl_vx_vy_is_not_one() {
        let mut chip8 = setup();
        let initial_pc = 512;
        let initial_vx: u8 = 0b1;
        chip8.opcode = 0x8ABE;
        chip8.v[0xA] = initial_vx;
        shl_vx_vy(&mut chip8);

        assert_eq!(
            0, chip8.v[0xF],
            "should set `vf` to 0 when most-significant bit is not 1"
        );
        assert_eq!(
            initial_vx << 1,
            chip8.v[0xA],
            "should multiply `vx` by 2 and store result in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_sne_vx_vy_neq() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x9120;
        chip8.v[0x1] = 100;
        chip8.v[0x2] = 150;
        chip8.pc = initial_pc;
        sne_vx_vy(&mut chip8);

        assert_eq!(
            initial_pc + 4,
            chip8.pc,
            "should increment program counter by 4 when `vx` and `vy` are not equal"
        );
    }

    #[test]
    fn test_sne_vx_vy_eq() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0x9120;
        chip8.v[0x1] = 100;
        chip8.v[0x2] = 100;
        chip8.pc = initial_pc;
        sne_vx_vy(&mut chip8);

        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2 when `vx` and `vy` are equal"
        );
    }

    #[test]
    fn test_ld_i_addr() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0xA666;
        chip8.i = 0;
        ld_i_addr(&mut chip8);

        assert_eq!(0x666, chip8.i, "should load addr into register i");
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_jp_v0_addr() {
        let mut chip8 = setup();
        chip8.opcode = 0xB512;
        chip8.v[0x0] = 100;
        chip8.pc = 512;
        jp_v0_addr(&mut chip8);

        assert_eq!(
            0x512 + 100,
            chip8.pc,
            "should set program counter to `nnn` + `v0`"
        );
    }

    #[test]
    fn test_rnd_vx_byte() {
        let mut chip8 = setup();
        let initial_pc = 512;
        chip8.opcode = 0xC144;
        chip8.pc = initial_pc;
        rnd_vx_byte(&mut chip8, || 0x40);

        assert_eq!(
            0x44 & 0x40,
            chip8.v[0x1],
            "should AND random number and `kk` and store result in `vx`"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_drw_vx_vy_nibble_no_collision_no_wrap() {
        let mut chip8 = setup();
        let sprites = [0b10101100; 4];
        let vx: usize = 0;
        let vy: usize = 0;
        chip8.opcode = 0xD8B4;
        chip8.i = 1000;
        chip8.v[0xF] = 0;
        chip8.v[0x8] = vx as u8;
        chip8.v[0xB] = vy as u8;
        chip8.gfx = [0; 64 * 32];
        chip8.memory = [0; 4096];
        // prepare memory with sprites
        for (sprite_i, &sprite) in sprites.iter().enumerate() {
            chip8.memory[chip8.i as usize + sprite_i] = sprite;
        }
        let old_gfx = chip8.gfx; // save a copy of initial gfx state
        drw_vx_vy_nibble(&mut chip8);

        for (i, &sprite) in sprites.iter().enumerate() {
            for (j, &sprite_bit) in into_bit_vec(sprite).iter().enumerate() {
                let row = (64 * (vy * i)) % 32;
                let offset = (vx + j) % 64;
                let bit_index = row + offset;
                let old_bit = old_gfx[bit_index];
                let new_bit = chip8.gfx[bit_index];

                assert_eq!(
                    old_bit ^ sprite_bit,
                    new_bit,
                    "should xor bits to position {}",
                    bit_index
                );
            }
        }
        assert_eq!(true, chip8.should_draw, "should draw to screen");
        assert_eq!(
            0, chip8.v[0xF],
            "should not set `vf` when there's no collision"
        );
    }

    #[test]
    fn test_drw_vx_vy_nibble_no_collision_wrap() {
        let mut chip8 = setup();
        let sprites = [0b10101100; 4];
        let vx: usize = 56;
        let vy: usize = 30;
        chip8.opcode = 0xD8B4;
        chip8.i = 1000;
        chip8.v[0xF] = 0;
        chip8.v[0x8] = vx as u8;
        chip8.v[0xB] = vy as u8;
        chip8.gfx = [0; 64 * 32];
        chip8.memory = [0; 4096];
        // prepare memory with sprites
        for (sprite_i, &sprite) in sprites.iter().enumerate() {
            chip8.memory[chip8.i as usize + sprite_i] = sprite;
        }
        let old_gfx = chip8.gfx; // save a copy of initial gfx state
        drw_vx_vy_nibble(&mut chip8);

        for (i, &sprite) in sprites.iter().enumerate() {
            for (j, &sprite_bit) in into_bit_vec(sprite).iter().enumerate() {
                let row = ((vy + i) % 32) * 64;
                let offset = (vx + j) % 64;
                let bit_index = row + offset;
                let old_bit = old_gfx[bit_index];
                let new_bit = chip8.gfx[bit_index];

                assert_eq!(
                    old_bit ^ sprite_bit,
                    new_bit,
                    "should xor bits to position {}",
                    bit_index
                );
            }
        }
        assert_eq!(true, chip8.should_draw, "should draw to screen");
        assert_eq!(
            0, chip8.v[0xF],
            "should not set `vf` when there's no collision"
        );
    }

    #[test]
    fn test_drw_vx_vy_nibble_collision_no_wrap() {
        let mut chip8 = setup();
        let sprites = [0b10101100; 4];
        let vx: usize = 0;
        let vy: usize = 0;
        chip8.opcode = 0xD8B4;
        chip8.i = 1000;
        chip8.v[0xF] = 0;
        chip8.v[0x8] = vx as u8;
        chip8.v[0xB] = vy as u8;
        chip8.gfx = [1; 64 * 32];
        chip8.memory = [0; 4096];
        // prepare memory with sprites
        for (sprite_i, &sprite) in sprites.iter().enumerate() {
            chip8.memory[chip8.i as usize + sprite_i] = sprite;
        }
        let old_gfx = chip8.gfx; // save a copy of initial gfx state
        drw_vx_vy_nibble(&mut chip8);

        for (i, &sprite) in sprites.iter().enumerate() {
            for (j, &sprite_bit) in into_bit_vec(sprite).iter().enumerate() {
                let row = ((vy + i) % 32) * 64;
                let offset = (vx + j) % 64;
                let bit_index = row + offset;
                let old_bit = old_gfx[bit_index];
                let new_bit = chip8.gfx[bit_index];

                assert_eq!(
                    old_bit ^ sprite_bit,
                    new_bit,
                    "should xor bits to position {}",
                    bit_index
                );
            }
        }
        assert_eq!(true, chip8.should_draw, "should draw to screen");
        assert_eq!(1, chip8.v[0xF], "should set `vf` when there's collision");
    }

    #[test]
    fn test_drw_vx_vy_nibble_collision_wrap() {
        let mut chip8 = setup();
        let sprites = [0b10101100; 4];
        let vx: usize = 58;
        let vy: usize = 26;
        chip8.opcode = 0xD8B4;
        chip8.i = 1000;
        chip8.v[0xF] = 0;
        chip8.v[0x8] = vx as u8;
        chip8.v[0xB] = vy as u8;
        chip8.gfx = [1; 64 * 32];
        chip8.memory = [0; 4096];
        // prepare memory with sprites
        for (sprite_i, &sprite) in sprites.iter().enumerate() {
            chip8.memory[chip8.i as usize + sprite_i] = sprite;
        }
        let old_gfx = chip8.gfx; // save a copy of initial gfx state
        drw_vx_vy_nibble(&mut chip8);

        for (i, &sprite) in sprites.iter().enumerate() {
            for (j, &sprite_bit) in into_bit_vec(sprite).iter().enumerate() {
                let row = ((vy + i) % 32) * 64;
                let offset = (vx + j) % 64;
                let bit_index = row + offset;
                let old_bit = old_gfx[bit_index];
                let new_bit = chip8.gfx[bit_index];

                assert_eq!(
                    old_bit ^ sprite_bit,
                    new_bit,
                    "should xor bits to position {}",
                    bit_index
                );
            }
        }
        assert_eq!(true, chip8.should_draw, "should draw to screen");
        assert_eq!(1, chip8.v[0xF], "should set `vf` when there's collision");
    }

    #[test]
    fn test_skp_vx() {
        let mut chip8 = setup();
        chip8.opcode = 0xE19E;
        chip8.pc = 512;
        chip8.key[1] = true;
        skp_vx(&mut chip8);

        assert_eq!(
            514, chip8.pc,
            "should skip next instruction if `vx` value key is pressed"
        );
    }

    #[test]
    fn test_sknp_vx() {}

    #[test]
    fn test_ld_vx_dt() {}

    #[test]
    fn test_ld_vx_k() {}

    #[test]
    fn test_ld_dt_vx() {}

    #[test]
    fn test_ld_st_vx() {}

    #[test]
    fn test_add_i_vx() {}

    #[test]
    fn test_ld_f_vx() {}

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

    #[test]
    fn test_ld_i_vx() {
        let mut chip8 = setup();
        let initial_pc = 512;
        let mem_start: usize = 1024;
        chip8.opcode = 0xFF55;
        chip8.pc = initial_pc;
        chip8.i = mem_start as u16;
        for i in 0..16 {
            chip8.v[i] = 123;
            chip8.memory[mem_start + i] = 0;
        }
        ld_i_vx(&mut chip8);

        assert_eq!(
            chip8.v[0..0xF],
            chip8.memory[mem_start..mem_start + 0xF],
            "should copy `v` registers from `v0` to `v[x]` into memory"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }

    #[test]
    fn test_ld_vx_i() {
        let mut chip8 = setup();
        let initial_pc = 512;
        let mem_start: usize = 1024;
        chip8.opcode = 0xFF65;
        chip8.pc = initial_pc;
        chip8.i = mem_start as u16;
        for i in 0..0xF {
            chip8.v[i] = 0;
            chip8.memory[mem_start + i] = 123;
        }
        ld_vx_i(&mut chip8);

        assert_eq!(
            chip8.memory[mem_start..mem_start + 0xF],
            chip8.v[0..0xF],
            "should copy values from memory into `v` registers"
        );
        assert_eq!(
            initial_pc + 2,
            chip8.pc,
            "should increment program counter by 2"
        );
    }
}
