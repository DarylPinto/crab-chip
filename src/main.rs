mod chip8;
use chip8::Chip8;

fn main() {
    // Set up render system and register input callbacks
    // setupGraphics();
    // setupInput();

    // Initialize the Chip8 system and load the game into the memory
    let chip8 = Chip8::new();
    chip8.loadGame("pong");

    loop {
        // Emulate one cycle
        chip8.emulateCycle();

        // If the draw flag is set, update the screen
        if chip8.draw_flag {
            // drawGraphics();
        }

        // Store key press state (Press and Release)
        chip8.setKeys();
    }

    println!("Hello, world #{}!", 0xFF);
}
