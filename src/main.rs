use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use chip8::chip8::*;
use chip8::drivers::audio::AudioDriver;

use chip8::drivers::input::InputDriver;
use chip8::drivers::video::VideoDriver;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const TICKS_PER_LOOP: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} path/to/game", &args[0]);
        return;
    }

    let sdl_context = sdl2::init().unwrap();

    let mut video_driver = VideoDriver::new(&sdl_context);
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new();

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
                    input_driver.poll_key(key);
                    if let Some(k) = input_driver.get_key_pressed() {
                        chip8_emu.keypress(k, true);
                    }
                    //if let Some(k) = keyboard_to_chip8(key) {
                    //    chip8_emu.keypress(k, true);
                    //}
                }

                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    input_driver.poll_key(key);
                    if let Some(k) = input_driver.get_key_pressed() {
                        chip8_emu.keypress(k, false);
                    }
                    // if let Some(k) = keyboard_to_chip8(key) {
                    //     chip8_emu.keypress(k, false);
                    // }
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
            audio_driver.play_sound();
        } else {
            // stop sound
            audio_driver.stop_sound();
        }

        let screen = chip8_emu.get_screen();
        video_driver.draw_screen(screen);
    }
}
