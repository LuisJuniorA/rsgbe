use super::*;

impl Cpu {
    pub(super) fn execute_cb(&mut self, bus: &mut Bus, opcode: u8) -> u8 {
        match opcode {
            0x00..=0x07 => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.rlc_r8(r),
                    Operand8::MemHL => self.rlc_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x08..=0x0F => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.rrc_r8(r),
                    Operand8::MemHL => self.rrc_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x10..=0x17 => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.rl_r8(r),
                    Operand8::MemHL => self.rl_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x18..=0x1F => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.rr_r8(r),
                    Operand8::MemHL => self.rr_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x20..=0x27 => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.sla_r8(r),
                    Operand8::MemHL => self.sla_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x28..=0x2F => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.sra_r8(r),
                    Operand8::MemHL => self.sra_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x30..=0x37 => todo!(),
            0x38..=0x3F => todo!(),
            0x40..=0x47 => todo!(),
            0x48..=0x4F => todo!(),
            0x50..=0x57 => todo!(),
            0x58..=0x5F => todo!(),
            0x60..=0x67 => todo!(),
            0x68..=0x6F => todo!(),
            0x70..=0x77 => todo!(),
            0x78..=0x7F => todo!(),
            0x80..=0x87 => todo!(),
            0x88..=0x8F => todo!(),
            0x90..=0x97 => todo!(),
            0x98..=0x9F => todo!(),
            0xA0..=0xA7 => todo!(),
            0xA8..=0xAF => todo!(),
            0xB8..=0xBF => todo!(),
            0xB0..=0xB7 => todo!(),
            0xC0..=0xC7 => todo!(),
            0xC8..=0xCF => todo!(),
            0xD0..=0xD7 => todo!(),
            0xD8..=0xDF => todo!(),
            0xE0..=0xE7 => todo!(),
            0xE8..=0xEF => todo!(),
            0xF0..=0xF7 => todo!(),
            0xF8..=0xFF => todo!(),
        }
    }

    fn rlc_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.rlc_logic(val);
        self.set_reg8(reg, res);
    }

    fn rlc_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.rlc_logic(val);
        bus.write_byte(addr, res);
    }

    fn rlc_logic(&mut self, val: u8) -> u8 {
        let bit7 = (val & 0x80) >> 7;
        let res = (val << 1) | bit7;

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            (bit7 != 0).into(),
        );
        res
    }

    fn rrc_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.rrc_logic(val);
        self.set_reg8(reg, res);
    }

    fn rrc_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.rrc_logic(val);
        bus.write_byte(addr, res);
    }

    fn rrc_logic(&mut self, val: u8) -> u8 {
        let bit0 = val & 0x01;
        let res = (val >> 1) | (bit0 << 7);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            (bit0 != 0).into(),
        );
        res
    }

    fn rl_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.rl_logic(val);
        self.set_reg8(reg, res);
    }

    fn rl_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.rl_logic(val);
        bus.write_byte(addr, res);
    }

    fn rl_logic(&mut self, val: u8) -> u8 {
        let carry = if (self.registers.f & FLAG_C) != 0 { 1 } else { 0 };
        let bit7 = (val & 0x80) >> 7;
        let res = (val << 1) | carry;

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            (bit7 != 0).into(),
        );
        res
    }

    fn rr_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.rr_logic(val);
        self.set_reg8(reg, res);
    }

    fn rr_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.rr_logic(val);
        bus.write_byte(addr, res);
    }

    fn rr_logic(&mut self, val: u8) -> u8 {
        let carry = if (self.registers.f & FLAG_C) != 0 { 1 } else { 0 };
        let bit0 = val & 0x01;
        let res = (val >> 1) | (carry << 7);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            (bit0 != 0).into(),
        );
        res
    }

    fn sla_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.sla_logic(val);
        self.set_reg8(reg, res);
    }

    fn sla_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.sla_logic(val);
        bus.write_byte(addr, res);
    }

    fn sla_logic(&mut self, val: u8) -> u8 {
        let bit7 = (val & 0x80) >> 7;
        let res = val << 1;

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            (bit7 != 0).into(),
        );
        res
    }

    fn sra_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.sra_logic(val);
        self.set_reg8(reg, res);
    }

    fn sra_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.sra_logic(val);
        bus.write_byte(addr, res);
    }

    fn sra_logic(&mut self, val: u8) -> u8 {
        let bit0 = val & 0x01;
        let res = (val >> 1) | (val & 0x80);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            (bit0 != 0).into(),
        );
        res
    }
}
