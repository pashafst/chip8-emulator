use sdl2::keyboard::Keycode;

pub struct InputDriver {
    key_pressed: Option<usize>,
}

impl InputDriver {
    pub fn new() -> Self {
        InputDriver { key_pressed: None }
    }

/*
 *      keyboard                chip8
 *      +---+---+---+---+       +---+---+---+---+
 *      | 1 | 2 | 3 | 4 |       | 1 | 2 | 3 | C |
 *      +---+---+---+---+       +---+---+---+---+
 *      | Q | W | E | R |       | 4 | 5 | 6 | D |
 *      +---+---+---+---+       +---+---+---+---+
 *      | A | S | D | F |       | 7 | 8 | 9 | E |
 *      +---+---+---+---+       +---+---+---+---+
 *      | Y | X | C | V |       | A | 0 | B | F |
 *      +---+---+---+---+       +---+---+---+---+
*/
    pub fn poll_key(&mut self, key: Keycode) {
        self.key_pressed = match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Y => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        };
    }

    pub fn get_key_pressed(&self) -> Option<usize> {
        self.key_pressed
    }
}

impl Default for InputDriver {
    fn default() -> Self {
        Self::new()
    }
}
