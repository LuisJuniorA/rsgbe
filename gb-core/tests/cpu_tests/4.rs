use super::*;

test_ld!(r8_r8, test_0x40_ld_b_b, 0x40, b, b, 4);

test_ld!(r8_r8, test_0x41_ld_b_c, 0x41, b, c, 4);

test_ld!(r8_r8, test_0x42_ld_b_d, 0x42, b, d, 4);

test_ld!(r8_r8, test_0x43_ld_b_e, 0x43, b, e, 4);

test_ld!(r8_r8, test_0x44_ld_b_h, 0x44, b, h, 4);

test_ld!(r8_r8, test_0x45_ld_b_l, 0x45, b, l, 4);

test_ld!(r8_hl_mem, test_0x46_ld_b_hl, 0x46, b, 8);

test_ld!(r8_r8, test_0x47_ld_b_a, 0x47, b, a, 4);

test_ld!(r8_r8, test_0x48_ld_c_b, 0x48, c, b, 4);

test_ld!(r8_r8, test_0x49_ld_c_c, 0x49, c, c, 4);

test_ld!(r8_r8, test_0x4a_ld_c_d, 0x4A, c, d, 4);

test_ld!(r8_r8, test_0x4b_ld_c_e, 0x4B, c, e, 4);

test_ld!(r8_r8, test_0x4c_ld_c_h, 0x4C, c, h, 4);

test_ld!(r8_r8, test_0x4d_ld_c_l, 0x4D, c, l, 4);

test_ld!(r8_hl_mem, test_0x4e_ld_c_hl, 0x4E, c, 8);

test_ld!(r8_r8, test_0x4f_ld_c_a, 0x4F, c, a, 4);
