use gb_core::{cpu::Cpu, cpu::FLAG_C, cpu::FLAG_H, cpu::FLAG_N, cpu::FLAG_Z, memory::Bus};

macro_rules! setup_test {
    ($data:expr) => {{
        let mut rom = vec![0x00; 0x0100];
        rom.extend_from_slice($data);
        let bus = Bus::new(rom);
        let mut cpu = Cpu::new();
        cpu.pc = 0x0100;
        cpu.registers.a = 0;
        cpu.registers.b = 0;
        cpu.registers.c = 0;
        cpu.registers.d = 0;
        cpu.registers.e = 0;
        cpu.registers.h = 0;
        cpu.registers.l = 0;
        cpu.registers.f = 0;
        cpu.sp = 0;
        (cpu, bus)
    }};
}

macro_rules! assert_flags {
    ($cpu:expr, $z:expr, $n:expr, $h:expr, $c:expr) => {
        let f = $cpu.registers.f;
        let expected =
            (($z as u8) << 7) | (($n as u8) << 6) | (($h as u8) << 5) | (($c as u8) << 4);
        assert_eq!(
            f & 0xF0,
            expected,
            "\nFlags mismatch!\nActual:   Z:{} N:{} H:{} C:{}\nExpected: Z:{} N:{} H:{} C:{}\n",
            f & FLAG_Z != 0,
            f & FLAG_N != 0,
            f & FLAG_H != 0,
            f & FLAG_C != 0,
            $z,
            $n,
            $h,
            $c
        );
    };
}

macro_rules! get_r16 {
    ($cpu:expr, bc) => {
        $cpu.registers.get_bc()
    };
    ($cpu:expr, de) => {
        $cpu.registers.get_de()
    };
    ($cpu:expr, hl) => {
        $cpu.registers.get_hl()
    };
    ($cpu:expr, af) => {
        $cpu.registers.get_af()
    };
    ($cpu:expr, sp) => {
        $cpu.sp
    };
}

macro_rules! set_r16 {
    ($cpu:expr, bc, $val:expr) => {
        $cpu.registers.set_bc($val)
    };
    ($cpu:expr, de, $val:expr) => {
        $cpu.registers.set_de($val)
    };
    ($cpu:expr, hl, $val:expr) => {
        $cpu.registers.set_hl($val)
    };
    ($cpu:expr, af, $val:expr) => {
        $cpu.registers.set_af($val)
    };
    ($cpu:expr, sp, $val:expr) => {
        $cpu.sp = $val
    };
}

macro_rules! test_ld {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $dst:ident, $src:ident, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.$src = 0x42;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.$dst, 0x42);
            assert_eq!(t, $cycles);
        }
    };
    ($(#[$attr:meta])* r8_n8, $name:ident, $opcode:expr, $dst:ident, $val:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, $val]);
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.$dst, $val);
            assert_eq!(t, $cycles);
        }
    };
    ($(#[$attr:meta])* r16_n16, $name:ident, $opcode:expr, $reg:ident, $val:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, ($val & 0xFF) as u8, ($val >> 8) as u8]);
            let t = cpu.step(&mut bus);
            assert_eq!(get_r16!(cpu, $reg), $val);
            assert_eq!(t, $cycles);
        }
    };
    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $dst:ident, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, 0x88);

            let t = cpu.step(&mut bus);

            assert_eq!(cpu.registers.$dst, 0x88);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_mem_write_r8 {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $addr_reg:ident, $src_reg:ident, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);

            cpu.registers.a = 0xAA;
            cpu.registers.b = 0xC0;
            cpu.registers.c = 0x22;
            cpu.registers.d = 0xC1;
            cpu.registers.e = 0x44;
            cpu.registers.h = 0xC2;
            cpu.registers.l = 0x00;
            cpu.sp = 0xD000;

            let addr = get_r16!(cpu, $addr_reg);
            let expected_val = cpu.registers.$src_reg;

            let t = cpu.step(&mut bus);

            assert_eq!(
                bus.read_byte(addr),
                expected_val,
                "Error: {} was not written to [{}] ({:04X})",
                stringify!($src_reg),
                stringify!($addr_reg),
                addr
            );
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_mem_read {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $addr_reg:ident, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            bus.write_byte(addr, 0xBE);
            set_r16!(cpu, $addr_reg, addr);
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, 0xBE);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_inc_dec {
    ($(#[$attr:meta])* r8, $name:ident, $opcode:expr, $reg:ident, $init:expr, $expected:expr, $z:expr, $n:expr, $h:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.$reg = $init;
            let old_f_c = cpu.registers.f & FLAG_C;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.$reg, $expected);
            assert_flags!(cpu, $z, $n, $h, old_f_c != 0);
            assert_eq!(t, $cycles);
        }
    };
    ($(#[$attr:meta])* r16, $name:ident, $opcode:expr, $reg:ident, $init:expr, $expected:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            set_r16!(cpu, $reg, $init);
            let old_f = cpu.registers.f;
            let t = cpu.step(&mut bus);
            assert_eq!(get_r16!(cpu, $reg), $expected);
            assert_eq!(cpu.registers.f, old_f);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_add {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_dest:ident, $reg_source:ident, $val_dest:expr, $val_src:expr, $expected:expr, $z:expr, $n:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.$reg_dest = $val_dest;
            if stringify!($reg_dest) != stringify!($reg_source) {
                cpu.registers.$reg_source = $val_src;
            }
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.$reg_dest, $expected);
            assert_flags!(cpu, $z, $n, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $expected:expr, $z:expr, $n:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, $n, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r16_r16, $name:ident, $opcode:expr, $src_reg:ident, $h1:expr, $h2:expr, $expected:expr, $h:expr, $c:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            set_r16!(cpu, hl, $h1);
            set_r16!(cpu, $src_reg, $h2);
            let old_z = cpu.registers.f & FLAG_Z;
            let t = cpu.step(&mut bus);
            assert_eq!(get_r16!(cpu, hl), $expected);
            assert_flags!(cpu, old_z != 0, false, $h, $c);
            assert_eq!(t, 8);
        }
    };

    ($(#[$attr:meta])* r8_n8, $name:ident, $opcode:expr, $val_a:expr, $imm:expr, $expected:expr, $z:expr, $n:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, $imm]);
            cpu.registers.a = $val_a;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, $n, $h, $c);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_adc {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_dest:ident, $reg_source:ident, $val_dest:expr, $val_src:expr, $init_c:expr, $expected:expr, $z:expr, $n:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.$reg_dest = $val_dest;
            if stringify!($reg_dest) != stringify!($reg_source) {
                cpu.registers.$reg_source = $val_src;
            }

            if $init_c { cpu.registers.f |= FLAG_C; } else { cpu.registers.f &= !FLAG_C; }

            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.$reg_dest, $expected);
            assert_flags!(cpu, $z, $n, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $init_c:expr, $expected:expr, $z:expr, $n:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;

            if $init_c { cpu.registers.f |= FLAG_C; } else { cpu.registers.f &= !FLAG_C; }

            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, $n, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_n8, $name:ident, $opcode:expr, $val_a:expr, $imm:expr, $init_c:expr, $expected:expr, $z:expr, $n:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, $imm]);
            cpu.registers.a = $val_a;

            if $init_c { cpu.registers.f |= FLAG_C; } else { cpu.registers.f &= !FLAG_C; }

            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, $n, $h, $c);
            assert_eq!(t, $cycles);
        }
    };
}
macro_rules! test_sub {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_source:ident, $val_a:expr, $val_src:expr, $expected:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.a = $val_a;
            if stringify!($reg_source) != "a" {
                cpu.registers.$reg_source = $val_src;
            }
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $expected:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_n8, $name:ident, $opcode:expr, $val_a:expr, $val_imm:expr, $expected:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, $val_imm]);
            cpu.registers.a = $val_a;

            let t = cpu.step(&mut bus);

            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_sbc {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_source:ident, $val_a:expr, $val_src:expr, $init_c:expr, $expected:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.a = $val_a;
            if stringify!($reg_source) != "a" {
                cpu.registers.$reg_source = $val_src;
            }

            if $init_c { cpu.registers.f |= FLAG_C; } else { cpu.registers.f &= !FLAG_C; }

            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $init_c:expr, $expected:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;

            if $init_c { cpu.registers.f |= FLAG_C; } else { cpu.registers.f &= !FLAG_C; }

            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_n8, $name:ident, $opcode:expr, $val_a:expr, $val_imm:expr, $init_c:expr, $expected:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, $val_imm]);
            cpu.registers.a = $val_a;

            if $init_c { cpu.registers.f |= FLAG_C; } else { cpu.registers.f &= !FLAG_C; }

            let t = cpu.step(&mut bus);

            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_and {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_source:ident, $val_a:expr, $val_src:expr, $expected:expr, $z:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.a = $val_a;
            if stringify!($reg_source) != "a" {
                cpu.registers.$reg_source = $val_src;
            }
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, false, true, false);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $expected:expr, $z:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, false, true, false);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_xor {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_source:ident, $val_a:expr, $val_src:expr, $expected:expr, $z:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.a = $val_a;
            if stringify!($reg_source) != "a" {
                cpu.registers.$reg_source = $val_src;
            }
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, false, false, false);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $expected:expr, $z:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, false, false, false);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_or {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_source:ident, $val_a:expr, $val_src:expr, $expected:expr, $z:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.a = $val_a;
            if stringify!($reg_source) != "a" {
                cpu.registers.$reg_source = $val_src;
            }
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, false, false, false);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $expected:expr, $z:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $expected);
            assert_flags!(cpu, $z, false, false, false);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_cp {
    ($(#[$attr:meta])* r8_r8, $name:ident, $opcode:expr, $reg_source:ident, $val_a:expr, $val_src:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.registers.a = $val_a;
            if stringify!($reg_source) != "a" {
                cpu.registers.$reg_source = $val_src;
            }
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $val_a);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };

    ($(#[$attr:meta])* r8_hl_mem, $name:ident, $opcode:expr, $val_a:expr, $val_mem:expr, $z:expr, $h:expr, $c:expr, $cycles:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            let addr = 0xC000;
            set_r16!(cpu, hl, addr);
            bus.write_byte(addr, $val_mem);
            cpu.registers.a = $val_a;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.registers.a, $val_a);
            assert_flags!(cpu, $z, true, $h, $c);
            assert_eq!(t, $cycles);
        }
    };
}

macro_rules! test_jr {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $offset:expr, $expected_pc_offset:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, $offset as u8]);
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.pc, (0x0102u16).wrapping_add_signed($expected_pc_offset as i16));
            assert_eq!(t, 12);
        }
    };
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $cond_flag:expr, $cond_state:expr, $offset:expr, $should_jump:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, $offset as u8]);
            if $cond_state { cpu.registers.f |= $cond_flag; } else { cpu.registers.f &= !$cond_flag; }
            let t = cpu.step(&mut bus);
            if $should_jump {
                assert_eq!(cpu.pc, (0x0102u16).wrapping_add_signed($offset as i16));
                assert_eq!(t, 12);
            } else {
                assert_eq!(cpu.pc, 0x0102);
                assert_eq!(t, 8);
            }
        }
    };
}

macro_rules! test_jp {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $dest:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, ($dest & 0xFF) as u8, ($dest >> 8) as u8]);
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.pc, $dest);
            assert_eq!(t, 16);
        }
    };
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $cond_flag:expr, $cond_state:expr, $dest:expr, $should_jump:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, ($dest & 0xFF) as u8, ($dest >> 8) as u8]);
            if $cond_state { cpu.registers.f |= $cond_flag; } else { cpu.registers.f &= !$cond_flag; }
            let t = cpu.step(&mut bus);
            if $should_jump {
                assert_eq!(cpu.pc, $dest);
                assert_eq!(t, 16);
            } else {
                assert_eq!(cpu.pc, 0x0103);
                assert_eq!(t, 12);
            }
        }
    };
}

macro_rules! test_ret {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $cond_flag:expr, $cond_state:expr, $should_jump:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.sp = 0xFFFC;
            bus.write_byte(0xFFFC, 0x34);
            bus.write_byte(0xFFFD, 0x12);

            if $cond_state { cpu.registers.f |= $cond_flag; } else { cpu.registers.f &= !$cond_flag; }

            let t = cpu.step(&mut bus);

            if $should_jump {
                assert_eq!(cpu.pc, 0x1234);
                assert_eq!(cpu.sp, 0xFFFE);
                assert_eq!(t, 20);
            } else {
                assert_eq!(cpu.pc, 0x0101);
                assert_eq!(cpu.sp, 0xFFFC);
                assert_eq!(t, 8);
            }
        }
    };

    ($(#[$attr:meta])* $name:ident, $opcode:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.sp = 0xFFFC;
            bus.write_byte(0xFFFC, 0x34);
            bus.write_byte(0xFFFD, 0x12);

            let t = cpu.step(&mut bus);

            assert_eq!(cpu.pc, 0x1234);
            assert_eq!(cpu.sp, 0xFFFE);
            assert_eq!(t, 16);
        }
    };
}

macro_rules! test_pop {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $reg:ident, $val:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);

            cpu.sp = 0xD000;
            bus.write_byte(0xD000, ($val & 0xFF) as u8);
            bus.write_byte(0xD001, (($val >> 8) & 0xFF) as u8);

            let t = cpu.step(&mut bus);

            assert_eq!(
                get_r16!(cpu, $reg),
                $val & (if stringify!($reg) == "af" {
                    0xFFF0
                } else {
                    0xFFFF
                })
            );

            assert_eq!(cpu.sp, 0xD002);
            assert_eq!(t, 12);
        }
    };
}

macro_rules! test_push {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $reg:ident, $val:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            set_r16!(cpu, $reg, $val);
            cpu.sp = 0xE000;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.sp, 0xDFFE);
            assert_eq!(bus.read_byte(0xDFFF), (($val >> 8) & 0xFF) as u8);
            assert_eq!(bus.read_byte(0xDFFE), ($val & 0xFF) as u8);
            assert_eq!(t, 16);
        }
    };
}

macro_rules! test_call {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $dest:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, ($dest & 0xFF) as u8, ($dest >> 8) as u8]);

            // Set SP to top of WRAM instead of HRAM
            cpu.sp = 0xE000;

            let t = cpu.step(&mut bus);

            assert_eq!(cpu.pc, $dest);
            assert_eq!(cpu.sp, 0xDFFE); // Decremented by 2

            // Return address is PC + 3 (0x0100 + 3 = 0x0103)
            assert_eq!(bus.read_byte(0xDFFF), 0x01); // High byte of 0x0103
            assert_eq!(bus.read_byte(0xDFFE), 0x03); // Low byte of 0x0103
            assert_eq!(t, 24);
        }
    };

    ($(#[$attr:meta])* $name:ident, $opcode:expr, $cond_flag:expr, $cond_state:expr, $dest:expr, $should_jump:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode, ($dest & 0xFF) as u8, ($dest >> 8) as u8]);

            // Set SP to top of WRAM
            cpu.sp = 0xE000;

            if $cond_state { cpu.registers.f |= $cond_flag; } else { cpu.registers.f &= !$cond_flag; }
            let t = cpu.step(&mut bus);

            if $should_jump {
                assert_eq!(cpu.pc, $dest);
                assert_eq!(cpu.sp, 0xDFFE); // Decremented by 2

                assert_eq!(bus.read_byte(0xDFFF), 0x01);
                assert_eq!(bus.read_byte(0xDFFE), 0x03);
                assert_eq!(t, 24);
            } else {
                assert_eq!(cpu.pc, 0x0103);
                assert_eq!(cpu.sp, 0xE000); // SP remains unchanged if not jumping
                assert_eq!(t, 12);
            }
        }
    };
}

macro_rules! test_rst {
    ($(#[$attr:meta])* $name:ident, $opcode:expr, $dest:expr) => {
        $(#[$attr])* #[test]
        fn $name() {
            let (mut cpu, mut bus) = setup_test!(&[$opcode]);
            cpu.sp = 0xE000;
            let t = cpu.step(&mut bus);
            assert_eq!(cpu.pc, $dest);
            assert_eq!(cpu.sp, 0xDFFE);
            assert_eq!(bus.read_byte(0xDFFF), 0x01); // High
            assert_eq!(bus.read_byte(0xDFFE), 0x01); // Low
            assert_eq!(t, 16);
        }
    };
}

#[path = "cpu_tests/0.rs"]
mod group_0;

#[path = "cpu_tests/1.rs"]
mod group_1;

#[path = "cpu_tests/2.rs"]
mod group_2;

#[path = "cpu_tests/3.rs"]
mod group_3;

#[path = "cpu_tests/4.rs"]
mod group_4;

#[path = "cpu_tests/5.rs"]
mod group_5;

#[path = "cpu_tests/6.rs"]
mod group_6;

#[path = "cpu_tests/7.rs"]
mod group_7;

#[path = "cpu_tests/8.rs"]
mod group_8;

#[path = "cpu_tests/9.rs"]
mod group_9;

#[path = "cpu_tests/A.rs"]
mod group_a;

#[path = "cpu_tests/B.rs"]
mod group_b;

#[path = "cpu_tests/C.rs"]
mod group_c;

#[path = "cpu_tests/D.rs"]
mod group_d;

#[path = "cpu_tests/E.rs"]
mod group_e;

#[path = "cpu_tests/F.rs"]
mod group_f;
