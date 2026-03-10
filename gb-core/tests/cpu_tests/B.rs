use super::*;

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
