use crate::cartridge::Cartridge;
use crate::joypad::Joypad;
use crate::ppu::Ppu;
use crate::timer::Timer;

const WRAM_SIZE: usize = 1 << 13; // 8192 bytes
const HRAM_SIZE: usize = (1 << 7) - 1; // 127 bytes
const VRAM_SIZE: usize = 0x2000; // 8192 bytes;
const OAM_SIZE: usize = 0xA0; // 160 bytes

pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    pub ie: u8,
    pub if_reg: u8,
    pub timer: Timer,
    pub ppu: Ppu,
    pub joypad: Joypad,
    serial_data: u8,
    pub serial_output: String,
}

impl Bus {
    pub fn new(rom: Vec<u8>, save: Option<Vec<u8>>) -> Self {
        Bus {
            cartridge: Cartridge::new(rom, save),
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            ie: 0,
            if_reg: 0,
            timer: Timer::new(),
            ppu: Ppu::new(),
            joypad: Joypad::new(),
            serial_data: 0,
            serial_output: String::new(),
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
            // $8000 - $9FFF: Video RAM
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            // 0xFF00: Joypad
            0xFF00 => self.joypad.read(),
            0xFF01 => self.serial_data,
            0xFF04 => (self.timer.div >> 8) as u8,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac,
            0xFF0F => self.if_reg | 0xE0,

            0xFF40 => self.ppu.lcdc,
            0xFF41 => self.ppu.stat,
            0xFF42 => self.ppu.scy,
            0xFF43 => self.ppu.scx,
            0xFF44 => self.ppu.ly,
            0xFF45 => self.ppu.lyc,
            0xFF47 => self.ppu.bgp,
            0xFF48 => self.ppu.obp0,
            0xFF49 => self.ppu.obp1,
            0xFF4A => self.ppu.wy,
            0xFF4B => self.ppu.wx,

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
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xA000..=0xBFFF => self.cartridge.write(addr, val),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = val, // Echo RAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = val,

            0xFF00 => self.joypad.write(val),
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
            0xFF40 => self.ppu.lcdc = val,
            // 0xFF41: LCD Status Register. 0x78 for Read/Write bits. 0x07 for Read only. 0x80 Because last bit is always 1
            0xFF41 => self.ppu.stat = (val & 0x78) | (self.ppu.stat & 0x07) | 0x80,
            0xFF42 => self.ppu.scy = val,
            0xFF43 => self.ppu.scx = val,
            0xFF44 => self.ppu.ly = val,
            0xFF45 => self.ppu.lyc = val,
            // DAM
            0xFF46 => {
                for i in 0..160 {
                    let src_addr = ((val as u16) << 8) + i as u16;
                    let data = self.read_byte(src_addr);
                    self.oam[i as usize] = data;
                }
            }
            0xFF47 => self.ppu.bgp = val,
            0xFF48 => self.ppu.obp0 = val,
            0xFF49 => self.ppu.obp1 = val,
            0xFF4A => self.ppu.wy = val,
            0xFF4B => self.ppu.wx = val,

            0xFF05 => self.timer.tima = val,
            0xFF06 => self.timer.tma = val,
            0xFF07 => self.timer.tac = val,

            0xFF0F => self.if_reg = val | 0xE0,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            0xFFFF => self.ie = val,
            _ => {}
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        if self.timer.step(cycles) {
            self.if_reg |= 0x04;
        }

        let ppu_interrupts = self.ppu.step(cycles, &self.vram, &self.oam);
        self.if_reg |= ppu_interrupts;
    }

    pub fn get_save_data(&self) -> Option<Vec<u8>> {
        self.cartridge.get_save_data()
    }
}
