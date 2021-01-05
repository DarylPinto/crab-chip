mod controls;
use crate::core::Chip8;
use crate::utils;
use crate::CLOCK_SPEED_HZ;
use crate::DEBUG;
use crate::TARGET_FPS;
use crate::VIDEO_HEIGHT;
use crate::VIDEO_WIDTH;
use minifb::{Key, Scale, Window, WindowOptions};
use spin_sleep;
use std::time::Duration;

const CYCLES_PER_FRAME: u64 = CLOCK_SPEED_HZ / TARGET_FPS;

pub fn render(rom_name: &str, mut chip8: Chip8) {
    let opts = WindowOptions {
        scale: Scale::X16,
        ..WindowOptions::default()
    };

    let window_title = format!("{} - Crab Chip", utils::trim_file_ext(rom_name));
    let mut window =
        Window::new(&window_title, VIDEO_WIDTH, VIDEO_HEIGHT, opts).unwrap_or_else(|err| {
            panic!("{}", err);
        });

    let mut framebuffer: Vec<u32> = vec![0; VIDEO_WIDTH * VIDEO_HEIGHT];

    // Keyboard controls
    let keyboard_controls = controls::get_keyboard_layout();

    // Unfortunately, due to cross platfrom differences, thread::sleep appears to
    // be unreliable on Windows, cutting the FPS in half. Because of this we must call
    // spin_sleep at the end of the loop body instead of using minifb's built-in
    // window.limit_update_rate
    window.limit_update_rate(None);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Send keyboard info to emulated keypad
        let held_keys: Vec<bool> = keyboard_controls
            .iter()
            .map(|key| window.is_key_down(*key))
            .collect();

        chip8.set_keys(held_keys);

        // Even though we already have a flag indicating whether to draw
        // within `chip8`, it's clock cycle is too fast for the event loop to pick up.
        // For this reason, we have an independant draw flag here for the event loop
        let mut should_draw = false;

        for _ in 0..CYCLES_PER_FRAME {
            chip8.emulate_cycle();
            if chip8.draw_flag {
                should_draw = true
            }
        }

        if DEBUG {
            utils::clear_terminal();
            println!("{:?}", chip8);
        }

        // Dump video ram data into frame buffer
        if should_draw {
            for (framebuffer_pixel, vram_pixel) in framebuffer.iter_mut().zip(chip8.gfx.iter()) {
                match *vram_pixel {
                    0xFF => *framebuffer_pixel = 0x00_FFFFFF,
                    _ => *framebuffer_pixel = 0x00_000000,
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&framebuffer, VIDEO_WIDTH, VIDEO_HEIGHT)
            .unwrap();

        // Limit to max fps
        spin_sleep::sleep(Duration::from_millis(1000 / TARGET_FPS));
    }
}
