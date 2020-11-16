use crate::chip8::Chip8;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct InputOutput {
    pub canvas: Canvas<Window>,
    device: AudioDevice<SquareWave>,
}

impl InputOutput {
    /// Initializes Core
    pub fn initialize(sdl_context: &Sdl, scale: u32) -> Self {
        // Set up audio
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44000),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // Initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        // Set up video
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rfc chip8", 64 * scale, 32 * scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Self { canvas, device }
    }

    /// Handles key down event
    pub fn handle_key_down(&mut self, chip8: &mut Chip8, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => {
                chip8.set_key(0x1, true);
            }
            Keycode::Num2 => {
                chip8.set_key(0x2, true);
            }
            Keycode::Num3 => {
                chip8.set_key(0x3, true);
            }
            Keycode::Num4 => {
                chip8.set_key(0xC, true);
            }
            Keycode::Q => {
                chip8.set_key(0x4, true);
            }
            Keycode::W => {
                chip8.set_key(0x5, true);
            }
            Keycode::E => {
                chip8.set_key(0x6, true);
            }
            Keycode::R => {
                chip8.set_key(0xD, true);
            }
            Keycode::A => {
                chip8.set_key(0x7, true);
            }
            Keycode::S => {
                chip8.set_key(0x8, true);
            }
            Keycode::D => {
                chip8.set_key(0x9, true);
            }
            Keycode::F => {
                chip8.set_key(0xE, true);
            }
            Keycode::Z => {
                chip8.set_key(0xA, true);
            }
            Keycode::X => {
                chip8.set_key(0x0, true);
            }
            Keycode::C => {
                chip8.set_key(0xB, true);
            }
            Keycode::V => {
                chip8.set_key(0xF, true);
            }
            _ => {}
        }
    }

    /// Handles key up event
    pub fn handle_key_up(&mut self, chip8: &mut Chip8, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => {
                chip8.set_key(0x1, false);
            }
            Keycode::Num2 => {
                chip8.set_key(0x2, false);
            }
            Keycode::Num3 => {
                chip8.set_key(0x3, false);
            }
            Keycode::Num4 => {
                chip8.set_key(0xC, false);
            }
            Keycode::Q => {
                chip8.set_key(0x4, false);
            }
            Keycode::W => {
                chip8.set_key(0x5, false);
            }
            Keycode::E => {
                chip8.set_key(0x6, false);
            }
            Keycode::R => {
                chip8.set_key(0xD, false);
            }
            Keycode::A => {
                chip8.set_key(0x7, false);
            }
            Keycode::S => {
                chip8.set_key(0x8, false);
            }
            Keycode::D => {
                chip8.set_key(0x9, false);
            }
            Keycode::F => {
                chip8.set_key(0xE, false);
            }
            Keycode::Z => {
                chip8.set_key(0xA, false);
            }
            Keycode::X => {
                chip8.set_key(0x0, false);
            }
            Keycode::C => {
                chip8.set_key(0xB, false);
            }
            Keycode::V => {
                chip8.set_key(0xF, false);
            }
            _ => {}
        }
    }

    /// Draws the CPU's display to the canvas
    pub fn draw_canvas(&mut self, chip8: &mut Chip8, scale: u32) {
        for i in 0..64 * 32 {
            let current_pixel = chip8.gfx[i];
            let x = (i % 64) * scale as usize;
            let y = (i / 64) * scale as usize;

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            if current_pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            let _ = self
                .canvas
                .fill_rect(Rect::new(x as i32, y as i32, scale, scale));
        }
        self.canvas.present();
    }

    /// Plays a beep sound
    pub fn play_sound(&mut self) {
        self.device.resume();
    }

    /// Stops the beep sound
    pub fn stop_sound(&mut self) {
        self.device.pause();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
