mod draw;
mod error;
mod fmt_debug;
mod fontset;
use crate::CLOCK_SPEED_HZ;
use crate::VIDEO_HEIGHT;
use crate::VIDEO_WIDTH;
use error::Error;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Read;

const FONTSET_START_ADDRESS: u16 = 0x50;
const PC_START_ADDRESS: u16 = 0x200;

// Chip8 timers decrement at 60hz, even though the clock speed may be higher
const CYCLES_PER_TIMER_DECREMENT: usize = CLOCK_SPEED_HZ as usize / 60;

pub struct Chip8 {
    memory: [u8; 4096],
    // Registers v0, v1 ... vF
    registers: [u8; 16],
    index_register: u16,
    // PC
    program_counter: u16,
    // Video RAM
    pub gfx: [u8; VIDEO_WIDTH * VIDEO_HEIGHT],
    // Timers
    delay_timer: u8,
    sound_timer: u8,
    // Call stack
    stack: [u16; 16],
    stack_pointer: u8,
    // Input
    keypad: [bool; 16],

    /* === Non-standard === */
    pub draw_flag: bool,
    timer_loop: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0,
            gfx: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keypad: [false; 16],
            draw_flag: false,
            timer_loop: 0,
        }
    }
    pub fn initialize(&mut self) {
        self.program_counter = PC_START_ADDRESS;

        // Load fontset into memory
        let font = fontset::get();
        let addr = FONTSET_START_ADDRESS as usize;
        let mem_slice = &mut self.memory[addr..];

        for (mem_byte, font_byte) in mem_slice.iter_mut().zip(font.iter()) {
            *mem_byte = *font_byte;
        }
    }
    pub fn load_game(&mut self, file_name: &str) -> Result<(), io::Error> {
        let file_path = format!("roms/{}", file_name);
        let f = File::open(file_path)?;

        let pc = self.program_counter as usize;
        let mem_slice = &mut self.memory[pc..];

        for (mem_byte, file_byte) in mem_slice.iter_mut().zip(f.bytes()) {
            *mem_byte = file_byte?;
        }

        Ok(())
    }
    pub fn set_keys(&mut self, keypad_state: Vec<bool>) {
        for (key_register, key_state) in self.keypad.iter_mut().zip(keypad_state.iter()) {
            *key_register = *key_state;
        }
    }
    pub fn emulate_cycle(&mut self) -> Result<(), Error> {
        self.draw_flag = false;

        // Usize casted pointers for indexing system memory
        let sp = self.stack_pointer as usize;
        let I = self.index_register as usize;

        // Fetch Opcode
        let pc = self.program_counter;
        let opcode = u16::from_be_bytes([self.memory[pc as usize], self.memory[pc as usize + 1]]);

        // These variables are derived from the opcode in many cases;
        // so much so that it makes sense to extract them here instead of
        // within each match arm
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.registers[x];
        let vy = self.registers[y];

        // Get the first half byte (nibble) to determine the opcode
        self.program_counter = match opcode & 0xF000 {
            // 0 series opcodes
            0x0000 => {
                match opcode & 0x00FF {
                    // 00E0: Clear screen
                    0xE0 => {
                        self.gfx.iter_mut().for_each(|byte| *byte = 0x00);
                        Ok(pc + 2)
                    }
                    // 00EE: Return from subroutine
                    0xEE => {
                        self.stack_pointer -= 1;
                        let sp = self.stack_pointer as usize;
                        Ok(self.stack[sp])
                    }
                    op => Err(Error::UnknownOpcode(op)),
                }
            }
            // 1NNN: Jump to NNN
            0x1000 => Ok(nnn),
            // 2NNN: Call NNN
            0x2000 => {
                // Store pc in the stack and increment sp
                self.stack[sp] = pc;
                self.stack_pointer += 1;

                // Call NNN
                Ok(nnn)
            }
            // 3XNN: Skip next instruction if vX == NN
            0x3000 => {
                // pc is auto-incremented after each cycle (except in special cases)
                // so we can skip the next instruction by manually incrementing it here
                // such that it increments twice
                if vx == nn {
                    Ok(pc + 4)
                } else {
                    Ok(pc + 2)
                }
            }
            // 4XNN: Skip next instruction if vX != NN
            0x4000 => {
                if vx == nn {
                    Ok(pc + 2)
                } else {
                    Ok(pc + 4)
                }
            }
            // 5XY0: Skip next instruction if vX == vY
            0x5000 => {
                if vx == vy {
                    Ok(pc + 4)
                } else {
                    Ok(pc + 2)
                }
            }
            // 6XNN: set VX to NN
            0x6000 => {
                self.registers[x] = nn;
                Ok(pc + 2)
            }
            // 7XNN: Add NN to vX
            0x7000 => {
                self.registers[x] = vx.wrapping_add(nn);
                Ok(pc + 2)
            }
            // 8 series opcodes
            0x8000 => {
                match opcode & 0x000F {
                    // 8XY0: set vX to value of vY
                    0x00 => {
                        self.registers[x] = vy;
                        Ok(pc + 2)
                    }
                    // 8XY1: set vX to vx OR vY
                    0x01 => {
                        self.registers[x] = vx | vy;
                        Ok(pc + 2)
                    }
                    // 8XY2: set vX to (vX & vY)
                    0x02 => {
                        self.registers[x] = vx & vy;
                        Ok(pc + 2)
                    }
                    // 8XY3: set vX to vx XOR vY
                    0x03 => {
                        self.registers[x] = vx ^ vy;
                        Ok(pc + 2)
                    }
                    // 8XY4: Adds VY to VX. V[0xF] is set to 1 when there's a carry, and to 0 when there isn't.
                    0x04 => {
                        let vx = vx as u16;
                        let vy = vy as u16;

                        let u8_max = u8::MAX as u16;

                        let wrapped_result = (vx as u8).wrapping_add(vy as u8);

                        self.registers[x] = wrapped_result;
                        // Carry flag
                        self.registers[0x0F] = ((vx + vy) > u8_max) as u8;
                        Ok(pc + 2)
                    }
                    // 8XY5: Subtract VY from VX. V[0xF] is set to 0 when there's a borrow, and to 1 when there isn't.
                    0x05 => {
                        let vx = vx as u16;
                        let vy = vy as u16;

                        let wrapped_result = (vx as u8).wrapping_sub(vy as u8);

                        self.registers[x] = wrapped_result;
                        self.registers[0x0F] = (vx > vy) as u8;
                        Ok(pc + 2)
                    }
                    // 8XY6: Store the least significant bit of VX in VF, then shift VX to the right by 1
                    0x06 => {
                        let least_significant_bit = vx & 0x01;
                        self.registers[0x0F] = least_significant_bit;

                        self.registers[x] = vx >> 1;
                        Ok(pc + 2)
                    }
                    // BXY7: Set VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                    0x07 => {
                        let vx = vx as u16;
                        let vy = vy as u16;

                        let wrapped_result = (vy as u8).wrapping_sub(vx as u8);

                        self.registers[x] = wrapped_result;
                        self.registers[0x0F] = (vy > vx) as u8;
                        Ok(pc + 2)
                    }
                    // 8XYE: Store the most significant bit of VX in VF, then shift VX to the left by 1
                    0x0E => {
                        // 0x80 is b10000000, so using AND and shifting right by 7
                        // gets us the most significant bit
                        let most_signficant_bit = (vx & 0x80) >> 7;
                        self.registers[0x0F] = most_signficant_bit;

                        self.registers[x] = vx << 1;
                        Ok(pc + 2)
                    }
                    op => Err(Error::UnknownOpcode(op)),
                }
            }
            // 9XY0: Skip next instruction if vX != vY
            0x9000 => {
                if vx != vy {
                    Ok(pc + 4)
                } else {
                    Ok(pc + 2)
                }
            }
            // ANNN: set index_register to NNN
            0xA000 => {
                self.index_register = nnn;
                Ok(pc + 2)
            }
            // BNNN: Jump to NNN + v0
            0xB000 => {
                let v0 = self.registers[0] as u16;
                Ok(nnn + v0)
            }
            // CXNN: set vX to a random u8 & NN (bitwise &)
            0xC000 => {
                let random: u8 = rand::thread_rng().gen();
                self.registers[x] = random & nn;
                Ok(pc + 2)
            }
            // DXYN: draw to the display
            0xD000 => {
                draw::dxyn(self, vx, vy, n, I);
                Ok(pc + 2)
            }
            // E series opcodes
            0xE000 => {
                let vx = vx as usize;
                match opcode & 0x00FF {
                    // EX9E: Skip next instruction if key in vX is pressed
                    0x9E => {
                        if self.keypad[vx] {
                            Ok(pc + 4)
                        } else {
                            Ok(pc + 2)
                        }
                    }
                    // EXA1: Skip next instruction if key in vX is NOT pressed
                    0xA1 => {
                        if !self.keypad[vx] {
                            Ok(pc + 4)
                        } else {
                            Ok(pc + 2)
                        }
                    }
                    op => return Err(Error::UnknownOpcode(op)),
                }
            }
            // F series opcodes
            0xF000 => {
                match opcode & 0x00FF {
                    // Fx15: Set vX to value of delay timer
                    0x07 => {
                        self.registers[x] = self.delay_timer;
                        Ok(pc + 2)
                    }
                    // Fx0A: wait for keypress, store in vX
                    0x0A => {
                        let mut any_key_pressed = false;

                        for key in 0..0x0F {
                            if self.keypad[key] {
                                self.registers[x] = key as u8;
                                any_key_pressed = true;
                            }
                        }

                        if any_key_pressed {
                            Ok(pc + 2)
                        } else {
                            Ok(pc)
                        }
                    }
                    // Fx15: Set delay timer to vX
                    0x15 => {
                        self.delay_timer = vx;
                        Ok(pc + 2)
                    }
                    // Fx18: Set sound timer to vX
                    0x18 => {
                        self.sound_timer = vx;
                        Ok(pc + 2)
                    }
                    // Fx1E: Adds VX to I. VF is not affected
                    0x1E => {
                        self.index_register += vx as u16;
                        Ok(pc + 2)
                    }
                    // Fx29: Set I to the location of the sprite for the character in vX
                    // Fontset should already be loaded in memory at 0x50
                    0x29 => {
                        let vx = vx as u16;
                        let font_sprite_address = FONTSET_START_ADDRESS + (vx * 5);
                        self.index_register = font_sprite_address;
                        Ok(pc + 2)
                    }
                    // Fx33 (hard to explain, check wikipedia)
                    0x33 => {
                        let hundreds = (vx / 100) % 10;
                        let tens = (vx / 10) % 10;
                        let ones = vx % 10;

                        self.memory[I] = hundreds;
                        self.memory[I + 1] = tens;
                        self.memory[I + 2] = ones;
                        Ok(pc + 2)
                    }
                    // Fx55: Store v0 to vX (including vX) in memory starting at I
                    0x55 => {
                        self.memory[I..=(x + I)].copy_from_slice(&self.registers[..=x]);
                        Ok(pc + 2)
                    }
                    // Fx65: Fill v0 to vX (including vX) with mem values starting from I
                    0x65 => {
                        self.registers[..=x].copy_from_slice(&self.memory[I..=(x + I)]);
                        Ok(pc + 2)
                    }
                    op => Err(Error::UnknownOpcode(op)),
                }
            }
            op => Err(Error::UnknownOpcode(op)),
        }?;

        // Because timers on chip8 only update 60 times/sec, but the clock speed in this
        // emulator is configurable, we can determine exactly when the timers should
        // decrement with a bit of math
        self.timer_loop = (self.timer_loop + 1) % CYCLES_PER_TIMER_DECREMENT as u16;

        // Update timers
        if self.timer_loop == 0 {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                if self.sound_timer == 1 {
                    println!("BEEP!");
                }
                self.sound_timer -= 1;
            }
        }

        Ok(())
    }
}
