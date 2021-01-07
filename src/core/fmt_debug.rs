use crate::Chip8;

impl std::fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Chip8 {{
\tV Registers: {registers:02x?}
\tIndex Register: {index_register:#06x?}
\tMemory val at Index Register: {mem_at_idx:#04x?}
\tProgram Counter: {program_counter:#06x?}
\tCall Stack: {stack:04x?}
\tStack Pointer: {stack_pointer}
\tDelay Timer: {delay_timer:#04x?}
\tSound Timer: {sound_timer:#04x?}
\tCurrently Pressed Keys: {pressed_keys:02x?}
}}",
            registers = self.registers,
            index_register = self.index_register,
            mem_at_idx = self.memory[self.index_register as usize],
            program_counter = self.program_counter,
            stack = self.stack,
            stack_pointer = self.stack_pointer,
            delay_timer = self.delay_timer,
            sound_timer = self.sound_timer,
            pressed_keys = self
                .keypad
                .iter()
                .enumerate()
                .map(|(key, is_pressed)| if *is_pressed { key } else { 0xFF })
                .filter(|key| *key as u8 != 0xFF)
                .collect::<Vec<usize>>(),
        )
    }
}
