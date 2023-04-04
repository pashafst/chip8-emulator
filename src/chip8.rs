use rand::random;

use crate::register::{IRegister, VRegister};
use crate::stack::Stack;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const MEM_SIZE: usize = 4096;

const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    mem: [u8; MEM_SIZE],
    v_regs: VRegister,
    i_reg: IRegister,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    stack: Stack,
    keypad: [bool; NUM_KEYS],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            mem: [0; MEM_SIZE],
            v_regs: VRegister::new(),
            i_reg: IRegister::new(),
            delay_timer: 0,
            sound_timer: 0,
            pc: START_ADDR,
            keypad: [false; NUM_KEYS],
            stack: Stack::new(),
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        };
        chip8.mem[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        chip8
    }

    pub fn reset(&mut self) {
        self.mem = [0; MEM_SIZE];
        self.v_regs.reset();
        self.i_reg.reset();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.pc = START_ADDR;
        self.stack.reset();
        self.keypad = [false; NUM_KEYS];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.mem[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode and Execute
        self.decode_and_execute(op);
    }

    pub fn timer_tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            // TODO: Give sound
            self.sound_timer -= 1;
        }
    }

    pub fn get_screen(&self) -> &[bool] {
        &self.screen
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keypad[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.mem[start..end].copy_from_slice(data);
    }

    fn fetch(&mut self) -> u16 {
        // TODO
        let high_byte: u16 = self.mem[self.pc as usize] as u16;
        let low_byte: u16 = self.mem[(self.pc + 1) as usize] as u16;
        let op = (high_byte << 8) + low_byte;
        self.pc += 2;
        op
    }

    fn decode_and_execute(&mut self, op: u16) {
        // 0xABCD -> digit0 digit1 digit2 digit3
        let digit0 = (op & 0xF000) >> 12;
        let digit1 = (op & 0x0F00) >> 8;
        let digit2 = (op & 0x00F0) >> 4;
        let digit3 = op & 0x000F;

        // Decode and Execute
        match (digit0, digit1, digit2, digit3) {
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            (0, 0, 0xE, 0xE) => {
                self.pc = self.stack.pop();
            }
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            }
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            (3, _, _, _) => {
                let x = digit1 as usize;
                let kk = (op & 0xFF) as u8;
                if self.v_regs.read(x) == kk {
                    self.pc += 2;
                }
            }
            (4, _, _, _) => {
                let x = digit1 as usize;
                let kk = (op & 0xFF) as u8;
                if self.v_regs.read(x) != kk {
                    self.pc += 2;
                }
            }
            (5, _, _, 0) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                if self.v_regs.read(x) == self.v_regs.read(y) {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => {
                let x = digit1 as usize;
                let kk = (op & 0xFF) as u8;
                self.v_regs.write(x, kk);
            }
            (7, _, _, _) => {
                let x = digit1 as usize;
                let kk = (op & 0xFF) as u8;
                let value = self.v_regs.read(x).wrapping_add(kk);
                self.v_regs.write(x, value);
            }
            (8, _, _, 0) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                self.v_regs.write(x, self.v_regs.read(y));
            }
            (8, _, _, 1) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                let value = self.v_regs.read(x) | self.v_regs.read(y);
                self.v_regs.write(x, value);
            }
            (8, _, _, 2) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                let value = self.v_regs.read(x) & self.v_regs.read(y);
                self.v_regs.write(x, value);
            }
            (8, _, _, 3) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                let value = self.v_regs.read(x) ^ self.v_regs.read(y);
                self.v_regs.write(x, value);
            }
            (8, _, _, 4) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                let (new_vx, carry_bit) = self.v_regs.read(x).overflowing_add(self.v_regs.read(y));
                let new_vf = if carry_bit { 1 } else { 0 };
                self.v_regs.write(x, new_vx);
                self.v_regs.write(0xF, new_vf);
            }
            (8, _, _, 5) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                let (new_vx, carry_bit) = self.v_regs.read(x).overflowing_sub(self.v_regs.read(y));
                let new_vf = if carry_bit { 0 } else { 1 };
                self.v_regs.write(x, new_vx);
                self.v_regs.write(0xF, new_vf);
            }
            (8, _, _, 6) => {
                let x = digit1 as usize;
                let lsb = self.v_regs.read(x) & 1;
                self.v_regs.write(x, self.v_regs.read(x) >> 1);
                self.v_regs.write(0xF, lsb);
            }
            (8, _, _, 7) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                let (new_vx, carry_bit) = self.v_regs.read(y).overflowing_sub(self.v_regs.read(x));
                let new_vf = if carry_bit { 0 } else { 1 };
                self.v_regs.write(x, new_vx);
                self.v_regs.write(0xF, new_vf);
            }
            (8, _, _, 0xE) => {
                let x = digit1 as usize;
                let msb = (self.v_regs.read(x) >> 7) & 1;
                self.v_regs.write(x, self.v_regs.read(x) << 1);
                self.v_regs.write(0xF, msb);
            }
            (9, _, _, 0) => {
                let x = digit1 as usize;
                let y = digit2 as usize;
                if self.v_regs.read(x) != self.v_regs.read(y) {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg.write(nnn);
            }
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_regs.read(0) as u16) + nnn;
            }
            (0xC, _, _, _) => {
                let x = digit1 as usize;
                let kk = (op & 0xFF) as u8;
                let random_num: u8 = random();
                self.v_regs.write(x, random_num & kk);
            }
            // Draw
            (0xD, _, _, _) => {
                // Get coords from v registers
                let x_coord = self.v_regs.read(digit1 as usize) as u16;
                let y_coord = self.v_regs.read(digit2 as usize) as u16;

                // num of rows
                let n = digit3;

                // keep track of collision
                let mut collision: bool = false;

                for y_line in 0..n {
                    let current_addr = self.i_reg.read() + y_line;
                    let pixels = self.mem[current_addr as usize];

                    // Iterate over row; length is 8
                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;
                            collision |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
                self.v_regs.write(0xF, if collision { 1 } else { 0 });
            }
            (0xE, _, 9, 0xE) => {
                let x = digit1 as usize;
                let key: bool = self.keypad[self.v_regs.read(x) as usize];
                if key {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                let x = digit1 as usize;
                let key: bool = self.keypad[self.v_regs.read(x) as usize];
                if !key {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => {
                let x = digit1 as usize;
                self.v_regs.write(x, self.delay_timer);
            }
            (0xF, _, 0, 0xA) => {
                let x = digit1 as usize;
                let mut pressed = false;

                for i in 0..self.keypad.len() {
                    let key = self.keypad[i];
                    if key {
                        self.v_regs.write(x, i as u8);
                        pressed = true;
                        break;
                    }
                }

                // repeat opcode by decreasing it by 2 (reversing fetch process)
                if !pressed {
                    self.pc -= 2;
                }
            }
            (0xF, _, 1, 5) => {
                let x = digit1 as usize;
                self.delay_timer = self.v_regs.read(x);
            }
            (0xF, _, 1, 8) => {
                let x = digit1 as usize;
                self.sound_timer = self.v_regs.read(x);
            }
            (0xF, _, 1, 0xE) => {
                let x = digit1 as usize;
                let vx = self.v_regs.read(x) as u16;
                let value = self.i_reg.read().wrapping_add(vx);
                self.i_reg.write(value);
            }
            (0xF, _, 2, 9) => {
                let x = digit1 as usize;
                let char = self.v_regs.read(x) as u16;
                self.i_reg.write(char * 5);
            }
            (0xF, _, 3, 3) => {
                let x = digit1 as usize;
                let value = self.v_regs.read(x) as f32;

                let hundreds = (value / 100.0).floor() as u8;
                let tens = ((value / 10.0) % 10.0).floor() as u8;
                let ones = (value % 10.0) as u8;
                let idx = self.i_reg.read() as usize;

                self.mem[idx] = hundreds;
                self.mem[idx + 1] = tens;
                self.mem[idx + 2] = ones;
            }
            (0xF, _, 5, 5) => {
                let x = digit1 as usize;
                let i = self.i_reg.read() as usize;
                for idx in 0..=x {
                    self.mem[i + idx] = self.v_regs.read(idx);
                }
            }
            (0xF, _, 6, 5) => {
                let x = digit1 as usize;
                let i = self.i_reg.read() as usize;
                for idx in 0..=x {
                    let value = self.mem[i + idx];
                    self.v_regs.write(idx, value);
                }
            }
            (_, _, _, _) => unimplemented!("Instruction not implemented!"),
        }
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Chip8;

    #[test]
    fn test_new_chip8() {
        let chip8: Chip8 = Chip8::new();
        assert_eq!(chip8.pc, 0x200);
    }

    #[test]
    fn test_push_and_pop() {
        let mut chip8: Chip8 = Chip8::new();
        chip8.stack.push(1);
        chip8.stack.push(2);

        assert_eq!(chip8.stack.get(0), 1);
        assert_eq!(chip8.stack.get(1), 2);

        assert_eq!(chip8.stack.pop(), 2);
        assert_eq!(chip8.stack.pop(), 1);
    }
}
