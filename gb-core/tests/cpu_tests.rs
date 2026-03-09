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

#[test]
fn test_0x00_nop() {
    let (mut cpu, mut bus) = setup_test!(&[0x00]);
    assert_eq!(cpu.step(&mut bus), 4);
}

test_ld!(r16_n16, test_0x01_ld_bc_n16, 0x01, bc, 0x1234, 12);
test_mem_write_r8!(test_0x02_ld_bc_a, 0x02, bc, a, 8);
test_inc_dec!(r16, test_0x03_inc_bc, 0x03, bc, 0x00FF, 0x0100, 8);
test_inc_dec!(
    r8,
    test_0x04_inc_b,
    0x04,
    b,
    0x0F,
    0x10,
    false,
    false,
    true,
    4
);
test_inc_dec!(
    r8,
    test_0x05_dec_b,
    0x05,
    b,
    0x01,
    0x00,
    true,
    true,
    false,
    4
);
test_ld!(r8_n8, test_0x06_ld_b_n8, 0x06, b, 0x77, 8);

#[test]
fn test_0x07_rlca() {
    let (mut cpu, mut bus) = setup_test!(&[0x07]);
    cpu.registers.a = 0x80;
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0x01);
    assert_flags!(cpu, false, false, false, true);
}

#[test]
fn test_0x08_ld_a16_mem_sp() {
    let (mut cpu, mut bus) = setup_test!(&[0x08, 0x00, 0xC0]);
    cpu.sp = 0xABCD;
    let t = cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xC000), 0xCD);
    assert_eq!(bus.read_byte(0xC001), 0xAB);
    assert_eq!(t, 20);
}

test_add!(
    r16_r16,
    test_0x09_add_hl_bc,
    0x09,
    bc,
    0x1000,
    0x1000,
    0x2000,
    false,
    false
);
test_mem_read!(test_0x0a_ld_a_bc, 0x0A, bc, 8);
test_inc_dec!(r16, test_0x0b_dec_bc, 0x0B, bc, 0x0001, 0x0000, 8);
test_inc_dec!(
    r8,
    test_0x0c_inc_c,
    0x0C,
    c,
    0xFF,
    0x00,
    true,
    false,
    true,
    4
);
test_inc_dec!(
    r8,
    test_0x0d_dec_c,
    0x0D,
    c,
    0x00,
    0xFF,
    false,
    true,
    true,
    4
);
test_ld!(r8_n8, test_0x0e_ld_c_n8, 0x0E, c, 0x12, 8);

#[test]
fn test_0x0f_rrca() {
    let (mut cpu, mut bus) = setup_test!(&[0x0F]);
    cpu.registers.a = 0x01;
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0x80);
    assert_flags!(cpu, false, false, false, true);
}
#[test]
#[should_panic(expected = "WIP")]
fn test_0x10_stop_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x10, 0x00]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
}

test_ld!(r16_n16, test_0x11_ld_de_n16, 0x11, de, 0x8000, 12);
test_mem_write_r8!(test_0x12_ld_de_a, 0x12, de, a, 8);
test_inc_dec!(r16, test_0x13_inc_de, 0x13, de, 0xFFFF, 0x0000, 8);
test_inc_dec!(
    r8,
    test_0x14_inc_d,
    0x14,
    d,
    0x00,
    0x01,
    false,
    false,
    false,
    4
);
test_inc_dec!(
    r8,
    test_0x15_dec_d,
    0x15,
    d,
    0x10,
    0x0F,
    false,
    true,
    true,
    4
);
test_ld!(r8_n8, test_0x16_ld_d_n8, 0x16, d, 0x33, 8);

#[test]
fn test_0x17_rla() {
    let (mut cpu, mut bus) = setup_test!(&[0x17]);
    cpu.registers.a = 0x80;
    cpu.registers.f = FLAG_C;
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0x01);
    assert_flags!(cpu, false, false, false, true);
}

test_jr!(test_0x18_jr_e8_forward, 0x18, 0x05, 5);
test_jr!(test_0x18_jr_e8_backward, 0x18, -5i8, -5);
test_add!(
    r16_r16,
    test_0x19_add_hl_de,
    0x19,
    de,
    0x0F00,
    0x0100,
    0x1000,
    true,
    false
);
test_mem_read!(test_0x1a_ld_a_de, 0x1A, de, 8);
test_inc_dec!(r16, test_0x1b_dec_de, 0x1B, de, 0x0001, 0x0000, 8);
test_inc_dec!(
    r8,
    test_0x1c_inc_e,
    0x1C,
    e,
    0x0F,
    0x10,
    false,
    false,
    true,
    4
);
test_inc_dec!(
    r8,
    test_0x1d_dec_e,
    0x1D,
    e,
    0x01,
    0x00,
    true,
    true,
    false,
    4
);
test_ld!(r8_n8, test_0x1e_ld_e_n8, 0x1E, e, 0x99, 8);

#[test]
fn test_0x1f_rra() {
    let (mut cpu, mut bus) = setup_test!(&[0x1F]);
    cpu.registers.a = 0x01;
    cpu.registers.f = FLAG_C;
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0x80);
    assert_flags!(cpu, false, false, false, true);
}

test_jr!(test_0x20_jr_nz_jump, 0x20, FLAG_Z, false, 0x0A, true);
test_jr!(test_0x20_jr_nz_no_jump, 0x20, FLAG_Z, true, 0x0A, false);
test_ld!(r16_n16, test_0x21_ld_hl_n16, 0x21, hl, 0xD000, 12);

#[test]
fn test_0x22_ld_hl_inc_mem_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x22]);
    cpu.registers.set_hl(0xC000);
    cpu.registers.a = 0x55;
    cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xC000), 0x55);
    assert_eq!(cpu.registers.get_hl(), 0xC001);
}

test_inc_dec!(r16, test_0x23_inc_hl, 0x23, hl, 0x4444, 0x4445, 8);
test_inc_dec!(
    r8,
    test_0x24_inc_h,
    0x24,
    h,
    0x00,
    0x01,
    false,
    false,
    false,
    4
);
test_inc_dec!(
    r8,
    test_0x25_dec_h,
    0x25,
    h,
    0x00,
    0xFF,
    false,
    true,
    true,
    4
);
test_ld!(r8_n8, test_0x26_ld_h_n8, 0x26, h, 0xFE, 8);

#[test]
fn test_0x27_daa() {
    let test_cases = [
        ((0x99, false, false, false), (0x99, false)),
        ((0x0B, false, false, false), (0x11, false)),
        ((0xA0, false, false, false), (0x00, true)),
        ((0x9A, false, false, false), (0x00, true)),
        ((0x02, false, true, false), (0x08, false)),
        ((0x90, false, false, true), (0xF0, true)),
        ((0x05, false, true, true), (0x6B, true)),
        ((0x99, true, false, false), (0x99, false)),
        ((0x05, true, true, false), (0xFF, false)),
        ((0x40, true, false, true), (0xE0, true)),
        ((0x22, true, true, true), (0xBC, true)),
        ((0x00, false, false, false), (0x00, false)),
        ((0x7A, false, false, false), (0x80, false)),
    ];

    for ((start_a, n, h, c), (exp_a, exp_c)) in test_cases {
        let (mut cpu, mut bus) = setup_test!(&[0x27]);

        cpu.registers.a = start_a;
        cpu.registers.f = 0;
        if n {
            cpu.registers.f |= FLAG_N;
        }
        if h {
            cpu.registers.f |= FLAG_H;
        }
        if c {
            cpu.registers.f |= FLAG_C;
        }

        cpu.step(&mut bus);

        let z_expected = exp_a == 0;

        assert_eq!(
            cpu.registers.a, exp_a,
            "DAA Failed! Input A: {:02X}, N:{}, H:{}, C:{}",
            start_a, n, h, c
        );

        assert_flags!(cpu, z_expected, n, false, exp_c);
    }
}

test_jr!(test_0x28_jr_z_jump, 0x28, FLAG_Z, true, 0x0A, true);
test_jr!(test_0x28_jr_z_no_jump, 0x28, FLAG_Z, false, 0x0A, false);
test_add!(
    r16_r16,
    test_0x29_add_hl_hl,
    0x29,
    hl,
    0x4000,
    0x4000,
    0x8000,
    false,
    false
);

#[test]
fn test_0x2a_ld_a_hl_inc_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x2A]);
    bus.write_byte(0xC000, 0x77);
    cpu.registers.set_hl(0xC000);
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0x77);
    assert_eq!(cpu.registers.get_hl(), 0xC001);
}

test_inc_dec!(r16, test_0x2b_dec_hl, 0x2B, hl, 0x0000, 0xFFFF, 8);
test_inc_dec!(
    r8,
    test_0x2c_inc_l,
    0x2C,
    l,
    0x0F,
    0x10,
    false,
    false,
    true,
    4
);
test_inc_dec!(
    r8,
    test_0x2d_dec_l,
    0x2D,
    l,
    0x10,
    0x0F,
    false,
    true,
    true,
    4
);
test_ld!(r8_n8, test_0x2e_ld_l_n8, 0x2E, l, 0x55, 8);

#[test]
fn test_0x2f_cpl() {
    let (mut cpu, mut bus) = setup_test!(&[0x2F]);
    cpu.registers.a = 0xAA;
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0x55);
    assert_flags!(cpu, false, true, true, false);
}

test_jr!(test_0x30_jr_nc_jump, 0x30, FLAG_C, false, 0x0A, true);
test_jr!(test_0x30_jr_nc_no_jump, 0x30, FLAG_C, true, 0x0A, false);
test_ld!(r16_n16, test_0x31_ld_sp_n16, 0x31, sp, 0xDFFF, 12);

#[test]
fn test_0x32_ld_hl_dec_mem_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x32]);
    cpu.registers.set_hl(0xC005);
    cpu.registers.a = 0x99;
    cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xC005), 0x99);
    assert_eq!(cpu.registers.get_hl(), 0xC004);
}

test_inc_dec!(r16, test_0x33_inc_sp, 0x33, sp, 0xFFFF, 0x0000, 8);

#[test]
fn test_0x34_inc_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x34]);
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x0F);
    cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xC000), 0x10);
    assert_flags!(cpu, false, false, true, false);
}

#[test]
fn test_0x35_dec_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x35]);
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x01);
    cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xC000), 0x00);
    assert_flags!(cpu, true, true, false, false);
}

#[test]
fn test_0x36_ld_hl_mem_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x36, 0x42]);
    cpu.registers.set_hl(0xC000);
    cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xC000), 0x42);
}

#[test]
fn test_0x37_scf() {
    let (mut cpu, mut bus) = setup_test!(&[0x37]);
    cpu.registers.f = FLAG_N | FLAG_H;
    cpu.step(&mut bus);
    assert_flags!(cpu, false, false, false, true);
}

test_jr!(test_0x38_jr_c_jump, 0x38, FLAG_C, true, 0x0A, true);
test_jr!(test_0x38_jr_c_no_jump, 0x38, FLAG_C, false, 0x0A, false);
test_add!(
    r16_r16,
    test_0x39_add_hl_sp,
    0x39,
    sp,
    0x0001,
    0x0001,
    0x0002,
    false,
    false
);
test_mem_read!(test_0x3a_ld_a_hld, 0x3A, hl, 8);
test_inc_dec!(r16, test_0x3b_dec_sp, 0x3B, sp, 0x0000, 0xFFFF, 8);
test_inc_dec!(
    r8,
    test_0x3c_inc_a,
    0x3C,
    a,
    0xFF,
    0x00,
    true,
    false,
    true,
    4
);
test_inc_dec!(
    r8,
    test_0x3d_dec_a,
    0x3D,
    a,
    0x10,
    0x0F,
    false,
    true,
    true,
    4
);
test_ld!(r8_n8, test_0x3e_ld_a_n8, 0x3E, a, 0x42, 8);

#[test]
fn test_0x3f_ccf() {
    let (mut cpu, mut bus) = setup_test!(&[0x3F]);
    cpu.registers.f = FLAG_C;
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.f & FLAG_C, 0);
}

test_ld!(r8_r8, test_0x40_ld_b_b, 0x40, b, b, 4);
test_ld!(r8_r8, test_0x41_ld_b_c, 0x41, b, c, 4);
test_ld!(r8_r8, test_0x42_ld_b_d, 0x42, b, d, 4);
test_ld!(r8_r8, test_0x43_ld_b_e, 0x43, b, e, 4);
test_ld!(r8_r8, test_0x44_ld_b_h, 0x44, b, h, 4);
test_ld!(r8_r8, test_0x45_ld_b_l, 0x45, b, l, 4);
test_ld!(r8_hl_mem, test_0x46_ld_b_hl, 0x46, b, 8);
test_ld!(r8_r8, test_0x47_ld_b_a, 0x47, b, a, 4);
test_ld!(r8_r8, test_0x48_ld_c_b, 0x48, c, b, 4);
test_ld!(r8_r8, test_0x49_ld_c_c, 0x49, c, c, 4);
test_ld!(r8_r8, test_0x4a_ld_c_d, 0x4A, c, d, 4);
test_ld!(r8_r8, test_0x4b_ld_c_e, 0x4B, c, e, 4);
test_ld!(r8_r8, test_0x4c_ld_c_h, 0x4C, c, h, 4);
test_ld!(r8_r8, test_0x4d_ld_c_l, 0x4D, c, l, 4);
test_ld!(r8_hl_mem, test_0x4e_ld_c_hl, 0x4E, c, 8);
test_ld!(r8_r8, test_0x4f_ld_c_a, 0x4F, c, a, 4);
test_ld!(r8_r8, test_0x50_ld_d_b, 0x50, d, b, 4);
test_ld!(r8_r8, test_0x51_ld_d_c, 0x51, d, c, 4);
test_ld!(r8_r8, test_0x52_ld_d_d, 0x52, d, d, 4);
test_ld!(r8_r8, test_0x53_ld_d_e, 0x53, d, e, 4);
test_ld!(r8_r8, test_0x54_ld_d_h, 0x54, d, h, 4);
test_ld!(r8_r8, test_0x55_ld_d_l, 0x55, d, l, 4);
test_ld!(r8_hl_mem, test_0x56_ld_d_hl, 0x56, d, 8);
test_ld!(r8_r8, test_0x57_ld_d_a, 0x57, d, a, 4);
test_ld!(r8_r8, test_0x58_ld_e_b, 0x58, e, b, 4);
test_ld!(r8_r8, test_0x59_ld_e_c, 0x59, e, c, 4);
test_ld!(r8_r8, test_0x5a_ld_e_d, 0x5A, e, d, 4);
test_ld!(r8_r8, test_0x5b_ld_e_e, 0x5B, e, e, 4);
test_ld!(r8_r8, test_0x5c_ld_e_h, 0x5C, e, h, 4);
test_ld!(r8_r8, test_0x5d_ld_e_l, 0x5D, e, l, 4);
test_ld!(r8_hl_mem, test_0x5e_ld_e_hl, 0x5E, e, 8);
test_ld!(r8_r8, test_0x5f_ld_e_a, 0x5F, e, a, 4);
test_ld!(r8_r8, test_0x60_ld_h_b, 0x60, h, b, 4);
test_ld!(r8_r8, test_0x61_ld_h_c, 0x61, h, c, 4);
test_ld!(r8_r8, test_0x62_ld_h_d, 0x62, h, d, 4);
test_ld!(r8_r8, test_0x63_ld_h_e, 0x63, h, e, 4);
test_ld!(r8_r8, test_0x64_ld_h_h, 0x64, h, h, 4);
test_ld!(r8_r8, test_0x65_ld_h_l, 0x65, h, l, 4);
test_ld!(r8_hl_mem, test_0x66_ld_h_hl, 0x66, h, 8);
test_ld!(r8_r8, test_0x67_ld_h_a, 0x67, h, a, 4);
test_ld!(r8_r8, test_0x68_ld_l_b, 0x68, l, b, 4);
test_ld!(r8_r8, test_0x69_ld_l_c, 0x69, l, c, 4);
test_ld!(r8_r8, test_0x6a_ld_l_d, 0x6A, l, d, 4);
test_ld!(r8_r8, test_0x6b_ld_l_e, 0x6B, l, e, 4);
test_ld!(r8_r8, test_0x6c_ld_l_h, 0x6C, l, h, 4);
test_ld!(r8_r8, test_0x6d_ld_l_l, 0x6D, l, l, 4);
test_ld!(r8_hl_mem, test_0x6e_ld_l_hl, 0x6E, l, 8);
test_ld!(r8_r8, test_0x6f_ld_l_a, 0x6F, l, a, 4);
test_mem_write_r8!(test_0x70_ld_hl_mem_b, 0x70, hl, b, 8);
test_mem_write_r8!(test_0x71_ld_hl_mem_c, 0x71, hl, c, 8);
test_mem_write_r8!(test_0x72_ld_hl_mem_d, 0x72, hl, d, 8);
test_mem_write_r8!(test_0x73_ld_hl_mem_e, 0x73, hl, e, 8);
test_mem_write_r8!(test_0x74_ld_hl_mem_h, 0x74, hl, h, 8);
test_mem_write_r8!(test_0x75_ld_hl_mem_l, 0x75, hl, l, 8);

#[test]
#[should_panic(expected = "WIP")]
fn test_0x76_halt() {
    let (mut cpu, mut bus) = setup_test!(&[0x76]);
    cpu.step(&mut bus);
}

test_mem_write_r8!(test_0x77_ld_hl_mem_a, 0x77, hl, a, 8);
test_ld!(r8_r8, test_0x78_ld_a_b, 0x78, a, b, 4);
test_ld!(r8_r8, test_0x79_ld_a_c, 0x79, a, c, 4);
test_ld!(r8_r8, test_0x7a_ld_a_d, 0x7A, a, d, 4);
test_ld!(r8_r8, test_0x7b_ld_a_e, 0x7B, a, e, 4);
test_ld!(r8_r8, test_0x7c_ld_a_h, 0x7C, a, h, 4);
test_ld!(r8_r8, test_0x7d_ld_a_l, 0x7D, a, l, 4);
test_ld!(r8_hl_mem, test_0x7e_ld_a_hl, 0x7E, a, 8);
test_ld!(r8_r8, test_0x7f_ld_a_a, 0x7F, a, a, 4);
test_add!(
    r8_r8,
    test_0x80_add_a_b,
    0x80,
    a,
    b,
    0x30,
    0x12,
    0x42,
    false,
    false,
    false,
    false,
    4
);
test_add!(
    r8_r8,
    test_0x81_add_a_c_zero,
    0x81,
    a,
    c,
    0x00,
    0x00,
    0x00,
    true,
    false,
    false,
    false,
    4
);
test_add!(
    r8_r8,
    test_0x82_add_a_d_hcarry,
    0x82,
    a,
    d,
    0x0F,
    0x01,
    0x10,
    false,
    false,
    true,
    false,
    4
);
test_add!(
    r8_r8,
    test_0x83_add_a_e_carry,
    0x83,
    a,
    e,
    0xFF,
    0x01,
    0x00,
    true,
    false,
    true,
    true,
    4
);
test_add!(
    r8_r8,
    test_0x84_add_a_h_hc,
    0x84,
    a,
    h,
    0x88,
    0x88,
    0x10,
    false,
    false,
    true,
    true,
    4
);
test_add!(
    r8_r8,
    test_0x85_add_a_l,
    0x85,
    a,
    l,
    0x10,
    0x20,
    0x30,
    false,
    false,
    false,
    false,
    4
);
test_add!(
    r8_hl_mem,
    test_0x86_add_a_hl,
    0x86,
    0x40,
    0x40,
    0x80,
    false,
    false,
    false,
    false,
    8
);
test_add!(
    r8_r8,
    test_0x87_add_a_a,
    0x87,
    a,
    a,
    0x10,
    0x10,
    0x20,
    false,
    false,
    false,
    false,
    4
);
test_adc!(
    r8_r8,
    test_0x88_adc_a_b,
    0x88,
    a,
    b,
    0x10,
    0x20,
    false,
    0x30,
    false,
    false,
    false,
    false,
    4
);
test_adc!(
    r8_r8,
    test_0x89_adc_a_c,
    0x89,
    a,
    c,
    0x10,
    0x20,
    true,
    0x31,
    false,
    false,
    false,
    false,
    4
);
test_adc!(
    r8_r8,
    test_0x8a_adc_a_d,
    0x8A,
    a,
    d,
    0x00,
    0x00,
    false,
    0x00,
    true,
    false,
    false,
    false,
    4
);
test_adc!(
    r8_r8,
    test_0x8b_adc_a_e,
    0x8B,
    a,
    e,
    0x0F,
    0x00,
    true,
    0x10,
    false,
    false,
    true,
    false,
    4
);
test_adc!(
    r8_r8,
    test_0x8c_adc_a_h,
    0x8C,
    a,
    h,
    0xFF,
    0x00,
    true,
    0x00,
    true,
    false,
    true,
    true,
    4
);
test_adc!(
    r8_r8,
    test_0x8d_adc_a_l,
    0x8D,
    a,
    l,
    0x80,
    0x7F,
    true,
    0x00,
    true,
    false,
    true,
    true,
    4
);
test_adc!(
    r8_hl_mem,
    test_0x8e_adc_a_hl_mem,
    0x8E,
    0x05,
    0x05,
    true,
    0x0B,
    false,
    false,
    false,
    false,
    8
);
test_adc!(
    r8_r8,
    test_0x8f_adc_a_a,
    0x8F,
    a,
    a,
    0x40,
    0x40,
    true,
    0x81,
    false,
    false,
    false,
    false,
    4
);
test_sub!(
    r8_r8,
    test_0x90_sub_a_b,
    0x90,
    b,
    0x0A,
    0x03,
    0x07,
    false,
    false,
    false,
    4
);
test_sub!(
    r8_r8,
    test_0x91_sub_a_c,
    0x91,
    c,
    0x10,
    0x01,
    0x0F,
    false,
    true,
    false,
    4
);
test_sub!(
    r8_r8,
    test_0x92_sub_a_d,
    0x92,
    d,
    0x00,
    0x01,
    0xFF,
    false,
    true,
    true,
    4
);
test_sub!(
    r8_r8,
    test_0x93_sub_a_e,
    0x93,
    e,
    0x05,
    0x05,
    0x00,
    true,
    false,
    false,
    4
);
test_sub!(
    r8_r8,
    test_0x94_sub_a_h,
    0x94,
    h,
    0x0A,
    0x03,
    0x07,
    false,
    false,
    false,
    4
);
test_sub!(
    r8_r8,
    test_0x95_sub_a_l,
    0x95,
    l,
    0x0A,
    0x03,
    0x07,
    false,
    false,
    false,
    4
);
test_sub!(
    r8_hl_mem,
    test_0x96_sub_a_hl,
    0x96,
    0x0A,
    0x03,
    0x07,
    false,
    false,
    false,
    8
);
test_sub!(
    r8_r8,
    test_0x97_sub_a_a,
    0x97,
    a,
    0x42,
    0x42,
    0x00,
    true,
    false,
    false,
    4
);
test_sbc!(
    r8_r8,
    test_0x98_sbc_a_b,
    0x98,
    b,
    0x05,
    0x02,
    true,
    0x02,
    false,
    false,
    false,
    4
);

test_sbc!(
    r8_r8,
    test_0x99_sbc_a_c_h_carry,
    0x99,
    c,
    0x10,
    0x01,
    true,
    0x0E,
    false,
    true,
    false,
    4
);

test_sbc!(
    r8_r8,
    test_0x9a_sbc_a_d_carry,
    0x9A,
    d,
    0x00,
    0x01,
    false,
    0xFF,
    false,
    true,
    true,
    4
);

test_sbc!(
    r8_r8,
    test_0x9b_sbc_a_e_carry_h_carry,
    0x9B,
    e,
    0x05,
    0x05,
    true,
    0xFF,
    false,
    true,
    true,
    4
);

test_sbc!(
    r8_r8,
    test_0x9c_sbc_a_h_zero,
    0x9C,
    h,
    0x05,
    0x05,
    false,
    0x00,
    true,
    false,
    false,
    4
);

test_sbc!(
    r8_r8,
    test_0x9d_sbc_a_l,
    0x9D,
    l,
    0x10,
    0x11,
    false,
    0xFF,
    false,
    true,
    true,
    4
);

test_sbc!(
    r8_hl_mem,
    test_0x9e_sbc_a_hl_mem,
    0x9E,
    0x40,
    0x20,
    true,
    0x1F,
    false,
    true,
    false,
    8
);

test_sbc!(
    r8_r8,
    test_0x9f_sbc_a_a,
    0x9F,
    a,
    0x42,
    0x42,
    true,
    0xFF,
    false,
    true,
    true,
    4
);
test_and!(
    r8_r8,
    test_0xa0_and_a_b,
    0xA0,
    b,
    0xFF,
    0x0F,
    0x0F,
    false,
    4
);
test_and!(
    r8_r8,
    test_0xa1_and_a_c_zero,
    0xA1,
    c,
    0x0F,
    0xF0,
    0x00,
    true,
    4
);
test_and!(
    r8_r8,
    test_0xa2_and_a_d_zero,
    0xA2,
    d,
    0xAA,
    0x55,
    0x00,
    true,
    4
);
test_and!(
    r8_r8,
    test_0xa3_and_a_e,
    0xA3,
    e,
    0x55,
    0xFF,
    0x55,
    false,
    4
);
test_and!(
    r8_r8,
    test_0xa4_and_a_h,
    0xA4,
    h,
    0x33,
    0x11,
    0x11,
    false,
    4
);
test_and!(
    r8_r8,
    test_0xa5_and_a_l_zero,
    0xA5,
    l,
    0x00,
    0xFF,
    0x00,
    true,
    4
);
test_and!(
    r8_hl_mem,
    test_0xa6_and_a_hl_mem,
    0xA6,
    0xFF,
    0x42,
    0x42,
    false,
    8
);
test_and!(
    r8_r8,
    test_0xa7_and_a_a,
    0xA7,
    a,
    0x12,
    0x12,
    0x12,
    false,
    4
);
test_xor!(
    r8_r8,
    test_0xa8_xor_a_b,
    0xA8,
    b,
    0xFF,
    0x0F,
    0xF0,
    false,
    4
);
test_xor!(
    r8_r8,
    test_0xa9_xor_a_c_zero,
    0xA9,
    c,
    0x0F,
    0x0F,
    0x00,
    true,
    4
);
test_xor!(
    r8_r8,
    test_0xaa_xor_a_d,
    0xAA,
    d,
    0xAA,
    0x55,
    0xFF,
    false,
    4
);
test_xor!(
    r8_r8,
    test_0xab_xor_a_e,
    0xAB,
    e,
    0x55,
    0xFF,
    0xAA,
    false,
    4
);
test_xor!(
    r8_r8,
    test_0xac_xor_a_h,
    0xAC,
    h,
    0x33,
    0x11,
    0x22,
    false,
    4
);
test_xor!(
    r8_r8,
    test_0xad_xor_a_l,
    0xAD,
    l,
    0x00,
    0xFF,
    0xFF,
    false,
    4
);
test_xor!(
    r8_hl_mem,
    test_0xae_xor_a_hl_mem,
    0xAE,
    0xFF,
    0x42,
    0xBD,
    false,
    8
);
test_xor!(
    r8_r8,
    test_0xaf_xor_a_a_zero,
    0xAF,
    a,
    0x12,
    0x12,
    0x00,
    true,
    4
);
test_or!(r8_r8, test_0xb0_or_a_b, 0xB0, b, 0xF0, 0x0F, 0xFF, false, 4);
test_or!(
    r8_r8,
    test_0xb1_or_a_c_zero,
    0xB1,
    c,
    0x00,
    0x00,
    0x00,
    true,
    4
);
test_or!(r8_r8, test_0xb2_or_a_d, 0xB2, d, 0xAA, 0x55, 0xFF, false, 4);
test_or!(r8_r8, test_0xb3_or_a_e, 0xB3, e, 0x05, 0x0A, 0x0F, false, 4);
test_or!(r8_r8, test_0xb4_or_a_h, 0xB4, h, 0x33, 0x11, 0x33, false, 4);
test_or!(r8_r8, test_0xb5_or_a_l, 0xB5, l, 0x00, 0xFF, 0xFF, false, 4);
test_or!(
    r8_hl_mem,
    test_0xb6_or_a_hl_mem,
    0xB6,
    0x01,
    0x42,
    0x43,
    false,
    8
);
test_or!(r8_r8, test_0xb7_or_a_a, 0xB7, a, 0x12, 0x12, 0x12, false, 4);
test_cp!(
    r8_r8,
    test_0xb8_cp_a_b,
    0xB8,
    b,
    0x0A,
    0x03,
    false,
    false,
    false,
    4
);

test_cp!(
    r8_r8,
    test_0xb9_cp_a_c_h_carry,
    0xB9,
    c,
    0x10,
    0x01,
    false,
    true,
    false,
    4
);

test_cp!(
    r8_r8,
    test_0xba_cp_a_d_carry,
    0xBA,
    d,
    0x00,
    0x01,
    false,
    true,
    true,
    4
);

test_cp!(
    r8_r8,
    test_0xbb_cp_a_e_zero,
    0xBB,
    e,
    0x05,
    0x05,
    true,
    false,
    false,
    4
);

test_cp!(
    r8_r8,
    test_0xbc_cp_a_h_carry_h_carry,
    0xBC,
    h,
    0x10,
    0x11,
    false,
    true,
    true,
    4
);

test_cp!(
    r8_r8,
    test_0xbd_cp_a_l_h_carry,
    0xBD,
    l,
    0x20,
    0x01,
    false,
    true,
    false,
    4
);

test_cp!(
    r8_hl_mem,
    test_0xbe_cp_a_hl_mem,
    0xBE,
    0x0A,
    0x03,
    false,
    false,
    false,
    8
);

test_cp!(
    r8_r8,
    test_0xbf_cp_a_a_zero,
    0xBF,
    a,
    0x42,
    0x42,
    true,
    false,
    false,
    4
);
test_ret!(test_0xc0_ret_nz_jump, 0xC0, FLAG_Z, false, true);
test_ret!(test_0xc0_ret_nz_no_jump, 0xC0, FLAG_Z, true, false);
test_pop!(test_0xc1_pop_bc, 0xC1, bc, 0x1234);
test_pop!(
    #[ignore]
    test_0xd1_pop_de,
    0xD1,
    de,
    0x5678
);
test_pop!(
    #[ignore]
    test_0xe1_pop_hl,
    0xE1,
    hl,
    0x9ABC
);
test_pop!(
    #[ignore]
    test_0xf1_pop_af,
    0xF1,
    af,
    0x42F0
);
test_jp!(test_0xc2_jp_nz_jump, 0xC2, FLAG_Z, false, 0x1234, true);
test_jp!(test_0xc2_jp_nz_no_jump, 0xC2, FLAG_Z, true, 0x1234, false);
test_jp!(test_0xc3_jp_a16, 0xC3, 0xABCD);
test_call!(test_0xc4_call_nz_jump, 0xC4, FLAG_Z, false, 0x1234, true);
test_call!(test_0xc4_call_nz_no_jump, 0xC4, FLAG_Z, true, 0x1234, false);
test_push!(test_0xc5_push_bc, 0xC5, bc, 0x1234);
test_add!(
    r8_n8,
    test_0xc6_add_a_n8,
    0xC6,
    0x10,
    0x20,
    0x30,
    false,
    false,
    false,
    false,
    8
);
test_rst!(test_0xc7_rst_00, 0xC7, 0x0000);
test_ret!(test_0xc8_ret_z_taken, 0xC8, FLAG_Z, true, true);
test_ret!(test_0xc8_ret_z_not_taken, 0xC8, FLAG_Z, false, false);
test_ret!(test_0xc9_ret, 0xC9);
test_jp!(test_0xca_jp_z_jump, 0xCA, FLAG_Z, true, 0x4000, true);
test_jp!(test_0xca_jp_z_no_jump, 0xCA, FLAG_Z, false, 0x4000, false);
#[test]
#[should_panic(expected = "WIP")]
fn test_prefix() {
    let (mut cpu, mut bus) = setup_test!(&[0xCB]);
    cpu.step(&mut bus);
}
test_call!(
    #[ignore]
    test_0xcc_call_z_jump,
    0xCC,
    FLAG_Z,
    true,
    0x5678,
    true
);
test_call!(
    #[ignore]
    test_0xcc_call_z_no_jump,
    0xCC,
    FLAG_Z,
    false,
    0x5678,
    false
);
test_call!(
    #[ignore]
    test_0xcd_call_a16,
    0xCD,
    0xABCD
);
test_jp!(
    #[ignore]
    test_0xd2_jp_nc_jump,
    0xD2,
    FLAG_C,
    false,
    0x5000,
    true
);
test_jp!(
    #[ignore]
    test_0xd2_jp_nc_no_jump,
    0xD2,
    FLAG_C,
    true,
    0x5000,
    false
);
test_call!(
    #[ignore]
    test_0xd4_call_nc_jump,
    0xD4,
    FLAG_C,
    false,
    0x9ABC,
    true
);
test_call!(
    #[ignore]
    test_0xd4_call_nc_no_jump,
    0xD4,
    FLAG_C,
    true,
    0x9ABC,
    false
);
test_push!(
    #[ignore]
    test_0xd5_push_de,
    0xD5,
    de,
    0x5678
);
test_jp!(
    #[ignore]
    test_0xda_jp_c_jump,
    0xDA,
    FLAG_C,
    true,
    0x6000,
    true
);
test_jp!(
    #[ignore]
    test_0xda_jp_c_no_jump,
    0xDA,
    FLAG_C,
    false,
    0x6000,
    false
);
test_call!(
    #[ignore]
    test_0xdc_call_c_jump,
    0xDC,
    FLAG_C,
    true,
    0xDEF0,
    true
);
test_call!(
    #[ignore]
    test_0xdc_call_c_no_jump,
    0xDC,
    FLAG_C,
    false,
    0xDEF0,
    false
);
test_push!(
    #[ignore]
    test_0xe5_push_hl,
    0xE5,
    hl,
    0x9ABC
);
test_push!(
    #[ignore]
    test_0xf5_push_af,
    0xF5,
    af,
    0x42F0
);
