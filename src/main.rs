#![windows_subsystem = "windows"]

mod core;
mod interface;
mod utils;
use crate::core::Chip8;

const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;
const CLOCK_SPEED_HZ: u64 = 600;
const TARGET_FPS: u64 = 60;

const DEBUG: bool = false;

fn main() {
    let settings = utils::parse_yaml_file("settings.yaml");

    let mut chip8 = Chip8::new();
    chip8.initialize();

    let rom_name = match settings.get("rom_name") {
        Some(name) => name,
        None => panic!("No rom name provided!"),
    };

    chip8.load_game(rom_name);

    interface::render(rom_name, chip8);
}
