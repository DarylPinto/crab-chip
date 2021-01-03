use rand::Rng;
use std::fs::File;
use std::io::{self, Read};
mod fmt_debug;
mod fontset;
mod operations;

const FONTSET_START_ADDRESS: u16 = 0x50;
const PC_START_ADDRESS: u16 = 0x200;

pub struct Chip8 {
    /* === CPU ===*/
    opcode: u16,
    memory: [u8; 4096],

    // registers v0, v1 ... vE
    registers: [u8; 16],

    index_register: u16,
    program_counter: u16,

    // Screen graphics (64 x 32 px)
    gfx: [u8; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stack_pointer: u8,

    keypad: [u8; 16],

    /* === FLAGS === */
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            opcode: 0x0000,
            memory: [0x00; 4096],
            registers: [0x00; 16],
            index_register: 0x00,
            program_counter: 0x00,
            gfx: [0x00; 64 * 32],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: [0x00; 16],
            stack_pointer: 0x00,
            // keypad: (0x0u8..0xEu8).collect::<[u8; 16]>()
            keypad: [0x00; 16],
            draw_flag: false,
        }
    }
    pub fn initialize(&mut self) {
        self.program_counter = PC_START_ADDRESS;

        // Clear display
        // Clear stack
        // Clear registers V0-VF
        // Clear memory

        // Load fontset into memory
        let font = fontset::get();
        let addr = FONTSET_START_ADDRESS as usize;
        let mem_slice = &mut self.memory[addr..];

        for (mem_byte, font_byte) in mem_slice.iter_mut().zip(font.iter()) {
            *mem_byte = *font_byte;
        }

        // Reset timers
    }
    pub fn load_game(&mut self, file_name: &str) -> io::Result<()> {
        let f = File::open(file_name)?;
        // TODO: Handle error for file loading

        let pc = self.program_counter as usize;
        let mem_slice = &mut self.memory[pc..];

        for (mem_byte, file_byte) in mem_slice.iter_mut().zip(f.bytes()) {
            *mem_byte = file_byte.unwrap();
        }

        Ok(())
    }
    pub fn set_keys(&self) {}
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
                    _ => {
                        panic!("Unknown CHIP-8 0-series opcode: {:#06x?}", self.opcode);
                    }
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
            // 6XNN: set VX to NN
            0x6000 => {
                self.registers[x] = nn;
            }
            // 7XNN: Add NN to vX
            0x7000 => {
                self.registers[x] += nn;
            }
            // ANNN: set index_register to NNN
            0xA000 => {
                self.index_register = nnn;
            }
            // CXNN: set vX to a random u8 & NN (bitwise &)
            0xC000 => {
                let random: u8 = rand::thread_rng().gen();
                self.registers[x] = random & nn;
            }
            // DXYN: draw to the display
            0xD000 => operations::dxyn(self.opcode, self),
            // F series opcodes
            0xF000 => {
                match self.opcode & 0x00FF {
                    // Fx15: Set vX to value of delay timer
                    0x07 => {
                        self.registers[x] = self.delay_timer;
                    }
                    // Fx15: Set delay timer to vX
                    0x15 => {
                        self.delay_timer = vx;
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
                    // Fx65: Fill v0 to vX (including vX) with mem values starting from I
                    0x65 => {
                        for offset in 0..x + 1 {
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

        // Update timers
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
