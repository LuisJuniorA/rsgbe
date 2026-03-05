use crate::memory::Bus;
use crate::registers::Registers;

pub struct Cpu {
    registers: Registers, // Register
    pc: u16,              // Program Counter
    sp: u16,              // Stack Pointer
}

enum AddrSource {
    BC,
    DE,
    HL,
    HLIncrement, // Pour LD A, [HL+]
    HLDecrement, // Pour LD A, [HL-]
}

pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFE,
        }
    }
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
            0x02 => {
                self.ld_mem_r(bus, AddrSource::BC, Reg8::A);
            }

            _ => {}
        }
    }

    fn get_addr_from_source(&mut self, source: AddrSource) -> u16 {
        match source {
            AddrSource::BC => self.registers.get_bc(),
            AddrSource::DE => self.registers.get_de(),
            AddrSource::HL => self.registers.get_hl(),
            AddrSource::HLIncrement => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_add(1));
                addr
            }
            AddrSource::HLDecrement => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_sub(1));
                addr
            }
        }
    }

    // LD R, R
    fn ld_r_r(&mut self, to: &mut u8, from: u8) {
        *to = from;
    }

    // LD R, [ADDR]
    fn ld_r_mem(&mut self, bus: &mut Bus, dest: Reg8, source: AddrSource) {
        let addr = self.get_addr_from_source(source);
        let val = bus.read_byte(addr);
        self.set_reg8(dest, val);
    }

    // LD [ADDR], R
    fn ld_mem_r(&mut self, bus: &mut Bus, dest_addr: AddrSource, src_reg: Reg8) {
        let val = self.get_reg8(src_reg);
        let addr = self.get_addr_from_source(dest_addr);
        bus.write_byte(addr, val);
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

    fn get_reg8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.registers.a,
            Reg8::B => self.registers.b,
            Reg8::C => self.registers.c,
            Reg8::D => self.registers.d,
            Reg8::E => self.registers.e,
            Reg8::H => self.registers.h,
            Reg8::L => self.registers.l,
        }
    }

    fn set_reg8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.registers.a = val,
            Reg8::B => self.registers.b = val,
            Reg8::C => self.registers.c = val,
            Reg8::D => self.registers.d = val,
            Reg8::E => self.registers.e = val,
            Reg8::H => self.registers.h = val,
            Reg8::L => self.registers.l = val,
        }
    }
}
