use minifb::Key;

/**
 *  Controls are mapped as such on a standard
 *  QWERTY keyboard:
 *
 *  Chip8                    Computer
 *  Hex Keypad               Keyboard
 *  +-+-+-+-+                +-+-+-+-+
 *  |1|2|3|C|                |1|2|3|4|
 *  +-+-+-+-+                +-+-+-+-+
 *  |4|5|6|D|                |Q|W|E|R|
 *  +-+-+-+-+       =>       +-+-+-+-+
 *  |7|8|9|E|                |A|S|D|F|
 *  +-+-+-+-+                +-+-+-+-+
 *  |A|0|B|F|                |Z|X|C|V|
 *  +-+-+-+-+                +-+-+-+-+
 */

pub fn get_keyboard_layout() -> [Key; 16] {
    [
        Key::X,    // 0
        Key::Key1, // 1
        Key::Key2, // 2
        Key::Key3, // 3
        Key::Q,    // 4
        Key::W,    // 5
        Key::E,    // 6
        Key::A,    // 7
        Key::S,    // 8
        Key::D,    // 9
        Key::Z,    // A
        Key::C,    // B
        Key::Key4, // C
        Key::R,    // D
        Key::F,    // E
        Key::V,    // F
    ]
}
