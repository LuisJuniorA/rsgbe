use crate::cartridge::Cartridge;
use crate::timer::Timer;

const WRAM_SIZE: usize = 1 << 13; // 8192 bytes
const HRAM_SIZE: usize = (1 << 7) - 1; // 127 bytes

pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    pub ie: u8,
    pub if_reg: u8,
    pub timer: Timer,
    serial_data: u8,
    pub serial_output: String,
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        Bus {
            cartridge: Cartridge::new(rom),
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            ie: 0,
            if_reg: 0,
            timer: Timer::new(),
            serial_data: 0,
            serial_output: String::new(),
            // vram: todo,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // $0000 - $7FFF: ROM
            0x0000..=0x7FFF => self.cartridge.read(addr),
            // $A000 - $BFFF External RAM
            0xA000..=0xBFFF => self.cartridge.read(addr),
            // $C000 - $DFFF: Work RAM
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize], // Echo RAM
            0xFF01 => self.serial_data,
            0xFF04 => (self.timer.div >> 8) as u8,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac,
            0xFF0F => self.if_reg,
            // $FF80 - $FFFE: High RAM
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
            // Unmapped
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartridge.write(addr, val),
            0x8000..=0x9FFF => {} // TODO: vram
            0xA000..=0xBFFF => self.cartridge.write(addr, val),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = val, // Echo RAM
            0xFF01 => self.serial_data = val,
            0xFF02 => {
                if val == 0x81 {
                    self.serial_output.push(self.serial_data as char);
                }
            }
            0xFF04 => {
                if self.timer.reset_div() {
                    self.if_reg |= 0x04; // Request timer interrupt if the reset caused an overflow
                }
            }
            0xFF05 => self.timer.tima = val,
            0xFF06 => self.timer.tma = val,
            0xFF07 => self.timer.tac = val,
            0xFF0F => self.if_reg = val,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            0xFFFF => self.ie = val,
            _ => {}
        }
    }
}
