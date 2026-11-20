use crate::utils::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Regs {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy)]
pub enum Flags {
    Z,
    N,
    H,
    C,
}

impl Flags {
    const fn mask(self) -> u8 {
        match self {
            Flags::Z => 0x80,
            Flags::N => 0x40,
            Flags::H => 0x20,
            Flags::C => 0x10,
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0x0000,
            sp: 0x0000,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }

    pub fn get_r8(&self, r: Regs) -> u8 {
        match r {
            Regs::A => self.a,
            Regs::B => self.b,
            Regs::C => self.c,
            Regs::D => self.d,
            Regs::E => self.e,
            Regs::F => self.f,
            Regs::H => self.h,
            Regs::L => self.l,
        }
    }

    pub fn set_r8(&mut self, r: Regs, value: u8) {
        match r {
            Regs::A => self.a = value,
            Regs::B => self.b = value,
            Regs::C => self.c = value,
            Regs::D => self.d = value,
            Regs::E => self.e = value,
            Regs::F => self.f = value & 0xF0, // lower 4 bits of f are always 0
            Regs::H => self.h = value,
            Regs::L => self.l = value,
        }
    }

    pub fn get_r16(&self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => merge_bytes(self.a, self.f),
            Regs16::BC => merge_bytes(self.b, self.c),
            Regs16::DE => merge_bytes(self.d, self.e),
            Regs16::HL => merge_bytes(self.h, self.l),
            Regs16::SP => self.sp,
        }
    }

    pub fn set_r16(&mut self, r: Regs16, value: u16) {
        let high = value.high_byte();
        let low = value.low_byte();

        match r {
            Regs16::AF => {
                self.set_r8(Regs::A, high);
                self.set_r8(Regs::F, low);
            }
            Regs16::BC => {
                self.set_r8(Regs::B, high);
                self.set_r8(Regs::C, low);
            }
            Regs16::DE => {
                self.set_r8(Regs::D, high);
                self.set_r8(Regs::E, low);
            }
            Regs16::HL => {
                self.set_r8(Regs::H, high);
                self.set_r8(Regs::L, low);
            }
            Regs16::SP => {
                self.sp = value;
            }
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        self.f & flag.mask() != 0
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            self.f |= flag.mask();
        } else {
            self.f &= !flag.mask();
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
