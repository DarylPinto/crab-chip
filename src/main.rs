mod chip8;
use chip8::Chip8;

fn main() {
    // Set up render system and register input callbacks
    // setupGraphics();
    // setupInput();

    // Initialize the Chip8 system and load the game into the memory
    let mut chip8 = Chip8::new();

    chip8.initialize();
    chip8.load_game("roms/pong.rom");

    let mut i = 0;
    loop {
        // Emulate one cycle
        i += 1;
        println!("\nCYCLE {}:", i);
        chip8.emulate_cycle();
        println!("{}", chip8);

        // If the draw flag is set, update the screen
        // if chip8.draw_flag {
        //     drawGraphics();
        // }

        // Store key press state (Press and Release)
        // chip8.set_keys();
    }
}
