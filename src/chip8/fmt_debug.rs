use crate::Chip8;

impl std::fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chip8 {{");

        writeln!(f, "\tOp Code: {:#06x?}", self.opcode);
        writeln!(f, "\tV Registers: {:02x?}", self.registers);

        writeln!(f, "\tIndex Register: {:#06x?}", self.index_register);
        writeln!(
            f,
            "\tMemory val at Index Register: {:#04x?}",
            self.memory[self.index_register as usize]
        );

        writeln!(f, "\tProgram Counter: {:#06x?}", self.program_counter);
        // writeln!(f, "\tNext 10 bytes of memory at PC: {:02x?}", &self.memory[self.program_counter as usize..(self.program_counter + 10) as usize]);

        writeln!(f, "\tCall Stack: {:04x?}", self.stack);
        writeln!(f, "\tStack Pointer: {}", self.stack_pointer);

        writeln!(f, "\tDelay Timer: {:#04x?}", self.delay_timer);
        writeln!(f, "\tSound Timer: {:#04x?}", self.sound_timer);

        // writeln!(f, "\tMemory: {:x?}", self.memory);

        writeln!(f, "\tKeypad: {:02x?}", self.keypad);

        writeln!(f, "\tScreen:");
        for chunk in self.gfx.chunks(64) {
            let line: String = chunk
                .iter()
                .map(|b| if *b == 0 { ' ' } else { '▇' })
                .collect();

            writeln!(f, "\t{:?}", line);
        }

        writeln!(f, "}}")
    }
}
