use super::*;

impl Cpu {
    /// Helper for operations that read, modify, and write back (Rotates, Shifts, RES, SET)
    fn mutate_operand<F>(&mut self, bus: &mut Bus, operand: Operand8, op: F) -> u8
    where
        F: FnOnce(&mut Self, u8) -> u8,
    {
        match operand {
            Operand8::Reg(r) => {
                let val = self.get_reg8(r);
                let res = op(self, val);
                self.set_reg8(r, res);
                8
            }
            Operand8::MemHL => {
                let addr = self.registers.get_hl();
                let val = bus.read_byte(addr);
                let res = op(self, val);
                bus.write_byte(addr, res);
                16
            }
        }
    }

    /// Helper for the BIT operation (reads only, does not write back, different cycle count)
    fn test_operand<F>(&mut self, bus: &mut Bus, operand: Operand8, op: F) -> u8
    where
        F: FnOnce(&mut Self, u8),
    {
        match operand {
            Operand8::Reg(r) => {
                let val = self.get_reg8(r);
                op(self, val);
                8
            }
            Operand8::MemHL => {
                let addr = self.registers.get_hl();
                let val = bus.read_byte(addr);
                op(self, val);
                12 // Note the 12 cycles for BIT (MemHL)
            }
        }
    }

    pub(super) fn execute_cb(&mut self, bus: &mut Bus, opcode: u8) -> u8 {
        // Decode the operand once at the top
        let operand = self.decode_bits(opcode & 0x07);
        let bit_idx = (opcode & 0x38) >> 3;

        match opcode {
            0x00..=0x07 => self.mutate_operand(bus, operand, Self::rlc_logic),
            0x08..=0x0F => self.mutate_operand(bus, operand, Self::rrc_logic),
            0x10..=0x17 => self.mutate_operand(bus, operand, Self::rl_logic),
            0x18..=0x1F => self.mutate_operand(bus, operand, Self::rr_logic),
            0x20..=0x27 => self.mutate_operand(bus, operand, Self::sla_logic),
            0x28..=0x2F => self.mutate_operand(bus, operand, Self::sra_logic),
            0x30..=0x37 => self.mutate_operand(bus, operand, Self::swap_logic),
            0x38..=0x3F => self.mutate_operand(bus, operand, Self::srl_logic),
            0x40..=0x7F => self.test_operand(bus, operand, |cpu, val| cpu.bit_logic(bit_idx, val)),
            0x80..=0xBF => self.mutate_operand(bus, operand, |_, val| val & !(1 << bit_idx)),
            0xC0..=0xFF => self.mutate_operand(bus, operand, |_, val| val | (1 << bit_idx)),
        }
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

    fn rl_logic(&mut self, val: u8) -> u8 {
        let carry = if (self.registers.f & FLAG_C) != 0 {
            1
        } else {
            0
        };
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

    fn rr_logic(&mut self, val: u8) -> u8 {
        let carry = if (self.registers.f & FLAG_C) != 0 {
            1
        } else {
            0
        };
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

    fn swap_logic(&mut self, val: u8) -> u8 {
        let res = val.rotate_right(4);

        self.set_flags(
            (res == 0).into(),
            FlagOp::Unset,
            FlagOp::Unset,
            FlagOp::Unset,
        );
        res
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

    fn bit_logic(&mut self, bit: u8, val: u8) {
        let bit_val = (val >> bit) & 0x01;

        self.set_flags(
            (bit_val == 0).into(),
            FlagOp::Unset,
            FlagOp::Set,
            FlagOp::Untouched,
        );
    }
}
