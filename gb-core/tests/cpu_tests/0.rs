use super::*;

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
