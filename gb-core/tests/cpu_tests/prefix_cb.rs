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

// RL
test_cb_r8!(test_cb_10_rl_b, 0x10, b, 0x85, 0x0A, false, false, false, true, 8);
test_cb_r8!(test_cb_11_rl_c, 0x11, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_12_rl_d, 0x12, d, 0x80, 0x00, true, false, false, true, 8);
test_cb_r8!(test_cb_13_rl_e, 0x13, e, 0x01, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_14_rl_h, 0x14, h, 0xFF, 0xFE, false, false, false, true, 8);
test_cb_r8!(test_cb_15_rl_l, 0x15, l, 0x0F, 0x1E, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_16_rl_hl_mem, 0x16, 0x81, 0x02, false, false, false, true, 16);
test_cb_r8!(test_cb_17_rl_a, 0x17, a, 0x85, 0x0A, false, false, false, true, 8);

// RR
test_cb_r8!(test_cb_18_rr_b, 0x18, b, 0x01, 0x00, true, false, false, true, 8);
test_cb_r8!(test_cb_19_rr_c, 0x19, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_1a_rr_d, 0x1A, d, 0x81, 0x40, false, false, false, true, 8);
test_cb_r8!(test_cb_1b_rr_e, 0x1B, e, 0x02, 0x01, false, false, false, false, 8);
test_cb_r8!(test_cb_1c_rr_h, 0x1C, h, 0xFF, 0x7F, false, false, false, true, 8);
test_cb_r8!(test_cb_1d_rr_l, 0x1D, l, 0x80, 0x40, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_1e_rr_hl_mem, 0x1E, 0x01, 0x00, true, false, false, true, 16);
test_cb_r8!(test_cb_1f_rr_a, 0x1F, a, 0x01, 0x00, true, false, false, true, 8);

// SLA (Shift Left Arithmetic)
// Shifts left: bit 0 becomes 0, bit 7 goes into the Carry flag.
test_cb_r8!(test_cb_20_sla_b, 0x20, b, 0x85, 0x0A, false, false, false, true, 8);
test_cb_r8!(test_cb_21_sla_c, 0x21, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_22_sla_d, 0x22, d, 0x80, 0x00, true, false, false, true, 8);
test_cb_r8!(test_cb_23_sla_e, 0x23, e, 0x01, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_24_sla_h, 0x24, h, 0xFF, 0xFE, false, false, false, true, 8);
test_cb_r8!(test_cb_25_sla_l, 0x25, l, 0x0F, 0x1E, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_26_sla_hl_mem, 0x26, 0x81, 0x02, false, false, false, true, 16);
test_cb_r8!(test_cb_27_sla_a, 0x27, a, 0x85, 0x0A, false, false, false, true, 8);

// SRA (Shift Right Arithmetic)
// Shifts right: bit 7 is duplicated (sign extension), bit 0 goes into the Carry flag.
test_cb_r8!(test_cb_28_sra_b, 0x28, b, 0x85, 0xC2, false, false, false, true, 8);
test_cb_r8!(test_cb_29_sra_c, 0x29, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_2a_sra_d, 0x2A, d, 0x80, 0xC0, false, false, false, false, 8);
test_cb_r8!(test_cb_2b_sra_e, 0x2B, e, 0x01, 0x00, true, false, false, true, 8);
test_cb_r8!(test_cb_2c_sra_h, 0x2C, h, 0xFF, 0xFF, false, false, false, true, 8);
test_cb_r8!(test_cb_2d_sra_l, 0x2D, l, 0x0F, 0x07, false, false, false, true, 8);
test_cb_hl_mem!(test_cb_2e_sra_hl_mem, 0x2E, 0x81, 0xC0, false, false, false, true, 16);
test_cb_r8!(test_cb_2f_sra_a, 0x2F, a, 0x01, 0x00, true, false, false, true, 8);
