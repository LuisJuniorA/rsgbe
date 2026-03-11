use super::*;

impl Cpu {
    pub(super) fn get_addr_from_source(&mut self, source: AddrSource) -> u16 {
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

    pub(super) fn set_addr_from_source(&mut self, dest: AddrSource, to: u16) {
        match dest {
            AddrSource::AF => self.registers.set_af(to),
            AddrSource::BC => self.registers.set_bc(to),
            AddrSource::DE => self.registers.set_de(to),
            AddrSource::HL => self.registers.set_hl(to),
            _ => {}
        }
    }

    // LD R, [ADDR]
    pub(super) fn ld_r_mem(&mut self, bus: &mut Bus, dest: Reg8, source: AddrSource) {
        let addr = self.get_addr_from_source(source);
        let val = bus.read_byte(addr);
        self.set_reg8(dest, val);
    }

    // LD [ADDR], R
    pub(super) fn ld_mem_r(&mut self, bus: &mut Bus, dest_addr: AddrSource, src_reg: Reg8) {
        let val = self.get_reg8(src_reg);
        let addr = self.get_addr_from_source(dest_addr);
        bus.write_byte(addr, val);
    }

    pub(super) fn ldh_mem_u8_r8(&mut self, bus: &mut Bus, dest_u8: u8, src: Reg8) {
        let val = self.get_reg8(src);
        let dest_u16 = 0xFF00 | dest_u8 as u16;
        bus.write_byte(dest_u16, val);
    }

    pub(super) fn ldh_r8_mem_u8(&mut self, bus: &Bus, dest: Reg8, src_u8: u8) {
        let src_u16 = 0xFF00 | (src_u8 as u16);
        let val = bus.read_byte(src_u16);
        self.set_reg8(dest, val);
    }

    pub(super) fn inc_u8(&mut self, addr: Reg8) {
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

    pub(super) fn inc_u16(&mut self, addr: AddrSource) {
        let to = self.get_addr_from_source(addr).wrapping_add(1);
        self.set_addr_from_source(addr, to);
    }

    pub(super) fn dec_u8(&mut self, addr: Reg8) {
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

    pub(super) fn dec_u16(&mut self, addr: AddrSource) {
        let to = self.get_addr_from_source(addr).wrapping_sub(1);
        self.set_addr_from_source(addr, to);
    }

    pub(super) fn fetch_u8(bus: &Bus, ptr: &mut u16) -> u8 {
        let value = bus.read_byte(*ptr);
        *ptr = ptr.wrapping_add(1);
        value
    }

    pub(super) fn fetch_u16(bus: &Bus, ptr: &mut u16) -> u16 {
        let low = Self::fetch_u8(bus, ptr) as u16;
        let high = Self::fetch_u8(bus, ptr) as u16;
        (high << 8) | low
    }

    pub(super) fn write_u16(&mut self, bus: &mut Bus, addr: u16, val: u16) {
        let low = (val & 0xFF) as u8;
        let high = (val >> 8) as u8;

        bus.write_byte(addr, low);
        bus.write_byte(addr.wrapping_add(1), high);
    }

    pub(super) fn add_u16(&mut self, dest: AddrSource, source: u16) {
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

    pub(super) fn inc_mem(&mut self, bus: &mut Bus, addr: u16) {
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

    pub(super) fn dec_mem(&mut self, bus: &mut Bus, addr: u16) {
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

    pub(super) fn jp_rel(&mut self, bus: &Bus, flag: Option<u8>, not: bool) -> u8 {
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

    pub(super) fn jp_abs(&mut self, bus: &Bus, flag: Option<u8>, not: bool) -> u8 {
        let a16 = Self::fetch_u16(bus, &mut self.pc);
        let flag_value = flag.unwrap_or(1);
        if (flag_value != 0) ^ not {
            self.pc = a16;
            16
        } else {
            12
        }
    }

    pub(super) fn get_reg8(&self, reg: Reg8) -> u8 {
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

    pub(super) fn set_reg8(&mut self, reg: Reg8, val: u8) {
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

    pub(super) fn set_flags(&mut self, z: FlagOp, n: FlagOp, h: FlagOp, c: FlagOp) {
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

    pub(super) fn decode_opcode(&self, opcode: u8) -> (Operand8, Operand8) {
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

    pub(super) fn decode_bits(&self, bits: u8) -> Operand8 {
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

    pub(super) fn add_u8(&mut self, dest: Reg8, val: u8, carry: bool) {
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

    pub(super) fn sub_u8(&mut self, dest: Reg8, val: u8, carry: bool) {
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

    pub(super) fn ret(&mut self, bus: &Bus, flag: Option<u8>, not: bool) -> u8 {
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

    pub(super) fn pop(&mut self, bus: &Bus, addr: AddrSource) {
        let mut value = Self::fetch_u16(bus, &mut self.sp);
        if let AddrSource::AF = addr {
            value &= 0xFFF0;
        }
        self.set_addr_from_source(addr, value);
    }

    pub(super) fn push(&mut self, bus: &mut Bus, addr: AddrSource) {
        let value = self.get_addr_from_source(addr);
        self.sp = self.sp.wrapping_sub(2);
        self.write_u16(bus, self.sp, value);
    }

    pub(super) fn call(&mut self, bus: &mut Bus, flag: Option<u8>, not: bool) -> u8 {
        let target_addr = Self::fetch_u16(bus, &mut self.pc);

        let should_call = flag.is_none_or(|v| (v != 0) ^ not);

        if should_call {
            self.sp = self.sp.wrapping_sub(2);
            self.write_u16(bus, self.sp, self.pc);
            self.pc = target_addr;

            24
        } else {
            12
        }
    }

    pub(super) fn rst(&mut self, bus: &mut Bus, dest: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_u16(bus, self.sp, self.pc);
        self.pc = dest;
    }

    pub(super) fn and_u8(&mut self, dest: Reg8, val: u8) {
        let res = self.get_reg8(dest) & val;
        self.set_reg8(dest, res);

        // AND always sets Half-Carry (H) to 1 on Game Boy
        self.set_flags((res == 0).into(), FlagOp::Unset, FlagOp::Set, FlagOp::Unset);
    }

    pub(super) fn xor_u8(&mut self, dest: Reg8, val: u8) {
        let res = self.get_reg8(dest) ^ val;
        self.set_reg8(dest, res);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            FlagOp::Unset,
        );
    }

    pub(super) fn or_u8(&mut self, dest: Reg8, val: u8) {
        let res = self.get_reg8(dest) | val;
        self.set_reg8(dest, res);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            FlagOp::Unset,
        );
    }

    pub(super) fn cp_u8(&mut self, dest: Reg8, val: u8) {
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
