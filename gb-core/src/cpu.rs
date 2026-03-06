use crate::memory::Bus;
use crate::registers::Registers;

#[derive(Clone, Copy)]
enum AddrSource {
    AF,
    BC,
    DE,
    HL,
    HLIncrement, // For LD A, [HL+]
    HLDecrement, // For LD A, [HL-]
}
#[derive(Clone, Copy)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub const FLAG_Z: u8 = 1 << 7; // 0b1000_0000
pub const FLAG_N: u8 = 1 << 6; // 0b0100_0000
pub const FLAG_H: u8 = 1 << 5; // 0b0010_0000
pub const FLAG_C: u8 = 1 << 4; // 0b0001_0000

enum FlagOp {
    Set,
    Unset,
    Untouched,
}

impl From<bool> for FlagOp {
    fn from(value: bool) -> Self {
        if value { FlagOp::Set } else { FlagOp::Unset }
    }
}
pub struct Cpu {
    pub registers: Registers, // Register
    pub pc: u16,              // Program Counter
    pub sp: u16,              // Stack Pointer
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFE,
        }
    }
    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let opcode = bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        self.execute(bus, opcode)
    }

    // LD [HL], A = bus.write_byte(hl, A)
    pub fn execute(&mut self, bus: &mut Bus, opcode: u8) -> u8 {
        match opcode {
            0x00 /* NOP */ => 4,
            0x01 /* LD BC, n16 */ => {
                let n16 = self.fetch_u16(bus);
                self.registers.set_bc(n16);
                12
            }
            0x02 /* LD [BC], A */ => {
                self.ld_mem_r(bus, AddrSource::BC, Reg8::A);
                8
            }
            0x03 /* INC BC */ => {
                self.inc_u16(AddrSource::BC);
                8
            }
            0x04 /* INC B */ => {
                self.inc_u8(Reg8::B);
                4
            }
            0x05 /* DEC B */ => {
                self.dec_u8(Reg8::B);
                4
            }
            0x06 /* LD B, n8 */ => {
                let n8 = self.fetch_u8(bus);
                self.registers.b = n8;
                8
            }
            0x07 /* RLCA */ => {
                let a = self.get_reg8(Reg8::A);
                let carry = (a & 0x80) != 0;
                let result = a.rotate_left(1);
                self.set_reg8(Reg8::A, result);
                self.set_flags(FlagOp::Unset, FlagOp::Unset, FlagOp::Unset, carry.into());
                4
            }
            0x08 /* LD [a16], SP */ => {
                let addr = self.fetch_u16(bus);
                self.write_u16(bus, addr, self.sp);
                20
            }
            0x09 /* ADD HL, BC */ => {
                self.add_u16(AddrSource::HL, AddrSource::BC);
                8
            }
            0x0A /* LD A, [BC] */ => {
                self.ld_r_mem(bus, Reg8::A, AddrSource::BC);
                8
            }
            0x0B /* DEC BC */ => {
                self.dec_u16(AddrSource::BC);
                8
            }
            0x0C /* INC C */ => {
                self.inc_u8(Reg8::C);
                4
            }
            0x0D /* DEC C */ => {
                self.dec_u8(Reg8::C);
                4
            }
            0x0E /* LD C, n8 */ => {
                let n8 = self.fetch_u8(bus);
                self.registers.c = n8;
                8
            }
            0x0F /* RRCA */ => {
                let a = self.get_reg8(Reg8::A);
                let carry = (a & 0x01) != 0;
                let result = a.rotate_right(1);
                self.set_reg8(Reg8::A, result);
                self.set_flags(FlagOp::Unset, FlagOp::Unset, FlagOp::Unset, carry.into());
                4
            }
            0x10 /*STOP n8 */ => {
                todo!("WIP");
                unreachable!();
                4
            }
            0x11 /* LD DE, n16 */ => {
                let n16 = self.fetch_u16(bus);
                self.registers.set_de(n16);
                12
            }
            0x12 /* LD [DE], A */ => {
                self.ld_mem_r(bus, AddrSource::DE, Reg8::A);
                8
            }
            0x13 /* INC DE */ => {
                self.inc_u16(AddrSource::DE);
                8
            }
            0x14 /* INC D */ => {
                self.inc_u8(Reg8::D);
                4
            }
            0x15 /* DEC D */ => {
                self.dec_u8(Reg8::D);
                4
            }
            0x16 /* LD D, n8 */ => {
                let n8 = self.fetch_u8(bus);
                self.registers.d = n8;
                8
            }
            0x17 /* RLA */ => {
                let a = self.get_reg8(Reg8::A);
                let carry = (a & 0x80) != 0;
                let result = (a << 1) | ((self.registers.f & FLAG_C) >> 4);
                self.set_reg8(Reg8::A, result);
                self.set_flags(FlagOp::Unset, FlagOp::Unset, FlagOp::Unset, carry.into());
                4
            }
            0x18 /* JR e8 */ => {
                self.jp_rel(bus, None, false)
            }
            0x19 /* ADD HL, DE */ => {
                self.add_u16(AddrSource::HL, AddrSource::DE);
                8
            }
            0x1A /* LD A, [DE] */ => {
                self.ld_r_mem(bus, Reg8::A, AddrSource::DE);
                8
            }
            0x1B /* DEC DE */ => {
                self.dec_u16(AddrSource::DE);
                8
            }
            0x1C /* INC E */ => {
                self.inc_u8(Reg8::E);
                4
            }
            0x1D /* DEC E */ => {
                self.dec_u8(Reg8::E);
                4
            }
            0x1E /* LD E, n8 */ => {
                let n8 = self.fetch_u8(bus);
                self.registers.e = n8;
                8
            }
            0x1F /* RRA */ => {
                let a = self.get_reg8(Reg8::A);
                let carry = (a & 0x01) != 0;
                let result = (a >> 1) | ((self.registers.f & FLAG_C) >> 4);
                self.set_reg8(Reg8::A, result);
                self.set_flags(FlagOp::Unset, FlagOp::Unset, FlagOp::Unset, carry.into());
                4
            }
            0x20 /*  JR NZ, e8 */ => {
                self.jp_rel(bus, Some(self.registers.f & FLAG_Z), true)
            }
            0x21 /* LD HL, n16 */ => {
                let n16 = self.fetch_u16(bus);
                self.registers.set_hl(n16);
                12
            }
            0x22 /* LD [HL+], A */ => {
                self.ld_mem_r(bus, AddrSource::HLIncrement, Reg8::A);
                8
            }


            v @ (0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD) => {
                panic!("Illegal opcode {:#04X} encountered", v);
            }
            _ => 4,
        }
    }

    fn get_addr_from_source(&mut self, source: AddrSource) -> u16 {
        match source {
            AddrSource::AF => self.registers.get_af(),
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

    fn set_addr_from_source(&mut self, dest: AddrSource, to: u16) {
        match dest {
            AddrSource::AF => self.registers.set_af(to),
            AddrSource::BC => self.registers.set_bc(to),
            AddrSource::DE => self.registers.set_de(to),
            AddrSource::HL => self.registers.set_hl(to),
            _ => {}
        }
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

    fn inc_u8(&mut self, addr: Reg8) {
        let val = self.get_reg8(addr);
        let result = val.wrapping_add(1);
        self.set_reg8(addr, result);

        // Z: Set if result is 0
        // N: Reset (0)
        // H: Set if carry from bit 3 (lower nibble was 0x0F)
        // C: Untouched
        self.set_flags(
            (result == 0).into(),
            FlagOp::Unset,
            ((val & 0x0F) == 0x0F).into(),
            FlagOp::Untouched,
        );
    }

    fn inc_u16(&mut self, addr: AddrSource) {
        let to = self.get_addr_from_source(addr).wrapping_add(1);
        self.set_addr_from_source(addr, to);
    }

    fn dec_u8(&mut self, addr: Reg8) {
        let val = self.get_reg8(addr);
        let result = val.wrapping_sub(1);
        self.set_reg8(addr, result);

        // Z: Set if result is 0
        // N: Set (1)
        // H: Set if carry from bit 3 (lower nibble was 0x0F)
        // C: Untouched
        self.set_flags(
            (result == 0).into(),
            FlagOp::Set,
            ((val & 0x0F) == 0).into(),
            FlagOp::Untouched,
        );
    }

    fn dec_u16(&mut self, addr: AddrSource) {
        let to = self.get_addr_from_source(addr).wrapping_sub(1);
        self.set_addr_from_source(addr, to);
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

    fn write_u16(&mut self, bus: &mut Bus, addr: u16, val: u16) {
        let low = (val & 0xFF) as u8;
        let high = (val >> 8) as u8;

        bus.write_byte(addr, low);
        bus.write_byte(addr.wrapping_add(1), high);
    }

    fn add_u16(&mut self, dest: AddrSource, source: AddrSource) {
        let val1 = self.get_addr_from_source(dest);
        let val2 = self.get_addr_from_source(source);

        let (res, carry) = val1.overflowing_add(val2);

        // 16-bit Half-Carry: Check if the lower 12 bits overflowed
        let half_carry = (val1 & 0x0FFF) + (val2 & 0x0FFF) > 0x0FFF;

        self.set_addr_from_source(dest, res);

        // Flags: Z: Untouched, N: 0, H: carry from bit 11, C: carry from bit 15
        self.set_flags(
            FlagOp::Untouched,
            FlagOp::Unset,
            half_carry.into(),
            carry.into(),
        );
    }

    fn jp_rel(&mut self, bus: &Bus, flag: Option<u8>, not: bool) -> u8 {
        let e8 = self.fetch_u8(bus);
        let flag_value = flag.unwrap_or(1);
        if (flag_value != 0) ^ not {
            let offset = e8 as i8 as i16;
            self.pc = self.pc.wrapping_add_signed(offset);
            12
        } else {
            8
        }
    }

    fn get_reg8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.registers.a,
            Reg8::B => self.registers.b,
            Reg8::C => self.registers.c,
            Reg8::D => self.registers.d,
            Reg8::E => self.registers.e,
            Reg8::F => self.registers.f,
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
            Reg8::F => self.registers.f = val,
            Reg8::H => self.registers.h = val,
            Reg8::L => self.registers.l = val,
        }
    }

    fn set_flags(&mut self, z: FlagOp, n: FlagOp, h: FlagOp, c: FlagOp) {
        match z {
            FlagOp::Set => self.registers.f |= FLAG_Z,
            FlagOp::Unset => self.registers.f &= !FLAG_Z,
            FlagOp::Untouched => {}
        }

        match n {
            FlagOp::Set => self.registers.f |= FLAG_N,
            FlagOp::Unset => self.registers.f &= !FLAG_N,
            FlagOp::Untouched => {}
        }

        match h {
            FlagOp::Set => self.registers.f |= FLAG_H,
            FlagOp::Unset => self.registers.f &= !FLAG_H,
            FlagOp::Untouched => {}
        }

        match c {
            FlagOp::Set => self.registers.f |= FLAG_C,
            FlagOp::Unset => self.registers.f &= !FLAG_C,
            FlagOp::Untouched => {}
        }
    }
}
