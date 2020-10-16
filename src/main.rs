mod chip_8;
use chip_8::Chip8;

fn main() {
    let chip8 = Chip8{};

    chip8.initialize();
    chip8.emulate_cycle();
}
