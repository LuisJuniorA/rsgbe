use super::*;

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
