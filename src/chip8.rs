#[derive(Debug)]
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

    stack: Stack,
    stack_pointer: u16,

    keypad: [u8; 16],

    /* === FLAGS === */
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        // TODO: Return initialized Cpu here
        Chip8 {
            opcode: 0x0000,
            memory: [0x00; 4096],
            registers: [0x00; 16],
            index_register: 0x00,
            program_counter: 0x00,
            gfx: [0x00; 64 * 32],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: Stack::new(),
            stack_pointer: 0x00,
            // keypad: (0x0u8..0xEu8).collect::<[u8; 16]>()
            keypad: [
                0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
            ],
            draw_flag: false,
        }
    }
    pub fn loadGame(&self, file_name: &str) {}
    pub fn setKeys(&self) {}
    pub fn emulateCycle(&self) {
        // Fetch Opcode

        // Decode Opcode
        // Execute Opcode
        
        // Update timers
    }
}

#[derive(Debug)]
struct Stack {
    levels: Vec<u16>,
}

impl Stack {
    fn new() -> Self {
        Stack { levels: Vec::new() }
    }
    // Consider initializing with Vec::with_capacity(16) instead
    fn push(&mut self, val: u16) {
        if self.levels.len() < 16 {
            self.levels.push(val);
        } else {
            eprintln!("CPU Stack is full!");
        }
    }
}
