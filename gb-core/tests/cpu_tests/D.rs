use super::*;
test_ret!(test_0xd0_ret_nc_jump, 0xD0, FLAG_C, false, true);
test_ret!(test_0xd0_ret_nc_no_jump, 0xD0, FLAG_C, true, false);
test_pop!(test_0xd1_pop_de, 0xD1, de, 0x5678);
test_jp!(test_0xd2_jp_nc_jump, 0xD2, FLAG_C, false, 0x5000, true);
test_jp!(test_0xd2_jp_nc_no_jump, 0xD2, FLAG_C, true, 0x5000, false);
test_call!(test_0xd4_call_nc_jump, 0xD4, FLAG_C, false, 0x9ABC, true);
test_call!(test_0xd4_call_nc_no_jump, 0xD4, FLAG_C, true, 0x9ABC, false);
test_push!(test_0xd5_push_de, 0xD5, de, 0x5678);
test_sub!(
    r8_n8,
    test_0xd6_sub_a_n8,
    0xD6,
    0x0A,
    0x03,
    0x07,
    false,
    false,
    false,
    8
);

test_ret!(test_0xd8_ret_z_taken, 0xD8, FLAG_C, true, true);
test_ret!(test_0xd8_ret_z_not_taken, 0xD8, FLAG_C, false, false);

test_rst!(rst_10, 0xD7, 0x0010);
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

#[test]
fn test_0xd9_reti() {
    let (mut cpu, mut bus) = setup_test!(&[0xD9]);
    cpu.sp = 0xFFFC;
    bus.write_byte(0xFFFC, 0x34);
    bus.write_byte(0xFFFD, 0x12);
    cpu.ime = false;
    let t = cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0x1234, "PC should jump to the popped address");
    assert_eq!(cpu.sp, 0xFFFE, "SP should be incremented by 2");
    assert_eq!(t, 16, "RETI should take 16 cycles");
    assert!(cpu.ime, "IME should be enabled after RETI");
}
