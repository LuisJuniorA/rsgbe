use crate::cartridge::mbc::MBC;
pub use crate::cartridge::mbc1::MBC1;
use crate::cartridge::no_mbc::NoMBC;

pub mod mbc;
mod mbc1;
mod no_mbc;

pub struct Cartridge {
    mbc: Box<dyn MBC>,
}

impl Cartridge {
    pub fn new(mut data: Vec<u8>) -> Self {
        let min_size = 0x8000;
        let power_of_two_size = data.len().next_power_of_two().max(min_size);

        if data.len() < power_of_two_size {
            data.resize(power_of_two_size, 0xFF);
        }

        let mbc_type = data[0x0147];
        let mbc: Box<dyn MBC> = match mbc_type {
            0x00 => Box::new(NoMBC::new(data)),
            0x01..=0x03 => Box::new(MBC1::new(data)),
            _ => panic!("Unknown MBC : {:#02X}", mbc_type),
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
