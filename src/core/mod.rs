use rand::Rng;
use std::fs::File;
use std::io::Read;
mod draw;
mod fmt_debug;
mod fontset;
use crate::CLOCK_SPEED_HZ;
use crate::VIDEO_HEIGHT;
use crate::VIDEO_WIDTH;

const FONTSET_START_ADDRESS: u16 = 0x50;
const PC_START_ADDRESS: u16 = 0x200;

// Chip8 timers decrement at 60hz, even though the clock speed may be higher
const CYCLES_PER_TIMER_DECREMENT: usize = CLOCK_SPEED_HZ as usize / 60;

pub struct Chip8 {
    opcode: u16,
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
            opcode: 0x0000,
            memory: [0x00; 4096],
            registers: [0x00; 16],
            index_register: 0x00,
            program_counter: 0x00,
            gfx: [0x00; VIDEO_WIDTH * VIDEO_HEIGHT],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: [0x00; 16],
            stack_pointer: 0x00,
            keypad: [false; 16],
            draw_flag: false,
            timer_loop: 0x0000,
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
    pub fn load_game(&mut self, file_name: &str) {
        let file_path = format!("roms/{}", file_name);
        let f = match File::open(file_path) {
            Err(msg) => panic!("couldn't open {}: {}", file_name, msg),
            Ok(file) => file,
        };

        let pc = self.program_counter as usize;
        let mem_slice = &mut self.memory[pc..];

        for (mem_byte, file_byte) in mem_slice.iter_mut().zip(f.bytes()) {
            *mem_byte = file_byte.unwrap();
        }
    }
    pub fn set_keys(&mut self, keypad_state: Vec<bool>) {
        for (key_register, key_state) in self.keypad.iter_mut().zip(keypad_state.iter()) {
            *key_register = *key_state;
        }
    }
    pub fn emulate_cycle(&mut self) {
        let mut pc_should_increment = true;

        // Usize casted pointers for indexing system memory
        let pc = self.program_counter as usize;
        let sp = self.stack_pointer as usize;
        let I = self.index_register as usize;

        // Fetch Opcode
        self.opcode = u16::from_be_bytes([self.memory[pc], self.memory[pc + 1]]);

        // These variables are derived from the opcode in many cases;
        // so much so that it makes sense to extract them here instead of
        // within each match arm
        let nnn = self.opcode & 0x0FFF;
        let nn = (self.opcode & 0x00FF) as u8;
        let n = (self.opcode & 0x000F) as u8;
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        let vx = self.registers[x];
        let vy = self.registers[y];

        // Get the first half byte (nibble) to determine the opcode
        match self.opcode & 0xF000 {
            // 0 series opcodes
            0x0000 => {
                match self.opcode & 0x00FF {
                    // 00E0: Clear screen
                    0xE0 => self.gfx.iter_mut().for_each(|byte| *byte = 0x00),
                    // 00EE: Return from subroutine
                    0xEE => {
                        self.stack_pointer -= 1;
                        let sp = self.stack_pointer as usize;
                        self.program_counter = self.stack[sp];
                    }
                    _ => panic!("Unknown CHIP-8 0-series opcode: {:#06x?}", self.opcode),
                }
            }
            // 1NNN: Jump to NNN
            0x1000 => {
                self.program_counter = nnn;
                pc_should_increment = false;
            }
            // 2NNN: Call NNN
            0x2000 => {
                // Store pc in the stack and increment sp
                self.stack[sp] = self.program_counter;
                self.stack_pointer += 1;

                // Call NNN
                self.program_counter = nnn;
                pc_should_increment = false;
            }
            // 3XNN: Skip next instruction if vX == NN
            0x3000 => {
                // pc is auto-incremented after each cycle (except in special cases)
                // so we can skip the next instruction by manually incrementing it here
                // such that it increments twice
                if vx == nn {
                    self.program_counter += 2;
                }
            }
            // 4XNN: Skip next instruction if vX != NN
            0x4000 => {
                if vx != nn {
                    self.program_counter += 2;
                }
            }
            // 5XY0: Skip next instruction if vX == vY
            0x5000 => {
                if vx == vy {
                    self.program_counter += 2;
                }
            }
            // 6XNN: set VX to NN
            0x6000 => {
                self.registers[x] = nn;
            }
            // 7XNN: Add NN to vX
            0x7000 => {
                self.registers[x] = vx.wrapping_add(nn);
            }
            // 8 series opcodes
            0x8000 => {
                match self.opcode & 0x000F {
                    // 8XY0: set vX to value of vY
                    0x00 => {
                        self.registers[x] = vy;
                    }
                    // 8XY1: set vX to vx OR vY
                    0x01 => {
                        self.registers[x] = vx | vy;
                    }
                    // 8XY2: set vX to (vX & vY)
                    0x02 => {
                        self.registers[x] = vx & vy;
                    }
                    // 8XY3: set vX to vx XOR vY
                    0x03 => {
                        self.registers[x] = vx ^ vy;
                    }
                    // 8XY4: Adds VY to VX. V[0xF] is set to 1 when there's a carry, and to 0 when there isn't.
                    0x04 => {
                        let vx = vx as u16;
                        let vy = vy as u16;

                        let u8_max = u8::MAX as u16;

                        let result = vx + vy;
                        let wrapped_result = (vx as u8).wrapping_add(vy as u8);
                        let carried = result > u8_max;

                        self.registers[x] = wrapped_result;
                        self.registers[0x0F] = match carried {
                            true => 0x01,
                            false => 0x00,
                        };
                    }
                    // 8XY5: Subtract VY from VX. V[0xF] is set to 0 when there's a borrow, and to 1 when there isn't.
                    0x05 => {
                        let vx = vx as u16;
                        let vy = vy as u16;

                        let wrapped_result = (vx as u8).wrapping_sub(vy as u8);

                        self.registers[x] = wrapped_result;
                        self.registers[0x0F] = match vx > vy {
                            true => 0x01,
                            false => 0x00,
                        };
                    }
                    // 8XY6: Store the least significant bit of VX in VF, then shift VX to the right by 1
                    0x06 => {
                        let least_significant_bit = vx & 0x01;
                        self.registers[0x0F] = least_significant_bit;

                        self.registers[x] = vx >> 1;
                    }
                    // BXY7: Set VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                    0x07 => {
                        let vx = vx as u16;
                        let vy = vy as u16;

                        let wrapped_result = (vy as u8).wrapping_sub(vx as u8);

                        self.registers[x] = wrapped_result;
                        self.registers[0x0F] = match vy > vx {
                            true => 0x01,
                            false => 0x00,
                        };
                    }
                    // 8XYE: Store the most significant bit of VX in VF, then shift VX to the left by 1
                    0x0E => {
                        // 0x80 is b10000000, so using AND and shifting right by 7
                        // gets us the most significant bit
                        let most_signficant_bit = (vx & 0x80) >> 7;
                        self.registers[0x0F] = most_signficant_bit;

                        self.registers[x] = vx << 1;
                    }
                    _ => panic!("Unknown CHIP-8 8-series opcode: {:#06x?}", self.opcode),
                }
            }
            // 9XY0: Skip next instruction if vX != vY
            0x9000 => {
                if vx != vy {
                    self.program_counter += 2;
                }
            }
            // ANNN: set index_register to NNN
            0xA000 => {
                self.index_register = nnn;
            }
            // BNNN: Jump to NNN + v0
            0xB000 => {
                let v0 = self.registers[0] as u16;
                self.program_counter = nnn + v0;
                pc_should_increment = false;
            }
            // CXNN: set vX to a random u8 & NN (bitwise &)
            0xC000 => {
                let random: u8 = rand::thread_rng().gen();
                self.registers[x] = random & nn;
            }
            // DXYN: draw to the display
            0xD000 => draw::dxyn(self, vx, vy, n, I),
            // E series opcodes
            0xE000 => {
                let vx = vx as usize;
                match self.opcode & 0x00FF {
                    // EX9E: Skip next instruction if key in vX is pressed
                    0x9E => {
                        if self.keypad[vx] {
                            self.program_counter += 2;
                        }
                    }
                    // EXA1: Skip next instruction if key in vX is NOT pressed
                    0xA1 => {
                        if !self.keypad[vx] {
                            self.program_counter += 2;
                        }
                    }
                    _ => panic!("Unknown CHIP-8 E-series opcode: {:#06x?}", self.opcode),
                }
            }
            // F series opcodes
            0xF000 => {
                match self.opcode & 0x00FF {
                    // Fx15: Set vX to value of delay timer
                    0x07 => {
                        self.registers[x] = self.delay_timer;
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

                        if !any_key_pressed {
                            pc_should_increment = false;
                        }
                    }
                    // Fx15: Set delay timer to vX
                    0x15 => {
                        self.delay_timer = vx;
                    }
                    // Fx18: Set sound timer to vX
                    0x18 => {
                        self.sound_timer = vx;
                    }
                    // Fx1E: Adds VX to I. VF is not affected
                    0x1E => {
                        self.index_register += vx as u16;
                    }
                    // Fx29: Set I to the location of the sprite for the character in vX
                    // Fontset should already be loaded in memory at 0x50
                    0x29 => {
                        let vx = vx as u16;
                        let font_sprite_address = FONTSET_START_ADDRESS + (vx * 5);
                        self.index_register = font_sprite_address;
                    }
                    // Fx33 (hard to explain, check wikipedia)
                    0x33 => {
                        let hundreds = (vx / 100) % 10;
                        let tens = (vx / 10) % 10;
                        let ones = vx % 10;

                        self.memory[I] = hundreds;
                        self.memory[I + 1] = tens;
                        self.memory[I + 2] = ones;
                    }
                    // Fx55: Store v0 to vX (including vX) in memory starting at I
                    0x55 => {
                        for offset in 0..=x {
                            self.memory[I + offset] = self.registers[offset];
                        }
                    }
                    // Fx65: Fill v0 to vX (including vX) with mem values starting from I
                    0x65 => {
                        for offset in 0..=x {
                            self.registers[offset] = self.memory[I + offset];
                        }
                    }
                    _ => panic!("Unknown CHIP-8 F-series opcode: {:#06x?}", self.opcode),
                }
            }
            _ => panic!("Unknown CHIP-8 opcode: {:#06x?}", self.opcode),
        }

        // Move pc 2 bytes to next opcode (unless current opcode has prevented it)
        if pc_should_increment {
            self.program_counter += 2;
        }

        // Because timers on chip8 only update 60 times/sec, but the clock speed in this
        // emulator is configurable, we can determine exactly when the timers should
        // decrement with a bit of math
        self.timer_loop = (self.timer_loop + 1) % CYCLES_PER_TIMER_DECREMENT as u16;
        let should_timer_update = self.timer_loop == 0;

        // Update timers
        if should_timer_update {
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
    }
}
