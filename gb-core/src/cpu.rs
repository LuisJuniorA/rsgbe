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

enum Operand8 {
    Reg(Reg8),
    MemHL,
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
    pub ime: bool,            // Interrupt Master Enable
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFE,
            ime: false,
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
                let n16 = Self::fetch_u16(bus, &mut self.pc);
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
                let n8 = Self::fetch_u8(bus, &mut self.pc);
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
                let addr = Self::fetch_u16(bus, &mut self.pc);
                self.write_u16(bus, addr, self.sp);
                20
            }
            0x09 /* ADD HL, BC */ => {
                let src = self.get_addr_from_source(AddrSource::BC);
                self.add_u16(AddrSource::HL, src);
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
                let n8 = Self::fetch_u8(bus, &mut self.pc);
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
                let n16 = Self::fetch_u16(bus, &mut self.pc);
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
                let n8 = Self::fetch_u8(bus, &mut self.pc);
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
                let src = self.get_addr_from_source(AddrSource::DE);
                self.add_u16(AddrSource::HL, src);
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
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.registers.e = n8;
                8
            }
            0x1F /* RRA */ => {
                let a = self.get_reg8(Reg8::A);
                let carry = (a & 0x01) != 0;
                let result = (a >> 1) | ((self.registers.f & FLAG_C) << 3);
                self.set_reg8(Reg8::A, result);
                self.set_flags(FlagOp::Unset, FlagOp::Unset, FlagOp::Unset, carry.into());
                4
            }
            0x20 /* JR NZ, e8 */ => {
                self.jp_rel(bus, Some(self.registers.f & FLAG_Z), true)
            }
            0x21 /* LD HL, n16 */ => {
                let n16 = Self::fetch_u16(bus, &mut self.pc);
                self.registers.set_hl(n16);
                12
            }
            0x22 /* LD [HL+], A */ => {
                self.ld_mem_r(bus, AddrSource::HLIncrement, Reg8::A);
                8
            }
            0x23 /* INC HL */ => {
                self.inc_u16(AddrSource::HL);
                8
            }
            0x24 /* INC H */ => {
                self.inc_u8(Reg8::H);
                4
            }
            0x25 /* DEC H */ => {
                self.dec_u8(Reg8::H);
                4
            }
            0x26 /* LD H, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.registers.h = n8;
                8
            }
            0x27 /* DAA */ => {
                let mut adjust: u8 = 0;
                let mut new_carry = (self.registers.f & FLAG_C) != 0;

                if (self.registers.f & FLAG_N) != 0 {
                    if (self.registers.f & FLAG_H) != 0 {
                        adjust = adjust.wrapping_add(0x06);
                    }
                    if (self.registers.f & FLAG_C) != 0 {
                        adjust = adjust.wrapping_add(0x60);
                    }
                    self.registers.a = self.registers.a.wrapping_sub(adjust);
                } else {
                    if (self.registers.f & FLAG_H) != 0 || ((self.registers.a) & 0x0F) > 0x09 {
                        adjust = adjust.wrapping_add(0x06);
                    }
                    if (self.registers.f & FLAG_C) != 0 || (self.registers.a) > 0x99 {
                        adjust = adjust.wrapping_add(0x60);
                        new_carry = true;
                    }
                    self.registers.a = self.registers.a.wrapping_add(adjust);
                }

                let z = self.registers.a == 0;
                self.set_flags(z.into(), FlagOp::Untouched, FlagOp::Unset, new_carry.into());

                4
            }
            0x28 /* JR Z, e8  */ => {
                self.jp_rel(bus, Some(self.registers.f & FLAG_Z), false)
            }
            0x29 /* ADD HL, HL  */ => {
                let src = self.get_addr_from_source(AddrSource::HL);
                self.add_u16(AddrSource::HL, src);
                8
            }
            0x2A /* LD A, [HL+] */ => {
                self.ld_r_mem(bus, Reg8::A, AddrSource::HLIncrement);
                8
            }
            0x2B /* DEC HL */ => {
                self.dec_u16(AddrSource::HL);
                8
            }
            0x2C /* INC L */ => {
                self.inc_u8(Reg8::L);
                4
            }
            0x2D /* DEC L */ => {
                self.dec_u8(Reg8::L);
                4
            }
            0x2E /* LD L, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.registers.l = n8;
                8
            }
            0x2F /* CPL */ => {
                self.registers.a = !self.registers.a;
                self.set_flags(FlagOp::Untouched, FlagOp::Set, FlagOp::Set, FlagOp::Untouched);
                4
            }
            0x30 /* JR NC, e8 */ => {
                self.jp_rel(bus, Some(self.registers.f & FLAG_C), true)
            }
            0x31 /* LD SP, n16 */ => {
                let n16 = Self::fetch_u16(bus, &mut self.pc);
                self.sp = n16;
                12
            }
            0x32 /* LD [HL-], A */ => {
                self.ld_mem_r(bus, AddrSource::HLDecrement, Reg8::A);
                8
            }
            0x33 /* INC SP */ => {
                self.sp = self.sp.wrapping_add(1);
                8
            }
            0x34 /* INC [HL] */ => {
                let addr = self.registers.get_hl();
                self.inc_mem(bus, addr);
                12
            }
            0x35 /* DEC [HL] */ => {
                let addr = self.registers.get_hl();
                self.dec_mem(bus, addr);
                12
            }
            0x36 /* LD [HL], n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                let addr = self.get_addr_from_source(AddrSource::HL);
                bus.write_byte(addr, n8);
                12
            }
            0x37 /* SCF */ => {
                self.set_flags(FlagOp::Untouched, FlagOp::Unset, FlagOp::Unset, FlagOp::Set);
                4
            }
            0x38 /* JR C, e8  */ => {
                self.jp_rel(bus, Some(self.registers.f & FLAG_C), false)
            }
            0x39 /* ADD HL, SP */ => {
                self.add_u16(AddrSource::HL, self.sp);
                8
            }
            0x3A /* LD A, [HL+] */ => {
                self.ld_r_mem(bus, Reg8::A, AddrSource::HLDecrement);
                8
            }
            0x3B /* DEC SP */ => {
                self.sp = self.sp.wrapping_sub(1);
                8
            }
            0x3C /* INC A */ => {
                self.inc_u8(Reg8::A);
                4
            }
            0x3D /* DEC A */ => {
                self.dec_u8(Reg8::A);
                4
            }
            0x3E /* LD A, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.registers.a = n8;
                8
            }
            0x3F /* CCF */ => {
                let new_c = self.get_reg8(Reg8::F) & FLAG_C == 0;
                self.set_flags(FlagOp::Untouched, FlagOp::Unset, FlagOp::Unset, new_c.into());
                4
            }
            v @ (0x40..=0x75 | 0x77..=0x7F) /* LD r8, r8 */ => {
                let (op_src, op_dest) = self.decode_opcode(v);

                match (op_src, op_dest) {
                    (Operand8::Reg(s), Operand8::Reg(d)) => {
                        let val = self.get_reg8(s);
                        self.set_reg8(d, val);
                        4
                    }
                    (Operand8::MemHL, Operand8::Reg(d)) => {
                        self.ld_r_mem(bus, d, AddrSource::HL);
                        8
                    }
                    (Operand8::Reg(src), Operand8::MemHL) => {
                        self.ld_mem_r(bus, AddrSource::HL, src);
                        8
                    }
                    // (MemHL, MemHL) does not exist. (It is HALT 0x76)
                    _ => unreachable!(),
                }
            }
            0x76 /* HALT */ => {
                todo!("WIP");
                unreachable!();
            }
            v @ 0x80..=0xBF => {
                let source_bits = v & 0b00_000_111;
                let source_op = self.decode_bits(source_bits);
                let val = match source_op {
                    Operand8::Reg(r) => self.get_reg8(r),
                    Operand8::MemHL => bus.read_byte(self.registers.get_hl()),
                };

                let op = (v & 0b00_111_000) >> 3;
                match op {
                    0b000 /* ADD A, r8 */ => self.add_u8(Reg8::A, val, false),
                    0b001 /* ADC A, r8 */ => self.add_u8(Reg8::A, val, true),
                    0b010 /* SUB A, r8 */ => self.sub_u8(Reg8::A, val, false),
                    0b011 /* SBC A, r8 */ => self.sub_u8(Reg8::A, val, true),
                    0b100 /* AND A, r8 */ => self.and_u8(Reg8::A, val),
                    0b101 /* XOR A, r8 */ => self.xor_u8(Reg8::A, val),
                    0b110 /* OR A, r8  */ => self.or_u8(Reg8::A, val),
                    0b111 /* CP A, r8  */ => self.cp_u8(Reg8::A, val),
                    _ => unreachable!(),
                }

                if let Operand8::MemHL = source_op { 8 } else { 4 }
            }
            0xC0 => /* RET NZ */ {
                self.ret(bus, Some(self.registers.f & FLAG_Z), true)
            }
            0xC1 => /* POP BC */ {
                self.pop(bus, AddrSource::BC);
                12
            }
            0xC2 => /* JP NZ, a16 */ {
                self.jp_abs(bus, Some(self.registers.f & FLAG_Z), true)
            }
            0xC3 => /* JP a16 */ {
                self.jp_abs(bus, None, false)
            }
            0xC4 => /* CALL NZ, a16 */ {
                self.call(bus, Some(self.registers.f & FLAG_Z), true)
            }
            0xC5 => /* PUSH BC */ {
                self.push(bus, AddrSource::BC);
                16
            }
            0xC6 /* ADD A, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.add_u8(Reg8::A, n8, false);
                8
            }
            0xC7 /* RST $00 */ => {
                self.rst(bus, 0x00);
                16
            }
            0xC8 /* RET Z */ => {
                self.ret(bus, Some(self.registers.f & FLAG_Z), false)
            }
            0xC9 /* RET */ => {
                self.ret(bus, None, false)
            }
            0xCA /* JP Z, a16 */ => {
                self.jp_abs(bus, Some(self.registers.f & FLAG_Z), false)
            }
            0xCB /* PREFIX */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                todo!("WIP");
            }
            0xCC /* CALL Z, a16 */ => {
                self.call(bus, Some(self.registers.f & FLAG_Z), false)
            }
            0xCD /* CALL a16 */ => {
                self.call(bus, None, false)
            }
            0xCE /* ADC A, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.add_u8(Reg8::A, n8, true);
                8
            }
            0xCF /* RST $08 */ => {
                self.rst(bus, 0x08);
                16
            }
            0xD0 /* RET NC */ => {
                self.ret(bus, Some(self.registers.f & FLAG_C), true)
            }
            0xD1 /* POP DE */ => {
                self.pop(bus, AddrSource::DE);
                12
            }
            0xD2 /* JP NC, a16 */ => {
                self.jp_abs(bus, Some(self.registers.f & FLAG_C), true)
            }
            0xD4 /* CALL NC, a16 */ => {
                self.call(bus, Some(self.registers.f & FLAG_C), true)
            }
            0xD5 /* PUSH DE */ => {
                self.push(bus, AddrSource::DE);
                16
            }
            0xD6 /* SUB A, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.sub_u8(Reg8::A, n8, false);
                8
            }
            0xD7 /* RST $10 */ => {
                self.rst(bus, 0x10);
                16
            }
            0xD8 /* RET C */ => {
                self.ret(bus, Some(self.registers.f & FLAG_C), false)
            }
            0xD9 /* RETI */ => {
                self.ime = true;
                self.ret(bus, None, false)
            }
            0xDA /* JP C, a16 */ => {
                self.jp_abs(bus, Some(self.registers.f & FLAG_C), false)
            }
            0xDC /* CALL C, a16 */ => {
                self.call(bus, Some(self.registers.f & FLAG_C), false)
            }
            0xDE /* SBC A, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.sub_u8(Reg8::A, n8, true);
                8
            }
            0xDF /* RST $18 */ => {
                self.rst(bus, 0x18);
                16
            }
            0xE0 /* LDH [a8], A */ => {
                let a8 = Self::fetch_u8(bus, &mut self.pc);
                self.ldh_mem_u8_r8(bus, a8, Reg8::A);
                12
            }
            0xE1 /* POP HL */ => {
                self.pop(bus, AddrSource::HL);
                12
            }
            0xE2 /* LDH [C], A */ => {
                self.ldh_mem_u8_r8(bus, self.registers.c, Reg8::A);
                8
            }
            0xE5 /* PUSH HL */ => {
                self.push(bus, AddrSource::HL);
                16
            }
            0xE6 /* AND A, n8 */ => {
                let n8 = Self::fetch_u8(bus, &mut self.pc);
                self.and_u8(Reg8::A, n8);
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

    fn ldh_mem_u8_r8(&mut self, bus: &mut Bus, dest_u8: u8, src: Reg8) {
        let val = self.get_reg8(src);
        let dest_u16 = 0xFF00 | dest_u8 as u16;
        bus.write_byte(dest_u16, val);
    }

    fn ldh_r8_mem_u8(&mut self, bus: &Bus, dest: Reg8, src_u8: u8) {
        let src_u16 = 0xFF00 | (src_u8 as u16);
        let val = bus.read_byte(src_u16);
        self.set_reg8(dest, val);
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

    fn fetch_u8(bus: &Bus, ptr: &mut u16) -> u8 {
        let value = bus.read_byte(*ptr);
        *ptr = ptr.wrapping_add(1);
        value
    }

    fn fetch_u16(bus: &Bus, ptr: &mut u16) -> u16 {
        let low = Self::fetch_u8(bus, ptr) as u16;
        let high = Self::fetch_u8(bus, ptr) as u16;
        (high << 8) | low
    }

    fn write_u16(&mut self, bus: &mut Bus, addr: u16, val: u16) {
        let low = (val & 0xFF) as u8;
        let high = (val >> 8) as u8;

        bus.write_byte(addr, low);
        bus.write_byte(addr.wrapping_add(1), high);
    }

    fn add_u16(&mut self, dest: AddrSource, source: u16) {
        let val = self.get_addr_from_source(dest);

        let (res, carry) = val.overflowing_add(source);

        // 16-bit Half-Carry: Check if the lower 12 bits overflowed
        let half_carry = (val & 0x0FFF) + (source & 0x0FFF) > 0x0FFF;

        self.set_addr_from_source(dest, res);

        // Flags: Z: Untouched, N: 0, H: carry from bit 11, C: carry from bit 15
        self.set_flags(
            FlagOp::Untouched,
            FlagOp::Unset,
            half_carry.into(),
            carry.into(),
        );
    }

    fn inc_mem(&mut self, bus: &mut Bus, addr: u16) {
        let val = bus.read_byte(addr);
        let result = val.wrapping_add(1);
        bus.write_byte(addr, result);

        self.set_flags(
            (result == 0).into(),
            FlagOp::Unset,
            ((val & 0x0F) == 0x0F).into(),
            FlagOp::Untouched,
        );
    }

    fn dec_mem(&mut self, bus: &mut Bus, addr: u16) {
        let val = bus.read_byte(addr);
        let result = val.wrapping_sub(1);
        bus.write_byte(addr, result);

        self.set_flags(
            (result == 0).into(),
            FlagOp::Set,
            ((val & 0x0F) == 0).into(),
            FlagOp::Untouched,
        );
    }

    fn jp_rel(&mut self, bus: &Bus, flag: Option<u8>, not: bool) -> u8 {
        let e8 = Self::fetch_u8(bus, &mut self.pc);
        let flag_value = flag.unwrap_or(1);
        if (flag_value != 0) ^ not {
            let offset = e8 as i8 as i16;
            self.pc = self.pc.wrapping_add_signed(offset);
            12
        } else {
            8
        }
    }

    fn jp_abs(&mut self, bus: &Bus, flag: Option<u8>, not: bool) -> u8 {
        let a16 = Self::fetch_u16(bus, &mut self.pc);
        let flag_value = flag.unwrap_or(1);
        if (flag_value != 0) ^ not {
            let offset = a16 as i16;
            self.pc = a16;
            16
        } else {
            12
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
            Reg8::F => self.registers.f = val & 0xF0,
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

    fn decode_opcode(&self, opcode: u8) -> (Operand8, Operand8) {
        // Most 8-bit load instructions (0x40 to 0x7F) follow a specific bit pattern:
        // Bit layout: 01_DDD_SSS
        // - "01" (bits 7-6): Opcode group identifier for "LD r, r"
        // - "DDD" (bits 5-3): Destination register index
        // - "SSS" (bits 2-0): Source register index
        //
        // Register mapping: 000=B, 001=C, 010=D, 011=E, 100=H, 101=L, 110=(HL), 111=A
        // Note: 0x76 (01 110 110) is a special case: it is decoded as HALT instead of LD (HL), (HL).

        // Extract destination bits (5, 4, 3) and shift them to the right
        let dest_bits = (opcode & 0b00_111_000) >> 3;

        // Extract source bits (2, 1, 0)
        let source_bits = opcode & 0b00_000_111;

        // Map bit patterns to Operand8 types (registers or memory)
        // Returns (source, destination) to match the expected function signature
        (self.decode_bits(source_bits), self.decode_bits(dest_bits))
    }

    fn decode_bits(&self, bits: u8) -> Operand8 {
        match bits {
            0b000 => Operand8::Reg(Reg8::B),
            0b001 => Operand8::Reg(Reg8::C),
            0b010 => Operand8::Reg(Reg8::D),
            0b011 => Operand8::Reg(Reg8::E),
            0b100 => Operand8::Reg(Reg8::H),
            0b101 => Operand8::Reg(Reg8::L),
            0b110 => Operand8::MemHL,
            0b111 => Operand8::Reg(Reg8::A),
            _ => unreachable!(),
        }
    }

    fn add_u8(&mut self, dest: Reg8, val: u8, carry: bool) {
        let current_val = self.get_reg8(dest);
        let c = if carry && (self.registers.f & FLAG_C) != 0 {
            1
        } else {
            0
        };

        // Use u16 to detect 8-bit carry
        let res_wide = (current_val as u16) + (val as u16) + (c as u16);
        let res = res_wide as u8;

        // Half-carry: bit 3 to bit 4
        let h = (current_val & 0x0F) + (val & 0x0F) + (c as u8) > 0x0F;

        self.set_reg8(dest, res);
        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            h.into(),
            (res_wide > 0xFF).into(),
        );
    }

    fn sub_u8(&mut self, dest: Reg8, val: u8, carry: bool) {
        let current_val = self.get_reg8(dest);
        let c = if carry && (self.registers.f & FLAG_C) != 0 {
            1
        } else {
            0
        };

        let res_wide = (current_val as i16) - (val as i16) - (c as i16);
        let res = res_wide as u8;

        // Half-borrow: check if lower nibble subtraction < 0
        let h = (current_val & 0x0F) < (val & 0x0F) + (c as u8);

        self.set_reg8(dest, res);
        self.set_flags(
            (res == 0).into(),
            FlagOp::Set,
            h.into(),
            (res_wide < 0).into(),
        );
    }

    fn ret(&mut self, bus: &Bus, flag: Option<u8>, not: bool) -> u8 {
        match flag {
            Some(v) => {
                if ((self.registers.f & v) != 0) ^ not {
                    let bytes = Self::fetch_u16(bus, &mut self.sp);
                    self.pc = bytes;
                    20
                } else {
                    8
                }
            }
            None => {
                let bytes = Self::fetch_u16(bus, &mut self.sp);
                self.pc = bytes;
                16
            }
        }
    }

    fn pop(&mut self, bus: &Bus, addr: AddrSource) {
        let value = Self::fetch_u16(bus, &mut self.sp);
        self.set_addr_from_source(addr, value);
    }

    fn push(&mut self, bus: &mut Bus, addr: AddrSource) {
        let value = self.get_addr_from_source(addr);
        self.sp = self.sp.wrapping_sub(2);
        self.write_u16(bus, self.sp, value);
    }

    fn call(&mut self, bus: &mut Bus, flag: Option<u8>, not: bool) -> u8 {
        let target_addr = Self::fetch_u16(bus, &mut self.pc);

        let should_call = flag.map_or(true, |v| (v != 0) ^ not);

        if should_call {
            self.sp = self.sp.wrapping_sub(2);
            self.write_u16(bus, self.sp, self.pc);
            self.pc = target_addr;

            24
        } else {
            12
        }
    }

    fn rst(&mut self, bus: &mut Bus, dest: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_u16(bus, self.sp, self.pc);
        self.pc = dest;
    }

    fn and_u8(&mut self, dest: Reg8, val: u8) {
        let res = self.get_reg8(dest) & val;
        self.set_reg8(dest, res);

        // AND always sets Half-Carry (H) to 1 on Game Boy
        self.set_flags((res == 0).into(), FlagOp::Unset, FlagOp::Set, FlagOp::Unset);
    }

    fn xor_u8(&mut self, dest: Reg8, val: u8) {
        let res = self.get_reg8(dest) ^ val;
        self.set_reg8(dest, res);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            FlagOp::Unset,
        );
    }

    fn or_u8(&mut self, dest: Reg8, val: u8) {
        let res = self.get_reg8(dest) | val;
        self.set_reg8(dest, res);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            FlagOp::Unset,
        );
    }

    fn cp_u8(&mut self, dest: Reg8, val: u8) {
        // CP is a comparison, so it acts like SUB but does NOT save the result back to the register
        let current_val = self.get_reg8(dest);
        let res = current_val.wrapping_sub(val);
        let h = (current_val & 0x0F) < (val & 0x0F);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Set,
            h.into(),
            (current_val < val).into(),
        );
    }
}
