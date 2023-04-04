const NUM_REGS: usize = 16;

pub struct IRegister {
    value: u16,
}

impl IRegister {
    pub fn new() -> Self {
        IRegister { value: 0 }
    }

    pub fn read(&self) -> u16 {
        self.value
    }

    pub fn write(&mut self, value: u16) {
        self.value = value;
    }

    pub fn reset(&mut self) {
        self.write(0);
    }
}

impl Default for IRegister {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VRegister {
    regs: [u8; NUM_REGS],
}

impl VRegister {
    pub fn new() -> Self {
        VRegister { regs: [0; NUM_REGS] }
    }

    pub fn read(&self, idx: usize) -> u8 {
        if idx >= NUM_REGS {
            panic!("Trying to access register bigger than 0xF");
        }

        self.regs[idx]
    }

    pub fn write(&mut self, idx: usize, value: u8) {
        if idx >= NUM_REGS {
            panic!("Trying to access register bigger than 0xF");
        }

        self.regs[idx] = value;
    }

    pub fn reset(&mut self) {
        self.regs = [0; NUM_REGS];
    }
}

impl Default for VRegister {
    fn default() -> Self {
        Self::new()
    }
}
