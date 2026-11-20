pub struct Bus {
    memory: Box<[u8; 0x10000]>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: Box::new([0; 0x10000]),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
