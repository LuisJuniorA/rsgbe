use super::*;

test_ld!(r8_r8, test_0x60_ld_h_b, 0x60, h, b, 4);

test_ld!(r8_r8, test_0x61_ld_h_c, 0x61, h, c, 4);

test_ld!(r8_r8, test_0x62_ld_h_d, 0x62, h, d, 4);

test_ld!(r8_r8, test_0x63_ld_h_e, 0x63, h, e, 4);

test_ld!(r8_r8, test_0x64_ld_h_h, 0x64, h, h, 4);

test_ld!(r8_r8, test_0x65_ld_h_l, 0x65, h, l, 4);

test_ld!(r8_hl_mem, test_0x66_ld_h_hl, 0x66, h, 8);

test_ld!(r8_r8, test_0x67_ld_h_a, 0x67, h, a, 4);

test_ld!(r8_r8, test_0x68_ld_l_b, 0x68, l, b, 4);

test_ld!(r8_r8, test_0x69_ld_l_c, 0x69, l, c, 4);

test_ld!(r8_r8, test_0x6a_ld_l_d, 0x6A, l, d, 4);

test_ld!(r8_r8, test_0x6b_ld_l_e, 0x6B, l, e, 4);

test_ld!(r8_r8, test_0x6c_ld_l_h, 0x6C, l, h, 4);

test_ld!(r8_r8, test_0x6d_ld_l_l, 0x6D, l, l, 4);

test_ld!(r8_hl_mem, test_0x6e_ld_l_hl, 0x6E, l, 8);

test_ld!(r8_r8, test_0x6f_ld_l_a, 0x6F, l, a, 4);
