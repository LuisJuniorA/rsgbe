use super::*;

test_ld!(r8_r8, test_0x50_ld_d_b, 0x50, d, b, 4);

test_ld!(r8_r8, test_0x51_ld_d_c, 0x51, d, c, 4);

test_ld!(r8_r8, test_0x52_ld_d_d, 0x52, d, d, 4);

test_ld!(r8_r8, test_0x53_ld_d_e, 0x53, d, e, 4);

test_ld!(r8_r8, test_0x54_ld_d_h, 0x54, d, h, 4);

test_ld!(r8_r8, test_0x55_ld_d_l, 0x55, d, l, 4);

test_ld!(r8_hl_mem, test_0x56_ld_d_hl, 0x56, d, 8);

test_ld!(r8_r8, test_0x57_ld_d_a, 0x57, d, a, 4);

test_ld!(r8_r8, test_0x58_ld_e_b, 0x58, e, b, 4);

test_ld!(r8_r8, test_0x59_ld_e_c, 0x59, e, c, 4);

test_ld!(r8_r8, test_0x5a_ld_e_d, 0x5A, e, d, 4);

test_ld!(r8_r8, test_0x5b_ld_e_e, 0x5B, e, e, 4);

test_ld!(r8_r8, test_0x5c_ld_e_h, 0x5C, e, h, 4);

test_ld!(r8_r8, test_0x5d_ld_e_l, 0x5D, e, l, 4);

test_ld!(r8_hl_mem, test_0x5e_ld_e_hl, 0x5E, e, 8);

test_ld!(r8_r8, test_0x5f_ld_e_a, 0x5F, e, a, 4);
