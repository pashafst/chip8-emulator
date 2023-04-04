const STACK_SIZE: usize = 16;

pub struct Stack {
    stack: [u16; STACK_SIZE],
    sp: u8,
}

impl Stack {
    pub fn new() -> Self {
        let stack: Stack = Stack {
            stack: [0; STACK_SIZE],
            sp: 0,
        };
        stack
    }
    pub fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn get(&self, idx: usize) -> u16 {
        self.stack[idx]
    }

    pub fn reset(&mut self) {
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
    }
}