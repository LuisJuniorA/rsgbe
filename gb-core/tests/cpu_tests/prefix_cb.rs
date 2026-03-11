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

// SWAP
// Swaps the upper and lower nibbles (4 bits) of the byte.
// Flags: Z is set if result is 0. N, H, and C are always reset.
test_cb_r8!(test_cb_30_swap_b, 0x30, b, 0xA5, 0x5A, false, false, false, false, 8);
test_cb_r8!(test_cb_31_swap_c, 0x31, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_32_swap_d, 0x32, d, 0xF0, 0x0F, false, false, false, false, 8);
test_cb_r8!(test_cb_33_swap_e, 0x33, e, 0x0F, 0xF0, false, false, false, false, 8);
test_cb_r8!(test_cb_34_swap_h, 0x34, h, 0x12, 0x21, false, false, false, false, 8);
test_cb_r8!(test_cb_35_swap_l, 0x35, l, 0xAB, 0xBA, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_36_swap_hl_mem, 0x36, 0x80, 0x08, false, false, false, false, 16);
test_cb_r8!(test_cb_37_swap_a, 0x37, a, 0x00, 0x00, true, false, false, false, 8);

// SRL (Shift Right Logical)
// Shifts right: bit 7 becomes 0, bit 0 goes into the Carry flag.
// Flags: Z is set if result is 0. N and H are reset. C contains old bit 0.
test_cb_r8!(test_cb_38_srl_b, 0x38, b, 0x85, 0x42, false, false, false, true, 8);
test_cb_r8!(test_cb_39_srl_c, 0x39, c, 0x00, 0x00, true, false, false, false, 8);
test_cb_r8!(test_cb_3a_srl_d, 0x3A, d, 0x80, 0x40, false, false, false, false, 8);
test_cb_r8!(test_cb_3b_srl_e, 0x3B, e, 0x01, 0x00, true, false, false, true, 8);
test_cb_r8!(test_cb_3c_srl_h, 0x3C, h, 0xFF, 0x7F, false, false, false, true, 8);
test_cb_r8!(test_cb_3d_srl_l, 0x3D, l, 0x0F, 0x07, false, false, false, true, 8);
test_cb_hl_mem!(test_cb_3e_srl_hl_mem, 0x3E, 0x81, 0x40, false, false, false, true, 16);
test_cb_r8!(test_cb_3f_srl_a, 0x3F, a, 0x01, 0x00, true, false, false, true, 8);

// BIT 0 (0x40 - 0x47)
test_cb_r8!(test_cb_40_bit_0_b, 0x40, b, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_41_bit_0_c, 0x41, c, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_42_bit_0_d, 0x42, d, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_43_bit_0_e, 0x43, e, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_44_bit_0_h, 0x44, h, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_45_bit_0_l, 0x45, l, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_hl_mem!(test_cb_46_bit_0_hl_mem, 0x46, 0x55, 0x55, false, false, true, false, 12);
test_cb_r8!(test_cb_47_bit_0_a, 0x47, a, 0xAA, 0xAA, true, false, true, false, 8);

// BIT 1 (0x48 - 0x4F)
test_cb_r8!(test_cb_48_bit_1_b, 0x48, b, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_49_bit_1_c, 0x49, c, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_4a_bit_1_d, 0x4A, d, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_4b_bit_1_e, 0x4B, e, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_4c_bit_1_h, 0x4C, h, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_4d_bit_1_l, 0x4D, l, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_hl_mem!(test_cb_4e_bit_1_hl_mem, 0x4E, 0x55, 0x55, true, false, true, false, 12);
test_cb_r8!(test_cb_4f_bit_1_a, 0x4F, a, 0xAA, 0xAA, false, false, true, false, 8);

// BIT 2 (0x50 - 0x57)
test_cb_r8!(test_cb_50_bit_2_b, 0x50, b, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_51_bit_2_c, 0x51, c, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_52_bit_2_d, 0x52, d, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_53_bit_2_e, 0x53, e, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_54_bit_2_h, 0x54, h, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_55_bit_2_l, 0x55, l, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_hl_mem!(test_cb_56_bit_2_hl_mem, 0x56, 0x55, 0x55, false, false, true, false, 12);
test_cb_r8!(test_cb_57_bit_2_a, 0x57, a, 0xAA, 0xAA, true, false, true, false, 8);

// BIT 3 (0x58 - 0x5F)
test_cb_r8!(test_cb_58_bit_3_b, 0x58, b, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_59_bit_3_c, 0x59, c, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_5a_bit_3_d, 0x5A, d, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_5b_bit_3_e, 0x5B, e, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_5c_bit_3_h, 0x5C, h, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_5d_bit_3_l, 0x5D, l, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_hl_mem!(test_cb_5e_bit_3_hl_mem, 0x5E, 0x55, 0x55, true, false, true, false, 12);
test_cb_r8!(test_cb_5f_bit_3_a, 0x5F, a, 0xAA, 0xAA, false, false, true, false, 8);

// BIT 4 (0x60 - 0x67)
test_cb_r8!(test_cb_60_bit_4_b, 0x60, b, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_61_bit_4_c, 0x61, c, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_62_bit_4_d, 0x62, d, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_63_bit_4_e, 0x63, e, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_64_bit_4_h, 0x64, h, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_65_bit_4_l, 0x65, l, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_hl_mem!(test_cb_66_bit_4_hl_mem, 0x66, 0x55, 0x55, false, false, true, false, 12);
test_cb_r8!(test_cb_67_bit_4_a, 0x67, a, 0xAA, 0xAA, true, false, true, false, 8);

// BIT 5 (0x68 - 0x6F)
test_cb_r8!(test_cb_68_bit_5_b, 0x68, b, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_69_bit_5_c, 0x69, c, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_6a_bit_5_d, 0x6A, d, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_6b_bit_5_e, 0x6B, e, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_6c_bit_5_h, 0x6C, h, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_6d_bit_5_l, 0x6D, l, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_hl_mem!(test_cb_6e_bit_5_hl_mem, 0x6E, 0x55, 0x55, true, false, true, false, 12);
test_cb_r8!(test_cb_6f_bit_5_a, 0x6F, a, 0xAA, 0xAA, false, false, true, false, 8);

// BIT 6 (0x70 - 0x77)
test_cb_r8!(test_cb_70_bit_6_b, 0x70, b, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_71_bit_6_c, 0x71, c, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_72_bit_6_d, 0x72, d, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_73_bit_6_e, 0x73, e, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_r8!(test_cb_74_bit_6_h, 0x74, h, 0x55, 0x55, false, false, true, false, 8);
test_cb_r8!(test_cb_75_bit_6_l, 0x75, l, 0xAA, 0xAA, true, false, true, false, 8);
test_cb_hl_mem!(test_cb_76_bit_6_hl_mem, 0x76, 0x55, 0x55, false, false, true, false, 12);
test_cb_r8!(test_cb_77_bit_6_a, 0x77, a, 0xAA, 0xAA, true, false, true, false, 8);

// BIT 7 (0x78 - 0x7F)
test_cb_r8!(test_cb_78_bit_7_b, 0x78, b, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_79_bit_7_c, 0x79, c, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_7a_bit_7_d, 0x7A, d, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_7b_bit_7_e, 0x7B, e, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_r8!(test_cb_7c_bit_7_h, 0x7C, h, 0x55, 0x55, true, false, true, false, 8);
test_cb_r8!(test_cb_7d_bit_7_l, 0x7D, l, 0xAA, 0xAA, false, false, true, false, 8);
test_cb_hl_mem!(test_cb_7e_bit_7_hl_mem, 0x7E, 0x55, 0x55, true, false, true, false, 12);
test_cb_r8!(test_cb_7f_bit_7_a, 0x7F, a, 0xAA, 0xAA, false, false, true, false, 8);

// RES 0 (0x80 - 0x87)
test_cb_r8!(test_cb_80_res_0_b, 0x80, b, 0xFF, 0xFE, false, false, false, false, 8);
test_cb_r8!(test_cb_81_res_0_c, 0x81, c, 0xFF, 0xFE, false, false, false, false, 8);
test_cb_r8!(test_cb_82_res_0_d, 0x82, d, 0xFF, 0xFE, false, false, false, false, 8);
test_cb_r8!(test_cb_83_res_0_e, 0x83, e, 0xFF, 0xFE, false, false, false, false, 8);
test_cb_r8!(test_cb_84_res_0_h, 0x84, h, 0xFF, 0xFE, false, false, false, false, 8);
test_cb_r8!(test_cb_85_res_0_l, 0x85, l, 0xFF, 0xFE, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_86_res_0_hl_mem, 0x86, 0xFF, 0xFE, false, false, false, false, 16);
test_cb_r8!(test_cb_87_res_0_a, 0x87, a, 0xFF, 0xFE, false, false, false, false, 8);

// RES 1 (0x88 - 0x8F)
test_cb_r8!(test_cb_88_res_1_b, 0x88, b, 0xFF, 0xFD, false, false, false, false, 8);
test_cb_r8!(test_cb_89_res_1_c, 0x89, c, 0xFF, 0xFD, false, false, false, false, 8);
test_cb_r8!(test_cb_8a_res_1_d, 0x8A, d, 0xFF, 0xFD, false, false, false, false, 8);
test_cb_r8!(test_cb_8b_res_1_e, 0x8B, e, 0xFF, 0xFD, false, false, false, false, 8);
test_cb_r8!(test_cb_8c_res_1_h, 0x8C, h, 0xFF, 0xFD, false, false, false, false, 8);
test_cb_r8!(test_cb_8d_res_1_l, 0x8D, l, 0xFF, 0xFD, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_8e_res_1_hl_mem, 0x8E, 0xFF, 0xFD, false, false, false, false, 16);
test_cb_r8!(test_cb_8f_res_1_a, 0x8F, a, 0xFF, 0xFD, false, false, false, false, 8);

// RES 2 (0x90 - 0x97)
test_cb_r8!(test_cb_90_res_2_b, 0x90, b, 0xFF, 0xFB, false, false, false, false, 8);
test_cb_r8!(test_cb_91_res_2_c, 0x91, c, 0xFF, 0xFB, false, false, false, false, 8);
test_cb_r8!(test_cb_92_res_2_d, 0x92, d, 0xFF, 0xFB, false, false, false, false, 8);
test_cb_r8!(test_cb_93_res_2_e, 0x93, e, 0xFF, 0xFB, false, false, false, false, 8);
test_cb_r8!(test_cb_94_res_2_h, 0x94, h, 0xFF, 0xFB, false, false, false, false, 8);
test_cb_r8!(test_cb_95_res_2_l, 0x95, l, 0xFF, 0xFB, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_96_res_2_hl_mem, 0x96, 0xFF, 0xFB, false, false, false, false, 16);
test_cb_r8!(test_cb_97_res_2_a, 0x97, a, 0xFF, 0xFB, false, false, false, false, 8);

// RES 3 (0x98 - 0x9F)
test_cb_r8!(test_cb_98_res_3_b, 0x98, b, 0xFF, 0xF7, false, false, false, false, 8);
test_cb_r8!(test_cb_99_res_3_c, 0x99, c, 0xFF, 0xF7, false, false, false, false, 8);
test_cb_r8!(test_cb_9a_res_3_d, 0x9A, d, 0xFF, 0xF7, false, false, false, false, 8);
test_cb_r8!(test_cb_9b_res_3_e, 0x9B, e, 0xFF, 0xF7, false, false, false, false, 8);
test_cb_r8!(test_cb_9c_res_3_h, 0x9C, h, 0xFF, 0xF7, false, false, false, false, 8);
test_cb_r8!(test_cb_9d_res_3_l, 0x9D, l, 0xFF, 0xF7, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_9e_res_3_hl_mem, 0x9E, 0xFF, 0xF7, false, false, false, false, 16);
test_cb_r8!(test_cb_9f_res_3_a, 0x9F, a, 0xFF, 0xF7, false, false, false, false, 8);

// RES 4 (0xA0 - 0xA7)
test_cb_r8!(test_cb_a0_res_4_b, 0xA0, b, 0xFF, 0xEF, false, false, false, false, 8);
test_cb_r8!(test_cb_a1_res_4_c, 0xA1, c, 0xFF, 0xEF, false, false, false, false, 8);
test_cb_r8!(test_cb_a2_res_4_d, 0xA2, d, 0xFF, 0xEF, false, false, false, false, 8);
test_cb_r8!(test_cb_a3_res_4_e, 0xA3, e, 0xFF, 0xEF, false, false, false, false, 8);
test_cb_r8!(test_cb_a4_res_4_h, 0xA4, h, 0xFF, 0xEF, false, false, false, false, 8);
test_cb_r8!(test_cb_a5_res_4_l, 0xA5, l, 0xFF, 0xEF, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_a6_res_4_hl_mem, 0xA6, 0xFF, 0xEF, false, false, false, false, 16);
test_cb_r8!(test_cb_a7_res_4_a, 0xA7, a, 0xFF, 0xEF, false, false, false, false, 8);

// RES 5 (0xA8 - 0xAF)
test_cb_r8!(test_cb_a8_res_5_b, 0xA8, b, 0xFF, 0xDF, false, false, false, false, 8);
test_cb_r8!(test_cb_a9_res_5_c, 0xA9, c, 0xFF, 0xDF, false, false, false, false, 8);
test_cb_r8!(test_cb_aa_res_5_d, 0xAA, d, 0xFF, 0xDF, false, false, false, false, 8);
test_cb_r8!(test_cb_ab_res_5_e, 0xAB, e, 0xFF, 0xDF, false, false, false, false, 8);
test_cb_r8!(test_cb_ac_res_5_h, 0xAC, h, 0xFF, 0xDF, false, false, false, false, 8);
test_cb_r8!(test_cb_ad_res_5_l, 0xAD, l, 0xFF, 0xDF, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_ae_res_5_hl_mem, 0xAE, 0xFF, 0xDF, false, false, false, false, 16);
test_cb_r8!(test_cb_af_res_5_a, 0xAF, a, 0xFF, 0xDF, false, false, false, false, 8);

// RES 6 (0xB0 - 0xB7)
test_cb_r8!(test_cb_b0_res_6_b, 0xB0, b, 0xFF, 0xBF, false, false, false, false, 8);
test_cb_r8!(test_cb_b1_res_6_c, 0xB1, c, 0xFF, 0xBF, false, false, false, false, 8);
test_cb_r8!(test_cb_b2_res_6_d, 0xB2, d, 0xFF, 0xBF, false, false, false, false, 8);
test_cb_r8!(test_cb_b3_res_6_e, 0xB3, e, 0xFF, 0xBF, false, false, false, false, 8);
test_cb_r8!(test_cb_b4_res_6_h, 0xB4, h, 0xFF, 0xBF, false, false, false, false, 8);
test_cb_r8!(test_cb_b5_res_6_l, 0xB5, l, 0xFF, 0xBF, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_b6_res_6_hl_mem, 0xB6, 0xFF, 0xBF, false, false, false, false, 16);
test_cb_r8!(test_cb_b7_res_6_a, 0xB7, a, 0xFF, 0xBF, false, false, false, false, 8);

// RES 7 (0xB8 - 0xBF)
test_cb_r8!(test_cb_b8_res_7_b, 0xB8, b, 0xFF, 0x7F, false, false, false, false, 8);
test_cb_r8!(test_cb_b9_res_7_c, 0xB9, c, 0xFF, 0x7F, false, false, false, false, 8);
test_cb_r8!(test_cb_ba_res_7_d, 0xBA, d, 0xFF, 0x7F, false, false, false, false, 8);
test_cb_r8!(test_cb_bb_res_7_e, 0xBB, e, 0xFF, 0x7F, false, false, false, false, 8);
test_cb_r8!(test_cb_bc_res_7_h, 0xBC, h, 0xFF, 0x7F, false, false, false, false, 8);
test_cb_r8!(test_cb_bd_res_7_l, 0xBD, l, 0xFF, 0x7F, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_be_res_7_hl_mem, 0xBE, 0xFF, 0x7F, false, false, false, false, 16);
test_cb_r8!(test_cb_bf_res_7_a, 0xBF, a, 0xFF, 0x7F, false, false, false, false, 8);

// SET 0 (0xC0 - 0xC7)
test_cb_r8!(test_cb_c0_set_0_b, 0xC0, b, 0x00, 0x01, false, false, false, false, 8);
test_cb_r8!(test_cb_c1_set_0_c, 0xC1, c, 0x00, 0x01, false, false, false, false, 8);
test_cb_r8!(test_cb_c2_set_0_d, 0xC2, d, 0x00, 0x01, false, false, false, false, 8);
test_cb_r8!(test_cb_c3_set_0_e, 0xC3, e, 0x00, 0x01, false, false, false, false, 8);
test_cb_r8!(test_cb_c4_set_0_h, 0xC4, h, 0x00, 0x01, false, false, false, false, 8);
test_cb_r8!(test_cb_c5_set_0_l, 0xC5, l, 0x00, 0x01, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_c6_set_0_hl_mem, 0xC6, 0x00, 0x01, false, false, false, false, 16);
test_cb_r8!(test_cb_c7_set_0_a, 0xC7, a, 0x00, 0x01, false, false, false, false, 8);

// SET 1 (0xC8 - 0xCF)
test_cb_r8!(test_cb_c8_set_1_b, 0xC8, b, 0x00, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_c9_set_1_c, 0xC9, c, 0x00, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_ca_set_1_d, 0xCA, d, 0x00, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_cb_set_1_e, 0xCB, e, 0x00, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_cc_set_1_h, 0xCC, h, 0x00, 0x02, false, false, false, false, 8);
test_cb_r8!(test_cb_cd_set_1_l, 0xCD, l, 0x00, 0x02, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_ce_set_1_hl_mem, 0xCE, 0x00, 0x02, false, false, false, false, 16);
test_cb_r8!(test_cb_cf_set_1_a, 0xCF, a, 0x00, 0x02, false, false, false, false, 8);

// SET 2 (0xD0 - 0xD7)
test_cb_r8!(test_cb_d0_set_2_b, 0xD0, b, 0x00, 0x04, false, false, false, false, 8);
test_cb_r8!(test_cb_d1_set_2_c, 0xD1, c, 0x00, 0x04, false, false, false, false, 8);
test_cb_r8!(test_cb_d2_set_2_d, 0xD2, d, 0x00, 0x04, false, false, false, false, 8);
test_cb_r8!(test_cb_d3_set_2_e, 0xD3, e, 0x00, 0x04, false, false, false, false, 8);
test_cb_r8!(test_cb_d4_set_2_h, 0xD4, h, 0x00, 0x04, false, false, false, false, 8);
test_cb_r8!(test_cb_d5_set_2_l, 0xD5, l, 0x00, 0x04, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_d6_set_2_hl_mem, 0xD6, 0x00, 0x04, false, false, false, false, 16);
test_cb_r8!(test_cb_d7_set_2_a, 0xD7, a, 0x00, 0x04, false, false, false, false, 8);

// SET 3 (0xD8 - 0xDF)
test_cb_r8!(test_cb_d8_set_3_b, 0xD8, b, 0x00, 0x08, false, false, false, false, 8);
test_cb_r8!(test_cb_d9_set_3_c, 0xD9, c, 0x00, 0x08, false, false, false, false, 8);
test_cb_r8!(test_cb_da_set_3_d, 0xDA, d, 0x00, 0x08, false, false, false, false, 8);
test_cb_r8!(test_cb_db_set_3_e, 0xDB, e, 0x00, 0x08, false, false, false, false, 8);
test_cb_r8!(test_cb_dc_set_3_h, 0xDC, h, 0x00, 0x08, false, false, false, false, 8);
test_cb_r8!(test_cb_dd_set_3_l, 0xDD, l, 0x00, 0x08, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_de_set_3_hl_mem, 0xDE, 0x00, 0x08, false, false, false, false, 16);
test_cb_r8!(test_cb_df_set_3_a, 0xDF, a, 0x00, 0x08, false, false, false, false, 8);

// SET 4 (0xE0 - 0xE7)
test_cb_r8!(test_cb_e0_set_4_b, 0xE0, b, 0x00, 0x10, false, false, false, false, 8);
test_cb_r8!(test_cb_e1_set_4_c, 0xE1, c, 0x00, 0x10, false, false, false, false, 8);
test_cb_r8!(test_cb_e2_set_4_d, 0xE2, d, 0x00, 0x10, false, false, false, false, 8);
test_cb_r8!(test_cb_e3_set_4_e, 0xE3, e, 0x00, 0x10, false, false, false, false, 8);
test_cb_r8!(test_cb_e4_set_4_h, 0xE4, h, 0x00, 0x10, false, false, false, false, 8);
test_cb_r8!(test_cb_e5_set_4_l, 0xE5, l, 0x00, 0x10, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_e6_set_4_hl_mem, 0xE6, 0x00, 0x10, false, false, false, false, 16);
test_cb_r8!(test_cb_e7_set_4_a, 0xE7, a, 0x00, 0x10, false, false, false, false, 8);

// SET 5 (0xE8 - 0xEF)
test_cb_r8!(test_cb_e8_set_5_b, 0xE8, b, 0x00, 0x20, false, false, false, false, 8);
test_cb_r8!(test_cb_e9_set_5_c, 0xE9, c, 0x00, 0x20, false, false, false, false, 8);
test_cb_r8!(test_cb_ea_set_5_d, 0xEA, d, 0x00, 0x20, false, false, false, false, 8);
test_cb_r8!(test_cb_eb_set_5_e, 0xEB, e, 0x00, 0x20, false, false, false, false, 8);
test_cb_r8!(test_cb_ec_set_5_h, 0xEC, h, 0x00, 0x20, false, false, false, false, 8);
test_cb_r8!(test_cb_ed_set_5_l, 0xED, l, 0x00, 0x20, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_ee_set_5_hl_mem, 0xEE, 0x00, 0x20, false, false, false, false, 16);
test_cb_r8!(test_cb_ef_set_5_a, 0xEF, a, 0x00, 0x20, false, false, false, false, 8);

// SET 6 (0xF0 - 0xF7)
test_cb_r8!(test_cb_f0_set_6_b, 0xF0, b, 0x00, 0x40, false, false, false, false, 8);
test_cb_r8!(test_cb_f1_set_6_c, 0xF1, c, 0x00, 0x40, false, false, false, false, 8);
test_cb_r8!(test_cb_f2_set_6_d, 0xF2, d, 0x00, 0x40, false, false, false, false, 8);
test_cb_r8!(test_cb_f3_set_6_e, 0xF3, e, 0x00, 0x40, false, false, false, false, 8);
test_cb_r8!(test_cb_f4_set_6_h, 0xF4, h, 0x00, 0x40, false, false, false, false, 8);
test_cb_r8!(test_cb_f5_set_6_l, 0xF5, l, 0x00, 0x40, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_f6_set_6_hl_mem, 0xF6, 0x00, 0x40, false, false, false, false, 16);
test_cb_r8!(test_cb_f7_set_6_a, 0xF7, a, 0x00, 0x40, false, false, false, false, 8);

// SET 7 (0xF8 - 0xFF)
test_cb_r8!(test_cb_f8_set_7_b, 0xF8, b, 0x00, 0x80, false, false, false, false, 8);
test_cb_r8!(test_cb_f9_set_7_c, 0xF9, c, 0x00, 0x80, false, false, false, false, 8);
test_cb_r8!(test_cb_fa_set_7_d, 0xFA, d, 0x00, 0x80, false, false, false, false, 8);
test_cb_r8!(test_cb_fb_set_7_e, 0xFB, e, 0x00, 0x80, false, false, false, false, 8);
test_cb_r8!(test_cb_fc_set_7_h, 0xFC, h, 0x00, 0x80, false, false, false, false, 8);
test_cb_r8!(test_cb_fd_set_7_l, 0xFD, l, 0x00, 0x80, false, false, false, false, 8);
test_cb_hl_mem!(test_cb_fe_set_7_hl_mem, 0xFE, 0x00, 0x80, false, false, false, false, 16);
test_cb_r8!(test_cb_ff_set_7_a, 0xFF, a, 0x00, 0x80, false, false, false, false, 8);
