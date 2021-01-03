use crate::Chip8;

/**
 * Draws sprites to Chip8 display
 */
pub fn dxyn(opcode: u16, chip8: &mut Chip8) {
    let bytes = opcode.to_be_bytes();
    let x = bytes[0] & 0x0F;
    let y = (bytes[1] & 0xF0) >> 4;

    let vx = chip8.registers[x as usize];
    let vy = chip8.registers[y as usize];
    let height = (bytes[1] & 0x0F) as usize;

    // Set VF to 0
    chip8.registers[0xF] = 0x00;

    // For each row of the sprite...
    for row in 0..height {
        // Byte at the I register will be drawn to the screen
        // bit by bit
        let I = chip8.index_register as usize;
        let sprite_byte = chip8.memory[I + row];

        // For each pixel in the row of the sprite...
        for col in 0..8 {
            // Get the sprite pixel by looking at a specific bit of the sprite byte
            let sprite_pixel = sprite_byte & (0x80 >> col);
            // Get the screen pixel
            let screen_pixel_index = (vy as i32 + row as i32) * 64 + (vx as i32 + col as i32);
            let screen_pixel = &mut chip8.gfx[screen_pixel_index as usize];

            // If Sprite pixel is on
            if sprite_pixel > 0 {
                // Screen pixel also on - collision
                if *screen_pixel == 0xFF {
                    chip8.registers[0xF] = 0x01;
                }

                // Effectively XOR with the sprite pixel
                *screen_pixel ^= 0xFF;
            }
        }
    }

    chip8.draw_flag = true;
    chip8.program_counter += 2;
}
