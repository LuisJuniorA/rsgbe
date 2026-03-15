use crate::cartridge::mbc::MBC;
use std::time::{SystemTime, UNIX_EPOCH};

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
    last_time: u64,        // Tracks Unix timestamp in seconds
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

        let mut ram = vec![0; ram_size];
        let mut rtc = RtcClock::default();
        let mut latched_rtc = RtcClock::default();
        let mut last_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(mut save_data) = save {
            if save_data.len() == ram_size + 48 {
                let rtc_data = save_data.split_off(ram_size);

                let mut offset = 0;
                let mut read_rtc_val = || {
                    let val = u32::from_le_bytes(rtc_data[offset..offset + 4].try_into().unwrap());
                    offset += 4;
                    val as u8
                };

                // Read Current RTC
                rtc.seconds = read_rtc_val();
                rtc.minutes = read_rtc_val();
                rtc.hours = read_rtc_val();
                rtc.days_l = read_rtc_val();
                rtc.days_h = read_rtc_val();

                // Read Latched RTC
                latched_rtc.seconds = read_rtc_val();
                latched_rtc.minutes = read_rtc_val();
                latched_rtc.hours = read_rtc_val();
                latched_rtc.days_l = read_rtc_val();
                latched_rtc.days_h = read_rtc_val();

                last_time = u64::from_le_bytes(rtc_data[offset..offset + 8].try_into().unwrap());
            }

            // Copy RAM
            let len = save_data.len().min(ram_size);
            ram[..len].copy_from_slice(&save_data[..len]);
        }

        let mut mbc = MBC3 {
            rom,
            ram,
            rom_bank: 1,
            ram_rtc_reg: 0,
            rom_bank_mask,
            ram_bank_mask,
            ram_enabled: false,
            rtc_latch_state: false,
            rtc,
            latched_rtc,
            last_time,
        };

        // Catch the clock up to the current real-world time
        mbc.update_time();
        mbc
    }

    /// Calculates elapsed real-world time and updates the RTC registers
    fn update_time(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let elapsed = now.saturating_sub(self.last_time);
        self.last_time = now;

        // Bit 6 of days_h is the HALT flag. If set, time does not advance.
        if (self.rtc.days_h & 0x40) != 0 || elapsed == 0 {
            return;
        }

        let secs = self.rtc.seconds as u64 + elapsed;
        self.rtc.seconds = (secs % 60) as u8;

        let mins = self.rtc.minutes as u64 + (secs / 60);
        self.rtc.minutes = (mins % 60) as u8;

        let hours = self.rtc.hours as u64 + (mins / 60);
        self.rtc.hours = (hours % 24) as u8;

        let days_add = hours / 24;
        if days_add > 0 {
            let current_days = (self.rtc.days_l as u16) | (((self.rtc.days_h & 1) as u16) << 8);
            let new_days = current_days as u64 + days_add;

            // 9-bit overflow sets the Carry bit (Bit 7 of days_h)
            if new_days > 0x1FF {
                self.rtc.days_h |= 0x80;
            }

            let final_days = new_days & 0x1FF;
            self.rtc.days_l = (final_days & 0xFF) as u8;

            // Preserve the Halt and Carry flags (bits 6 and 7), and update the 9th day bit (bit 0)
            self.rtc.days_h = (self.rtc.days_h & 0xC0) | ((final_days >> 8) as u8 & 1);
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
                self.rom_bank = bank;
            }

            // Ram Bank or RTC Register
            0x4000..=0x5FFF => {
                self.ram_rtc_reg = val;
            }

            // Latch Clock
            0x6000..=0x7FFF => {
                if val == 0x00 {
                    self.rtc_latch_state = true;
                } else if val == 0x01 {
                    if self.rtc_latch_state {
                        self.update_time(); // Catch up the live clock before latching
                        self.latched_rtc = self.rtc;
                    }
                    self.rtc_latch_state = false;
                } else {
                    self.rtc_latch_state = false;
                }
            }

            // Writing to Ram or RTC
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
                    0x08..=0x0C => {
                        // Crucial: Update the clock up to THIS exact moment
                        // before the user overwrites the register
                        self.update_time();

                        match self.ram_rtc_reg {
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
            _ => {}
        }
    }

    fn get_save_data(&self) -> Option<Vec<u8>> {
        if self.ram.is_empty() && self.last_time == 0 {
            return None;
        }

        let mut save_data = self.ram.clone();

        let mut push_rtc_val = |val: u8| {
            save_data.extend_from_slice(&(val as u32).to_le_bytes());
        };

        // Serialize Current RTC
        push_rtc_val(self.rtc.seconds);
        push_rtc_val(self.rtc.minutes);
        push_rtc_val(self.rtc.hours);
        push_rtc_val(self.rtc.days_l);
        push_rtc_val(self.rtc.days_h);

        // Serialize Latched RTC
        push_rtc_val(self.latched_rtc.seconds);
        push_rtc_val(self.latched_rtc.minutes);
        push_rtc_val(self.latched_rtc.hours);
        push_rtc_val(self.latched_rtc.days_l);
        push_rtc_val(self.latched_rtc.days_h);

        // Serialize Timestamp
        save_data.extend_from_slice(&self.last_time.to_le_bytes());

        Some(save_data)
    }
}
