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
            _ => unreachable!("CB Opcode {:#04X} not implemented", opcode),
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
}
