use super::*;

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
