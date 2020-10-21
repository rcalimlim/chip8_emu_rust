mod chip8;
mod instructions;
mod lib;
use chip8::Chip8;

fn main() {
    // setup graphics
    // setup input

    let mut chip8 = Chip8::new();
    chip8.initialize();
    chip8.load_rom("../chip8-test-rom/test_opcode.ch8");

    loop {
        chip8.emulate_cycle();

        if chip8.should_draw() {
            // draw to screen
        }

        chip8.set_keys();

        break;
    }
}
