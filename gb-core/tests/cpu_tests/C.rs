use super::*;

test_ret!(test_0xc0_ret_nz_jump, 0xC0, FLAG_Z, false, true);

test_ret!(test_0xc0_ret_nz_no_jump, 0xC0, FLAG_Z, true, false);

test_pop!(test_0xc1_pop_bc, 0xC1, bc, 0x1234);

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

test_call!(test_0xcc_call_z_jump, 0xCC, FLAG_Z, true, 0x5678, true);

test_call!(test_0xcc_call_z_no_jump, 0xCC, FLAG_Z, false, 0x5678, false);

test_call!(test_0xcd_call_a16, 0xCD, 0xABCD);

test_adc!(
    r8_n8,
    test_0xce_adc_simple,
    0xCE,
    0x10,
    0x01,
    false,
    0x11,
    false,
    false,
    false,
    false,
    8
);

test_adc!(
    r8_n8,
    test_0xce_adc_with_carry,
    0xCE,
    0x10,
    0x01,
    true,
    0x12,
    false,
    false,
    false,
    false,
    8
);

test_adc!(
    r8_n8,
    test_0xce_adc_half_carry,
    0xCE,
    0x0F,
    0x00,
    true,
    0x10,
    false,
    false,
    true,
    false,
    8
);

test_adc!(
    r8_n8,
    test_0xce_adc_zero_and_carry,
    0xCE,
    0xFF,
    0x00,
    true,
    0x00,
    true,
    false,
    true,
    true,
    8
);

test_rst!(test_0xcf_rst_08, 0xCF, 0x0008);
