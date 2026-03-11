#[derive(Default)]
pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, cycles: u8) -> bool {
        let mut request_interrupt = false;

        for _ in 0..cycles {
            let old_div = self.div;
            self.div = self.div.wrapping_add(1);

            if (self.tac & 0x04) != 0 {
                let bit_pos = match self.tac & 0x03 {
                    0 => 9, // 4096 Hz
                    1 => 3, // 262144 Hz
                    2 => 5, // 65536 Hz
                    3 => 7, // 16384 Hz
                    _ => unreachable!(),
                };

                let bit_mask = 1 << bit_pos;
                let old_bit = (old_div & bit_mask) != 0;
                let new_bit = (self.div & bit_mask) != 0;

                if old_bit && !new_bit {
                    let (new_tima, overflow) = self.tima.overflowing_add(1);
                    if overflow {
                        self.tima = self.tma;
                        request_interrupt = true;
                    } else {
                        self.tima = new_tima;
                    }
                }
            }
        }

        request_interrupt
    }

    pub fn reset_div(&mut self) -> bool {
        let mut request_interrupt = false;

        let bit_pos = match self.tac & 0x03 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!(),
        };

        let bit_mask = 1 << bit_pos;

        let old_bit = (self.div & bit_mask) != 0;

        self.div = 0;

        if (self.tac & 0x04) != 0 && old_bit {
            let (new_tima, overflow) = self.tima.overflowing_add(1);
            if overflow {
                self.tima = self.tma;
                request_interrupt = true;
            } else {
                self.tima = new_tima;
            }
        }

        request_interrupt
    }
}
