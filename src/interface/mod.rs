mod controls;
use crate::core::Chip8;
use crate::utils;
use crate::CLOCK_SPEED_HZ;
use crate::TARGET_FPS;
use crate::VIDEO_HEIGHT;
use crate::VIDEO_WIDTH;
use minifb::{Key, Scale, Window, WindowOptions};
use std::time::Duration;

const CYCLES_PER_FRAME: u64 = CLOCK_SPEED_HZ / TARGET_FPS;

pub fn render(mut chip8: Chip8) {
    let opts = WindowOptions {
        scale: Scale::X16,
        ..WindowOptions::default()
    };

    let mut window = Window::new("Chip8", VIDEO_WIDTH, VIDEO_HEIGHT, opts).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    let mut framebuffer: Vec<u32> = vec![0; VIDEO_WIDTH * VIDEO_HEIGHT];

    // Keyboard controls
    let keyboard_controls = controls::get_keyboard_layout();

    // Limit to max fps
    window.limit_update_rate(Some(Duration::from_millis(1000 / TARGET_FPS)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let held_keys: Vec<bool> = keyboard_controls
            .iter()
            .map(|key| window.is_key_down(*key))
            .collect();

        chip8.set_keys(held_keys);
        for _ in 0..CYCLES_PER_FRAME {
            chip8.emulate_cycle();
        }

        utils::clear_screen();
        println!("{:?}", chip8);

        // Dump video ram data into frame buffer
        for (framebuffer_pixel, vram_pixel) in framebuffer.iter_mut().zip(chip8.gfx.iter()) {
            match *vram_pixel {
                0xFF => *framebuffer_pixel = 0x00_FFFFFF,
                _ => *framebuffer_pixel = 0x00_000000,
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&framebuffer, VIDEO_WIDTH, VIDEO_HEIGHT)
            .unwrap();
    }
}
