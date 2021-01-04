mod core;
mod interface;
mod utils;
use crate::core::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.initialize();

    chip8.load_game("roms/pong.rom");

    interface::render(chip8);
}
