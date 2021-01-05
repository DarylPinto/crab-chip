use crate::Chip8;
use crate::VIDEO_HEIGHT;
use crate::VIDEO_WIDTH;

const SPRITE_WIDTH: u8 = 8;

/**
 * Draw sprites to Chip8 VRAM
 */
pub fn dxyn(chip8: &mut Chip8, vx: u8, vy: u8, n: u8, I: usize) {
    let height = n;

    // Wrap if going beyond screen boundaries
    let vy = vy as usize;
    let vx = vx as usize;

    // Set VF to 0
    chip8.registers[0xF] = 0;

    // For each row of the sprite...
    for row in 0..height {
        let row = row as usize;

        // Byte at `I` register will be drawn to the screen bit by bit
        let sprite_byte = chip8.memory[I + row];

        // For each pixel in the row of the sprite...
        for col in 0..SPRITE_WIDTH {
            let col = col as usize;
            // Get the sprite pixel by looking at a specific bit of the sprite byte
            let sprite_pixel = sprite_byte & (0x80 >> col);
            // Get the screen pixel
            let mut screen_pixel_index = (vy + row) * VIDEO_WIDTH + (vx + col);
            screen_pixel_index = screen_pixel_index % (VIDEO_WIDTH * VIDEO_HEIGHT); // prevent index out of bounds
            let screen_pixel = &mut chip8.gfx[screen_pixel_index];

            // If sprite pixel is on
            if sprite_pixel > 0 {
                // And screen pixel also on - collision. Write to vF
                if *screen_pixel == 0xFF {
                    chip8.registers[0xF] = 0x01;
                }

                // XOR with the sprite pixel
                *screen_pixel ^= 0xFF;
            }
        }
    }

    chip8.draw_flag = true;
}
