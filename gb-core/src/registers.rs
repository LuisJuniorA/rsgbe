pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4d,
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub fn set_af(&mut self, to: u16) {
        self.a = (to >> 8) as u8;
        self.f = to as u8;
    }

    pub fn set_bc(&mut self, to: u16) {
        self.b = (to >> 8) as u8;
        self.c = to as u8;
    }

    pub fn set_de(&mut self, to: u16) {
        self.d = (to >> 8) as u8;
        self.e = to as u8;
    }

    pub fn set_hl(&mut self, to: u16) {
        self.h = (to >> 8) as u8;
        self.l = to as u8;
    }
}
