use crate::cartridge::mbc::MBC;
pub use crate::cartridge::mbc1::MBC1;
pub use crate::cartridge::mbc2::MBC2;
pub use crate::cartridge::mbc3::MBC3;
use crate::cartridge::mbc5::MBC5;
use crate::cartridge::no_mbc::NoMBC;

pub mod mbc;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
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
        let ram_size = Self::calculate_ram_size(data[0x0149]);

        let mbc: Box<dyn MBC> = match mbc_type {
            0x00 | 0xFF => Box::new(NoMBC::new(data)),
            0x01..=0x03 => {
                let is_mbc1m = Self::is_mbc1m(&data);
                Box::new(MBC1::new(data, ram_size, is_mbc1m))
            }
            0x05 | 0x06 => Box::new(MBC2::new(data)),
            0x0F | 0x10 => Box::new(MBC3::new(data, ram_size)),
            0x10..=0x1E => Box::new(MBC5::new(data, ram_size)),
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

    fn calculate_ram_size(code: u8) -> usize {
        match code {
            0x01 => 1 << 11, // 2 KiB
            0x02 => 1 << 13, // 8 KiB
            0x03 => 1 << 15, // 32 KiB
            0x04 => 1 << 17, // 128 KiB
            0x05 => 1 << 19, // 512 KiB
            _ => 0,
        }
    }

    fn is_mbc1m(data: &[u8]) -> bool {
        if data.len() <= 0x40000 {
            return false;
        }

        let nintendo_logo_start = 0x0104;
        let block_size = 0x40000; // 256 KiB

        let mut matches = 0;
        let num_blocks = data.len() / block_size;

        for i in 1..num_blocks.min(4) {
            let offset = i * block_size + nintendo_logo_start;
            if offset + 4 < data.len() {
                if data[offset..offset + 4] == [0xCE, 0xED, 0x66, 0x66] {
                    matches += 1;
                }
            }
        }

        matches > 0
    }
}
