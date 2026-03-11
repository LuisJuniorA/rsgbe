use crate::cartridge::mbc::MBC;
use crate::cartridge::no_mbc::NoMBC;

mod mbc;
mod no_mbc;

pub struct Cartridge {
    mbc: Box<dyn MBC>,
}

impl Cartridge {
    pub fn new(mut data: Vec<u8>) -> Self {
        let power_of_two_size = data.len().next_power_of_two().max(32768);

        if data.len() < power_of_two_size {
            data.resize(power_of_two_size, 0xFF);
        }

        let mbc_type = data[0x0147];
        let mbc: Box<dyn MBC> = match mbc_type {
            0x00 => Box::new(NoMBC::new(data)),
            _ => panic!("MBC inconnu"),
        };

        Cartridge { mbc }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.mbc.read_byte(addr)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.mbc.write_byte(addr, val);
    }
}
