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

test_push!(
    #[ignore]
    test_0xf5_push_af,
    0xF5,
    af,
    0x42F0
);
