mod chip8;
mod input_output;
mod instructions;
mod utils;

use chip8::Chip8;
use input_output::InputOutput;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const SCALE: u32 = 12;

fn main() {
    let mut chip8 = Chip8::initialize();
    chip8.load_rom("../chip8-test-rom/test_opcode.ch8");

    let sdl_context = sdl2::init().unwrap();
    let mut io = InputOutput::initialize(&sdl_context, SCALE);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => io.handle_key_down(&mut chip8, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => io.handle_key_up(&mut chip8, keycode),
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
        chip8.emulate_cycle();

        if chip8.should_draw == true {
            io.draw_canvas(&mut chip8, SCALE);
        }
    }
}
