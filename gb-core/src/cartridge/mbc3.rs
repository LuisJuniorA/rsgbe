use crate::cartridge::mbc::MBC;

#[derive(Default, Clone, Copy)]
struct RtcClock {
    seconds: u8,
    minutes: u8,
    hours: u8,
    days_l: u8,
    days_h: u8, // Bit 0: MSB of days, Bit 6: Halt, Bit 7: Carry
}

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,    // 32 KiB max for ram
    rom_bank: u8,    // Rom's Bank number (7 bits)
    ram_rtc_reg: u8, // Ram's Bank number (2 bits) or RTC Reg
    rom_bank_mask: u8,
    ram_bank_mask: u8,
    ram_enabled: bool,
    rtc_latch_state: bool, // Latch toggle
    rtc: RtcClock,         // Live clock
    latched_rtc: RtcClock, // Latched clock
}

impl MBC3 {
    pub fn new(rom: Vec<u8>, save: Option<Vec<u8>>, ram_size: usize) -> Self {
        let rom_num_banks = rom.len() / 0x4000;
        let rom_bank_mask = if rom_num_banks > 0 {
            (rom_num_banks.next_power_of_two() - 1) as u8
        } else {
            0
        };

        let ram_num_banks = (ram_size / 0x2000).max(1) as u8;
        let ram_bank_mask = ram_num_banks.next_power_of_two() - 1;

        MBC3 {
            rom,
            ram: save.unwrap_or(vec![0; ram_size]),
            rom_bank: 1,
            ram_rtc_reg: 0,
            rom_bank_mask,
            ram_bank_mask,
            ram_enabled: false,
            rtc_latch_state: false,
            rtc: RtcClock::default(),
            latched_rtc: RtcClock::default(),
        }
    }
}

impl MBC for MBC3 {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => {
                let bank = (self.rom_bank & self.rom_bank_mask) as usize;
                self.rom[bank * 0x4000 + (addr as usize - 0x4000)]
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }

                match self.ram_rtc_reg {
                    0x00..=0x03 => {
                        if self.ram.is_empty() {
                            return 0xFF;
                        }
                        let bank = (self.ram_rtc_reg & self.ram_bank_mask) as usize;
                        self.ram[bank * 0x2000 + (addr as usize - 0xA000)]
                    }
                    0x08 => self.latched_rtc.seconds,
                    0x09 => self.latched_rtc.minutes,
                    0x0A => self.latched_rtc.hours,
                    0x0B => self.latched_rtc.days_l,
                    0x0C => self.latched_rtc.days_h,
                    _ => 0xFF,
                }
            }
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            // Enabling RAM and RTC
            0x0000..=0x1FFF => self.ram_enabled = (val & 0x0F) == 0x0A,
            // ROM's Bank
            0x2000..=0x3FFF => {
                let mut bank = val & 0x7F;
                if bank == 0 {
                    bank = 1;
                }

                // 7 bits
                self.rom_bank = bank;
            }
            0x4000..=0x5FFF => {
                // Ram Bank or RTC Register
                self.ram_rtc_reg = val;
            }
            0x6000..=0x7FFF => {
                // Latch Clock
                if val == 0x00 {
                    self.rtc_latch_state = true;
                } else if val == 0x01 {
                    if self.rtc_latch_state {
                        self.latched_rtc = self.rtc;
                    }
                    self.rtc_latch_state = false;
                } else {
                    self.rtc_latch_state = false;
                }
            }
            // Writing in Ram or RTC
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }

                match self.ram_rtc_reg {
                    0x00..=0x03 => {
                        if !self.ram.is_empty() {
                            let bank = (self.ram_rtc_reg & self.ram_bank_mask) as usize;
                            let offset = (addr - 0xA000) as usize;
                            let addr_in_ram = (bank * 0x2000) + offset;
                            if addr_in_ram < self.ram.len() {
                                self.ram[addr_in_ram] = val;
                            }
                        }
                    }
                    0x08 => self.rtc.seconds = val,
                    0x09 => self.rtc.minutes = val,
                    0x0A => self.rtc.hours = val,
                    0x0B => self.rtc.days_l = val,
                    0x0C => self.rtc.days_h = val,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn get_save_data(&self) -> Option<&[u8]> {
        if self.ram.is_empty() {
            None
        } else {
            Some(&self.ram[..])
        }
    }
}
