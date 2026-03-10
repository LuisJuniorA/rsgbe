use super::*;

test_mem_write_r8!(test_0x70_ld_hl_mem_b, 0x70, hl, b, 8);

test_mem_write_r8!(test_0x71_ld_hl_mem_c, 0x71, hl, c, 8);

test_mem_write_r8!(test_0x72_ld_hl_mem_d, 0x72, hl, d, 8);

test_mem_write_r8!(test_0x73_ld_hl_mem_e, 0x73, hl, e, 8);

test_mem_write_r8!(test_0x74_ld_hl_mem_h, 0x74, hl, h, 8);

test_mem_write_r8!(test_0x75_ld_hl_mem_l, 0x75, hl, l, 8);

#[test]
#[should_panic(expected = "WIP")]
fn test_0x76_halt() {
    let (mut cpu, mut bus) = setup_test!(&[0x76]);
    cpu.step(&mut bus);
}

test_mem_write_r8!(test_0x77_ld_hl_mem_a, 0x77, hl, a, 8);

test_ld!(r8_r8, test_0x78_ld_a_b, 0x78, a, b, 4);

test_ld!(r8_r8, test_0x79_ld_a_c, 0x79, a, c, 4);

test_ld!(r8_r8, test_0x7a_ld_a_d, 0x7A, a, d, 4);

test_ld!(r8_r8, test_0x7b_ld_a_e, 0x7B, a, e, 4);

test_ld!(r8_r8, test_0x7c_ld_a_h, 0x7C, a, h, 4);

test_ld!(r8_r8, test_0x7d_ld_a_l, 0x7D, a, l, 4);

test_ld!(r8_hl_mem, test_0x7e_ld_a_hl, 0x7E, a, 8);

test_ld!(r8_r8, test_0x7f_ld_a_a, 0x7F, a, a, 4);
