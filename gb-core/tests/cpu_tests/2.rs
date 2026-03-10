use super::*;

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
