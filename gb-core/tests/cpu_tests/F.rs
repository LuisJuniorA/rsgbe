use super::*;

#[test]
fn test_0xf0_ldh_a_a8() {
    let (mut cpu, mut bus) = setup_test!(&[0xF0, 0x88]);
    bus.write_byte(0xFF88, 0xE7);
    let t = cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0xE7);
    assert_eq!(t, 12);
}

test_pop!(test_0xf1_pop_af, 0xF1, af, 0x42F0);
#[test]
fn test_0xf2_ldh_a_c() {
    let (mut cpu, mut bus) = setup_test!(&[0xF2]);
    cpu.registers.c = 0x85;
    bus.write_byte(0xFF85, 0x12);
    let t = cpu.step(&mut bus);
    assert_eq!(cpu.registers.a, 0x12);
    assert_eq!(t, 8);
}
#[test]
fn test_0xf3_di() {
    let (mut cpu, mut bus) = setup_test!(&[0xF3]);
    cpu.ime = true;
    let t = cpu.step(&mut bus);
    assert!(!cpu.ime, "IME should be disabled after DI");
    assert_eq!(t, 4, "DI should take 4 cycles");
}

test_push!(test_0xf5_push_af, 0xF5, af, 0x42F0);
test_or!(r8_n8, test_0xf6_or_a_n8, 0xF6, 0x5A, 0x0F, 0x5F, false, 8);
