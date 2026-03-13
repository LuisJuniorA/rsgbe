pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    pub tima_reload_delay: i8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_reload_delay: -1,
        }
    }

    pub fn step(&mut self, cycles: u8) -> bool {
        let mut interrupt = false;

        for _ in 0..cycles {
            // Handle 1-cycle (4 T-cycles) reload delay
            if self.tima_reload_delay >= 0 {
                if self.tima_reload_delay == 0 {
                    self.tima = self.tma;
                    interrupt = true;
                }
                self.tima_reload_delay -= 1;
            }

            let old_div = self.div;
            self.div = self.div.wrapping_add(1);

            if self.detect_edge(old_div, self.tac, self.div, self.tac) {
                self.inc_tima();
            }
        }
        interrupt
    }

    pub fn reset_div(&mut self) -> bool {
        let old_div = self.div;
        self.div = 0;
        if self.detect_edge(old_div, self.tac, self.div, self.tac) {
            self.inc_tima();
            return self.tima_reload_delay == 0;
        }
        false
    }

    pub fn write_tima(&mut self, val: u8) {
        // Manual write cancels pending reload
        self.tima = val;
        self.tima_reload_delay = -1;
    }

    pub fn write_tac(&mut self, val: u8) -> bool {
        let old_div = self.div;
        let old_tac = self.tac;
        self.tac = val;

        if self.detect_edge(old_div, old_tac, self.div, self.tac) {
            self.inc_tima();
            return self.tima_reload_delay == 0;
        }
        false
    }

    fn inc_tima(&mut self) {
        let (new, overflow) = self.tima.overflowing_add(1);
        if overflow {
            self.tima = 0; // Transition state
            self.tima_reload_delay = 4;
        } else {
            self.tima = new;
        }
    }

    fn detect_edge(&self, div1: u16, tac1: u8, div2: u16, tac2: u8) -> bool {
        let b1 = self.get_bit(div1, tac1);
        let b2 = self.get_bit(div2, tac2);
        b1 == 1 && b2 == 0 // Falling edge
    }

    fn get_bit(&self, div: u16, tac: u8) -> u8 {
        if (tac & 0x04) == 0 {
            return 0;
        }
        let pos = match tac & 0x03 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!(),
        };
        ((div >> pos) & 1) as u8
    }
}
