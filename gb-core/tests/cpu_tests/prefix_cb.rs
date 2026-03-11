use super::*;

// RLC
test_cb_r8!(test_cb_00_rlc_b, 0x00, b, 0x85, 0x0B, false, false, false, true, 8);
test_cb_r8!(test_cb_01_rlc_c, 0x01, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_02_rlc_d, 0x02, d, 0x80, 0x01, false, false, false, true, 8);
test_cb_r8!(test_cb_03_rlc_e, 0x03, e, 0x01, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_04_rlc_h, 0x04, h, 0xFF, 0xFF, false, false, false, true, 8);
test_cb_r8!(test_cb_05_rlc_l, 0x05, l, 0x0F, 0x1E, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_06_rlc_hl_mem, 0x06, 0x81, 0x03, false, false, false, true, 16);
test_cb_r8!(test_cb_07_rlc_a, 0x07, a, 0x85, 0x0B, false, false, false, true, 8);

// RRC
test_cb_r8!(test_cb_08_rrc_b, 0x08, b, 0x01, 0x80, false, false, false, true, 8);
test_cb_r8!(test_cb_09_rrc_c, 0x09, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_0a_rrc_d, 0x0A, d, 0x81, 0xC0, false, false, false, true, 8);
test_cb_r8!(test_cb_0b_rrc_e, 0x0B, e, 0x02, 0x01, false, false, false, false, 8);
test_cb_r8!(test_cb_0c_rrc_h, 0x0C, h, 0xFF, 0xFF, false, false, false, true, 8);
test_cb_r8!(test_cb_0d_rrc_l, 0x0D, l, 0x80, 0x40, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_0e_rrc_hl_mem, 0x0E, 0x01, 0x80, false, false, false, true, 16);
test_cb_r8!(test_cb_0f_rrc_a, 0x0F, a, 0x01, 0x80, false, false, false, true, 8);
