use super::*;

#[test]
fn test_0xe0_ldh_a8_a() {
    let (mut cpu, mut bus) = setup_test!(&[0xE0, 0x80]);
    cpu.registers.a = 0x42;
    let t = cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xFF80), 0x42);
    assert_eq!(t, 12);
}
test_pop!(test_0xe1_pop_hl, 0xE1, hl, 0x9ABC);
#[test]
fn test_0xe2_ldh_c_a() {
    let (mut cpu, mut bus) = setup_test!(&[0xE2]);
    cpu.registers.a = 0x42;
    cpu.registers.c = 0x80;
    let t = cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xFF80), 0x42);
    assert_eq!(t, 8);
}
test_push!(test_0xe5_push_hl, 0xE5, hl, 0x9ABC);
test_add!(
    r8_n8,
    test_0xe6_and_a_n8,
    0xE6,
    0x5A,
    0x0F,
    0x0A,
    false,
    false,
    true,
    false,
    8
);
test_rst!(test_0xe7_rst_20, 0xE7, 0x0020);
#[test]
fn test_0xe8_add_sp_e8() {
    let (mut cpu, mut bus) = setup_test!(&[0xE8, 0x02]);
    cpu.sp = 0xFFF0;
    let t = cpu.step(&mut bus);
    assert_eq!(cpu.sp, 0xFFF2);
    assert_flags!(cpu, false, false, false, false);
    assert_eq!(t, 16);
}
#[test]
fn test_0xe9_jp_hl() {
    let (mut cpu, mut bus) = setup_test!(&[0xE9]);
    set_r16!(cpu, hl, 0x4000);
    let t = cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0x4000);
    assert_eq!(t, 4);
}
#[test]
fn test_0xea_ld_a16_a() {
    let (mut cpu, mut bus) = setup_test!(&[0xEA, 0x23, 0xC1]);
    cpu.registers.a = 0xBC;
    let t = cpu.step(&mut bus);
    assert_eq!(bus.read_byte(0xC123), 0xBC);
    assert_eq!(t, 16);
}
test_xor!(r8_n8, test_0xee_xor_a_n8, 0xEE, 0x0F, 0x0F, 0x00, true, 8);
test_rst!(test_0xef_rst_28, 0xEF, 0x0028);
