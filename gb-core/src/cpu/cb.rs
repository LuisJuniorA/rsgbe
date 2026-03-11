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
            0x30..=0x37 => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.swap_r8(r),
                    Operand8::MemHL => self.swap_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x38..=0x3F => {
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.srl_r8(r),
                    Operand8::MemHL => self.srl_hl(bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
            0x40..=0x7F => {
                let bit = (opcode & 0x38) >> 3;
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.bit_r8(bit, r),
                    Operand8::MemHL => self.bit_hl(bit, bus),
                }

                if let Operand8::MemHL = operand { 12 } else { 8 }
            }
            0x80..=0xBF => {
                let bit = (opcode & 0x38) >> 3;
                let operand = self.decode_bits(opcode & 0x07);

                match operand {
                    Operand8::Reg(r) => self.res_r8(bit, r),
                    Operand8::MemHL => self.res_hl(bit, bus),
                }

                if let Operand8::MemHL = operand { 16 } else { 8 }
            }
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

    fn swap_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.swap_logic(val);
        self.set_reg8(reg, res);
    }

    fn swap_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.swap_logic(val);
        bus.write_byte(addr, res);
    }

    fn swap_logic(&mut self, val: u8) -> u8 {
        let res = (val << 4) | (val >> 4);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            FlagOp::Unset,
        );
        res
    }

    fn srl_r8(&mut self, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = self.srl_logic(val);
        self.set_reg8(reg, res);
    }

    fn srl_hl(&mut self, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = self.srl_logic(val);
        bus.write_byte(addr, res);
    }

    fn srl_logic(&mut self, val: u8) -> u8 {
        let bit0 = val & 0x01;
        let res = val >> 1;

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            (bit0 != 0).into(),
        );
        res
    }

    fn bit_r8(&mut self, bit: u8, reg: Reg8) {
        let val = self.get_reg8(reg);
        self.bit_logic(bit, val);
    }

    fn bit_hl(&mut self, bit: u8, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        self.bit_logic(bit, val);
    }

    fn bit_logic(&mut self, bit: u8, val: u8) {
        let bit_val = (val >> bit) & 0x01;

        self.set_flags(
            (bit_val == 0).into(),
            FlagOp::Unset,
            FlagOp::Set,
            FlagOp::Untouched,
        );
    }

    fn res_r8(&mut self, bit: u8, reg: Reg8) {
        let val = self.get_reg8(reg);
        let res = val & !(1 << bit);
        self.set_reg8(reg, res);
    }

    fn res_hl(&mut self, bit: u8, bus: &mut Bus) {
        let addr = self.registers.get_hl();
        let val = bus.read_byte(addr);
        let res = val & !(1 << bit);
        bus.write_byte(addr, res);
    }
}
