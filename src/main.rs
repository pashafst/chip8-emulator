use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use chip8::audio::SquareWave;
use chip8::chip8::*;

use sdl2::audio::AudioSpecDesired;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_LOOP: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} path/to/game", &args[0]);
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_audiospec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    let device = audio_subsystem
        .open_playback(None, &desired_audiospec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 480.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    let window = video_subsystem
        .window("CHIP8 EMULATOR", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8_emu = Chip8::new();

    let mut program = File::open(&args[1]).expect("Unable to open file.");
    let mut buffer: Vec<u8> = Vec::new();
    program.read_to_end(&mut buffer).unwrap();
    chip8_emu.load(&buffer);

    let mut timer = Instant::now();
    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'gameloop,

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keyboard_to_chip8(key) {
                        chip8_emu.keypress(k, true);
                    }
                }

                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keyboard_to_chip8(key) {
                        chip8_emu.keypress(k, false);
                    }
                }

                _ => {}
            }
        }

        for _ in 0..TICKS_PER_LOOP {
            chip8_emu.tick();
        }

        // Timer tick every 60Hz
        if timer.elapsed() >= Duration::from_secs_f32(0.01667) {
            chip8_emu.timer_tick();
            timer = Instant::now();
        }

        let sound_timer = chip8_emu.get_sound_timer();
        if sound_timer > 0 {
            // play sound
            device.resume();
        } else {
            // stop sound
            device.pause();
        }

        draw_screen(&chip8_emu, &mut canvas);
    }
}

fn draw_screen(chip8: &Chip8, canvas: &mut WindowCanvas) {
    // Black background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let chip8_screen = chip8.get_screen();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (idx, pixel) in chip8_screen.iter().enumerate() {
        if *pixel {
            let x = (idx % SCREEN_WIDTH) as u32;
            let y = (idx / SCREEN_WIDTH) as u32;
            let rect: Rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

/*
 *      keyboard                chip8
 *      +---+---+---+---+       +---+---+---+---+
 *      | 1 | 2 | 3 | 4 |       | 1 | 2 | 3 | C |
 *      +---+---+---+---+       +---+---+---+---+
 *      | Q | W | E | R |       | 4 | 5 | 6 | D |
 *      +---+---+---+---+       +---+---+---+---+
 *      | A | S | D | F |       | 7 | 8 | 9 | E |
 *      +---+---+---+---+       +---+---+---+---+
 *      | Y | X | C | V |       | A | 0 | B | F |
 *      +---+---+---+---+       +---+---+---+---+
*/

fn keyboard_to_chip8(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Y => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
