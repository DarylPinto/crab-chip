pub fn clear_screen() {
	print!("\x1B[2J\x1B[1;1H");
}