// #![windows_subsystem = "windows"]
mod core;
mod interface;
mod utils;
use crate::core::Chip8;
use std::fs::File;

const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;
const CLOCK_SPEED_HZ: u64 = 600;
const TARGET_FPS: u64 = 60;

const DEBUG: bool = false;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    rom_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Config = serde_yaml::from_reader(File::open("settings.yaml")?)?;

    let mut chip8 = Chip8::new();
    chip8.initialize();

    chip8.load_game(&settings.rom_name)?;

    interface::render(&settings.rom_name, chip8)?;

    Ok(())
}
