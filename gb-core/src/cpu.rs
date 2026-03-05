use crate::memory::Bus;
use crate::registers::Registers;

struct Cpu {
    registers: Registers, // Register
    pc: u16,              // Program Counter
    sp: u16,              // Stack Pointer
}

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) {
        let opcode = bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        self.execute(bus, opcode);
    }

    // LD [HL], A = bus.write_byte(hl, A)
    pub fn execute(&mut self, bus: &mut Bus, opcode: u8) {
        match opcode {
            0x00 => {}
            0x01 => {
                let n16 = self.fetch_u16(bus);
                self.registers.set_bc(n16);
            }

            _ => {}
        }
    }

    // LD R, R
    fn ld_r_r(&mut self, to: &mut u8, from: u8) {
        *to = from;
    }

    // LD R, [HL]
    fn ld_r_hl(&mut self, bus: &mut Bus, to: &mut u8) {
        let addr = self.registers.get_hl();
        *to = bus.read_byte(addr);
    }

    // LD [HL], R
    fn ld_hl_r(&mut self, bus: &mut Bus, from: u8) {
        let addr = self.registers.get_hl();
        bus.write_byte(addr, from);
    }

    fn fetch_u8(&mut self, bus: &Bus) -> u8 {
        let value = bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn fetch_u16(&mut self, bus: &Bus) -> u16 {
        let low = self.fetch_u8(bus) as u16;
        let high = self.fetch_u8(bus) as u16;
        (high << 8) | low
    }
}
