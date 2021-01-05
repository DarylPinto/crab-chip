mod core;
mod interface;
mod utils;
use crate::core::Chip8;

const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;
const CLOCK_SPEED_HZ: u64 = 600;
const TARGET_FPS: u64 = 60;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.initialize();

    chip8.load_game("roms/pong.rom");

    interface::render(chip8);
}
