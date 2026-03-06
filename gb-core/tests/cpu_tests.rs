use gb_core::{cpu::Cpu, cpu::FLAG_C, cpu::FLAG_H, cpu::FLAG_N, cpu::FLAG_Z, memory::Bus};

macro_rules! setup_test {
    ($data:expr) => {{
        let mut rom = vec![0x00; 0x0100]; // Boot padding
        rom.extend_from_slice($data);
        let bus = Bus::new(rom);
        let mut cpu = Cpu::new();
        cpu.pc = 0x0100;
        cpu.registers.a = 0;
        cpu.registers.b = 0;
        cpu.registers.c = 0;
        cpu.registers.d = 0;
        cpu.registers.e = 0;
        cpu.registers.h = 0;
        cpu.registers.l = 0;
        cpu.registers.f = 0;
        (cpu, bus)
    }};
}

macro_rules! assert_flags {
    ($cpu:expr, $z:expr, $n:expr, $h:expr, $c:expr) => {
        let f = $cpu.registers.f;
        assert_eq!((f & FLAG_Z != 0), $z, "Z flag mismatch");
        assert_eq!((f & FLAG_N != 0), $n, "N flag mismatch");
        assert_eq!((f & FLAG_H != 0), $h, "H flag mismatch");
        assert_eq!((f & FLAG_C != 0), $c, "C flag mismatch");
        assert_eq!(f & 0x0F, 0, "Lower 4 bits of F must always be 0");
    };
}

#[test]
fn test_0x00_nop() {
    let (mut cpu, mut bus) = setup_test!(&[0x00]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
}

#[test]
fn test_0x01_ld_bc_n16() {
    let (mut cpu, mut bus) = setup_test!(&[0x01, 0x34, 0x12]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(
        cpu.pc - old_pc,
        3,
        "PC should increment by 3 after LD BC, d16"
    );

    assert_eq!(
        cpu.registers.b, 0x12,
        "Register B should hold the high byte"
    );

    assert_eq!(cpu.registers.c, 0x34, "Register C should hold the low byte");

    assert_eq!(
        cpu.registers.get_bc(),
        0x1234,
        "Register BC should hold both bytes"
    );
}

#[test]
fn test_0x02_ld_bc_mem_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x02]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x42;
    cpu.registers.set_bc(0xC000);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(bus.read_byte(0xC000), 0x42);
}

#[test]
fn test_0x03_inc_bc() {
    let (mut cpu, mut bus) = setup_test!(&[0x03]);
    let old_pc = cpu.pc;
    cpu.registers.set_bc(0xFFFF);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_bc(), 0x0000);
}

#[test]
fn test_0x04_inc_b() {
    let (mut cpu, mut bus) = setup_test!(&[0x04]);
    let old_pc = cpu.pc;
    cpu.registers.b = 0x0F;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x10);
    assert_flags!(cpu, false, false, true, false);
}

#[test]
fn test_0x05_dec_b() {
    let (mut cpu, mut bus) = setup_test!(&[0x05]);
    let old_pc = cpu.pc;
    cpu.registers.b = 0x01;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x00);
    assert_flags!(cpu, true, true, false, false);
}

#[test]
fn test_0x06_ld_b_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x06, 0x77]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(cpu.registers.b, 0x77);
}

#[test]
fn test_0x07_rlca() {
    let (mut cpu, mut bus) = setup_test!(&[0x07]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x80;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0x01);
    assert_flags!(cpu, false, false, false, true);
}

#[test]
fn test_0x08_ld_a16_mem_sp() {
    let (mut cpu, mut bus) = setup_test!(&[0x08, 0x20, 0xC1]);
    let old_pc = cpu.pc;
    cpu.sp = 0xABCD;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 3);
    assert_eq!(bus.read_byte(0xC120), 0xCD);
    assert_eq!(bus.read_byte(0xC121), 0xAB);
}

#[test]
fn test_0x09_add_hl_bc() {
    let (mut cpu, mut bus) = setup_test!(&[0x09]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0x0FFF);
    cpu.registers.set_bc(0x0001);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_hl(), 0x1000);
    assert_flags!(cpu, false, false, true, false);
}

#[test]
fn test_0x0a_ld_a_bc_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x0A]);
    let old_pc = cpu.pc;
    cpu.registers.set_bc(0xC000);
    bus.write_byte(0xC000, 0xEF);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0xEF);
}

#[test]
fn test_0x0b_dec_bc() {
    let (mut cpu, mut bus) = setup_test!(&[0x0B]);
    let old_pc = cpu.pc;
    cpu.registers.set_bc(0x0001);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_bc(), 0x0000);
}

#[test]
fn test_0x0c_inc_c() {
    let (mut cpu, mut bus) = setup_test!(&[0x0C]);
    let old_pc = cpu.pc;
    cpu.registers.c = 0xFF;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x00);
    assert_flags!(cpu, true, false, true, false);
}

#[test]
fn test_0x0d_dec_c() {
    let (mut cpu, mut bus) = setup_test!(&[0x0D]);
    let old_pc = cpu.pc;
    cpu.registers.c = 0x00;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0xFF);
    assert_flags!(cpu, false, true, true, false);
}

#[test]
fn test_0x0e_ld_c_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x0E, 0x12]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(cpu.registers.c, 0x12);
}

#[test]
fn test_0x0f_rrca() {
    let (mut cpu, mut bus) = setup_test!(&[0x0F]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x01;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0x80);
    assert_flags!(cpu, false, false, false, true);
}

#[test]
#[should_panic(expected = "WIP")]
fn test_0x10_stop_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x10, 0x00]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
}

#[test]
fn test_0x11_ld_de_n16() {
    let (mut cpu, mut bus) = setup_test!(&[0x11, 0x00, 0x80]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 3);
    assert_eq!(cpu.registers.get_de(), 0x8000);
}

#[test]
fn test_0x12_ld_de_mem_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x12]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x55;
    cpu.registers.set_de(0xC001);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(bus.read_byte(0xC001), 0x55);
}

#[test]
fn test_0x13_inc_de() {
    let (mut cpu, mut bus) = setup_test!(&[0x13]);
    let old_pc = cpu.pc;
    cpu.registers.set_de(0x10FF);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_de(), 0x1100);
}

#[test]
fn test_0x14_inc_d() {
    let (mut cpu, mut bus) = setup_test!(&[0x14]);
    let old_pc = cpu.pc;
    cpu.registers.d = 0x00;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x01);
    assert_flags!(cpu, false, false, false, false);
}

#[test]
fn test_0x15_dec_d() {
    let (mut cpu, mut bus) = setup_test!(&[0x15]);
    let old_pc = cpu.pc;
    cpu.registers.d = 0x10;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x0F);
    assert_flags!(cpu, false, true, true, false);
}

#[test]
fn test_0x16_ld_d_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x16, 0x33]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(cpu.registers.d, 0x33);
}

#[test]
fn test_0x17_rla() {
    let (mut cpu, mut bus) = setup_test!(&[0x17]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x80;
    cpu.registers.f = 0; // Carry clear
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0x00);
    assert_flags!(cpu, false, false, false, true);
}

#[test]
fn test_0x18_jr_e8() {
    let (mut cpu, mut bus) = setup_test!(&[0x18, 0x05]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 7, "JR forward failed");
}

#[test]
#[ignore]
fn test_0x19_add_hl_de() {
    let (mut cpu, mut bus) = setup_test!(&[0x19]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0x8000);
    cpu.registers.set_de(0x8000);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_hl(), 0x0000);
    assert_flags!(cpu, false, false, false, true);
}

#[test]
fn test_0x1a_ld_a_de_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x1A]);
    let old_pc = cpu.pc;
    cpu.registers.set_de(0xC000);
    bus.write_byte(0xC000, 0x11);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0x11);
}

#[test]
#[ignore]
fn test_0x1b_dec_de() {
    let (mut cpu, mut bus) = setup_test!(&[0x1B]);
    let old_pc = cpu.pc;
    cpu.registers.set_de(0x0001);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_de(), 0x0000);
}

#[test]
#[ignore]
fn test_0x1c_inc_e() {
    let (mut cpu, mut bus) = setup_test!(&[0x1C]);
    let old_pc = cpu.pc;
    cpu.registers.e = 0x0F;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, false, false, true, false);
}

#[test]
#[ignore]
fn test_0x1d_dec_e() {
    let (mut cpu, mut bus) = setup_test!(&[0x1D]);
    let old_pc = cpu.pc;
    cpu.registers.e = 0x01;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, true, true, false, false);
}

#[test]
#[ignore]
fn test_0x1e_ld_e_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x1E, 0x99]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(cpu.registers.e, 0x99);
}

#[test]
#[ignore]
fn test_0x1f_rra() {
    let (mut cpu, mut bus) = setup_test!(&[0x1F]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x01;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0x00);
    assert_flags!(cpu, false, false, false, true);
}

#[test]
#[ignore]
fn test_0x20_jr_nz_e8() {
    todo!()
}

#[test]
#[ignore]
fn test_0x21_ld_hl_n16() {
    let (mut cpu, mut bus) = setup_test!(&[0x21, 0x00, 0xD0]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 3);
    assert_eq!(cpu.registers.get_hl(), 0xD000);
}

#[test]
#[ignore]
fn test_0x22_ld_hl_inc_mem_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x22]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0xAA;
    cpu.registers.set_hl(0xC000);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(bus.read_byte(0xC000), 0xAA);
    assert_eq!(cpu.registers.get_hl(), 0xC001);
}

#[test]
#[ignore]
fn test_0x23_inc_hl() {
    let (mut cpu, mut bus) = setup_test!(&[0x23]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0x4444);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_hl(), 0x4445);
}

#[test]
fn test_0x24_inc_h() {
    let (mut cpu, mut bus) = setup_test!(&[0x24]);
    let old_pc = cpu.pc;
    cpu.registers.h = 0x00;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, false, false, false, false);
}

#[test]
#[ignore]
fn test_0x25_dec_h() {
    let (mut cpu, mut bus) = setup_test!(&[0x25]);
    let old_pc = cpu.pc;
    cpu.registers.h = 0x00;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, false, true, true, false);
}

#[test]
#[ignore]
fn test_0x26_ld_h_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x26, 0xFE]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(cpu.registers.h, 0xFE);
}

#[test]
#[ignore]
fn test_0x27_daa() {
    todo!()
}

#[test]
#[ignore]
fn test_0x28_jr_z_e8() {
    todo!()
}

#[test]
#[ignore]
fn test_0x29_add_hl_hl() {
    let (mut cpu, mut bus) = setup_test!(&[0x29]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0x1234);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_hl(), 0x2468);
    assert_flags!(cpu, false, false, false, false);
}

#[test]
#[ignore]
fn test_0x2a_ld_a_hl_inc_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x2A]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x42);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0x42);
    assert_eq!(cpu.registers.get_hl(), 0xC001);
}

#[test]
#[ignore]
fn test_0x2b_dec_hl() {
    let (mut cpu, mut bus) = setup_test!(&[0x2B]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0x0000);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_hl(), 0xFFFF);
}

#[test]
#[ignore]
fn test_0x2c_inc_l() {
    let (mut cpu, mut bus) = setup_test!(&[0x2C]);
    let old_pc = cpu.pc;
    cpu.registers.l = 0x0F;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, false, false, true, false);
}

#[test]
#[ignore]
fn test_0x2d_dec_l() {
    let (mut cpu, mut bus) = setup_test!(&[0x2D]);
    let old_pc = cpu.pc;
    cpu.registers.l = 0x10;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, false, true, true, false);
}

#[test]
#[ignore]
fn test_0x2e_ld_l_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x2E, 0x55]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(cpu.registers.l, 0x55);
}

#[test]
#[ignore]
fn test_0x2f_cpl() {
    todo!()
}

#[test]
#[ignore]
fn test_0x30_jr_nc_e8() {
    todo!()
}

#[test]
#[ignore]
fn test_0x31_ld_sp_n16() {
    let (mut cpu, mut bus) = setup_test!(&[0x31, 0xFF, 0xDF]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 3);
    assert_eq!(cpu.sp, 0xDFFF);
}

#[test]
#[ignore]
fn test_0x32_ld_hl_dec_mem_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x32]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x77;
    cpu.registers.set_hl(0xC001);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(bus.read_byte(0xC001), 0x77);
    assert_eq!(cpu.registers.get_hl(), 0xC000);
}

#[test]
#[ignore]
fn test_0x33_inc_sp() {
    let (mut cpu, mut bus) = setup_test!(&[0x33]);
    let old_pc = cpu.pc;
    cpu.sp = 0xFFFF;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.sp, 0x0000);
}

#[test]
#[ignore]
fn test_0x34_inc_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x34]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x00);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(bus.read_byte(0xC000), 0x01);
    assert_flags!(cpu, false, false, false, false);
}

#[test]
#[ignore]
fn test_0x35_dec_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x35]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x00);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(bus.read_byte(0xC000), 0xFF);
    assert_flags!(cpu, false, true, true, false);
}

#[test]
#[ignore]
fn test_0x36_ld_hl_mem_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x36, 0x12]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(bus.read_byte(0xC000), 0x12);
}

#[test]
#[ignore]
fn test_0x37_scf() {
    todo!()
}

#[test]
#[ignore]
fn test_0x38_jr_c_e8() {
    todo!()
}

#[test]
#[ignore]
fn test_0x39_add_hl_sp() {
    let (mut cpu, mut bus) = setup_test!(&[0x39]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0x0001);
    cpu.sp = 0x0001;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.get_hl(), 0x0002);
    assert_flags!(cpu, false, false, false, false);
}

#[test]
#[ignore]
fn test_0x3a_ld_a_hl_dec_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x3A]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC005);
    bus.write_byte(0xC005, 0x99);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.a, 0x99);
    assert_eq!(cpu.registers.get_hl(), 0xC004);
}

#[test]
#[ignore]
fn test_0x3b_dec_sp() {
    let (mut cpu, mut bus) = setup_test!(&[0x3B]);
    let old_pc = cpu.pc;
    cpu.sp = 0x0000;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.sp, 0xFFFF);
}

#[test]
#[ignore]
fn test_0x3c_inc_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x3C]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0xFF;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, true, false, true, false);
}

#[test]
#[ignore]
fn test_0x3d_dec_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x3D]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x10;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_flags!(cpu, false, true, true, false);
}

#[test]
#[ignore]
fn test_0x3e_ld_a_n8() {
    let (mut cpu, mut bus) = setup_test!(&[0x3E, 0x42]);
    let old_pc = cpu.pc;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 2);
    assert_eq!(cpu.registers.a, 0x42);
}

#[test]
#[ignore]
fn test_0x3f_ccf() {
    todo!()
}

#[test]
#[ignore]
fn test_0x40_ld_b_b() {
    let (mut cpu, mut bus) = setup_test!(&[0x40]);
    let old_pc = cpu.pc;
    cpu.registers.b = 0x11;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x11);
}

#[test]
#[ignore]
fn test_0x41_ld_b_c() {
    let (mut cpu, mut bus) = setup_test!(&[0x41]);
    let old_pc = cpu.pc;
    cpu.registers.c = 0x22;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x22);
}

#[test]
#[ignore]
fn test_0x42_ld_b_d() {
    let (mut cpu, mut bus) = setup_test!(&[0x42]);
    let old_pc = cpu.pc;
    cpu.registers.d = 0x33;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x33);
}

#[test]
#[ignore]
fn test_0x43_ld_b_e() {
    let (mut cpu, mut bus) = setup_test!(&[0x43]);
    let old_pc = cpu.pc;
    cpu.registers.e = 0x44;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x44);
}

#[test]
#[ignore]
fn test_0x44_ld_b_h() {
    let (mut cpu, mut bus) = setup_test!(&[0x44]);
    let old_pc = cpu.pc;
    cpu.registers.h = 0x55;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x55);
}

#[test]
#[ignore]
fn test_0x45_ld_b_l() {
    let (mut cpu, mut bus) = setup_test!(&[0x45]);
    let old_pc = cpu.pc;
    cpu.registers.l = 0x66;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x66);
}

#[test]
#[ignore]
fn test_0x46_ld_b_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x46]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x77);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x77);
}

#[test]
#[ignore]
fn test_0x47_ld_b_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x47]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x88;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.b, 0x88);
}

#[test]
#[ignore]
fn test_0x48_ld_c_b() {
    let (mut cpu, mut bus) = setup_test!(&[0x48]);
    let old_pc = cpu.pc;
    cpu.registers.b = 0x11;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x11);
}

#[test]
#[ignore]
fn test_0x49_ld_c_c() {
    let (mut cpu, mut bus) = setup_test!(&[0x49]);
    let old_pc = cpu.pc;
    cpu.registers.c = 0x22;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
}

#[test]
#[ignore]
fn test_0x4a_ld_c_d() {
    let (mut cpu, mut bus) = setup_test!(&[0x4A]);
    let old_pc = cpu.pc;
    cpu.registers.d = 0x33;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x33);
}

#[test]
#[ignore]
fn test_0x4b_ld_c_e() {
    let (mut cpu, mut bus) = setup_test!(&[0x4B]);
    let old_pc = cpu.pc;
    cpu.registers.e = 0x44;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x44);
}

#[test]
#[ignore]
fn test_0x4c_ld_c_h() {
    let (mut cpu, mut bus) = setup_test!(&[0x4C]);
    let old_pc = cpu.pc;
    cpu.registers.h = 0x55;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x55);
}

#[test]
#[ignore]
fn test_0x4d_ld_c_l() {
    let (mut cpu, mut bus) = setup_test!(&[0x4D]);
    let old_pc = cpu.pc;
    cpu.registers.l = 0x66;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x66);
}

#[test]
#[ignore]
fn test_0x4e_ld_c_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x4E]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x77);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x77);
}

#[test]
#[ignore]
fn test_0x4f_ld_c_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x4F]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x88;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.c, 0x88);
}

#[test]
#[ignore]
fn test_0x50_ld_d_b() {
    let (mut cpu, mut bus) = setup_test!(&[0x50]);
    let old_pc = cpu.pc;
    cpu.registers.b = 0x11;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x11);
}

#[test]
#[ignore]
fn test_0x51_ld_d_c() {
    let (mut cpu, mut bus) = setup_test!(&[0x51]);
    let old_pc = cpu.pc;
    cpu.registers.c = 0x22;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x22);
}

#[test]
#[ignore]
fn test_0x52_ld_d_d() {
    let (mut cpu, mut bus) = setup_test!(&[0x52]);
    let old_pc = cpu.pc;
    cpu.registers.d = 0x33;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
}

#[test]
#[ignore]
fn test_0x53_ld_d_e() {
    let (mut cpu, mut bus) = setup_test!(&[0x53]);
    let old_pc = cpu.pc;
    cpu.registers.e = 0x44;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x44);
}

#[test]
#[ignore]
fn test_0x54_ld_d_h() {
    let (mut cpu, mut bus) = setup_test!(&[0x54]);
    let old_pc = cpu.pc;
    cpu.registers.h = 0x55;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x55);
}

#[test]
#[ignore]
fn test_0x55_ld_d_l() {
    let (mut cpu, mut bus) = setup_test!(&[0x55]);
    let old_pc = cpu.pc;
    cpu.registers.l = 0x66;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x66);
}

#[test]
#[ignore]
fn test_0x56_ld_d_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x56]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x77);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x77);
}

#[test]
#[ignore]
fn test_0x57_ld_d_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x57]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x88;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.d, 0x88);
}

#[test]
#[ignore]
fn test_0x58_ld_e_b() {
    let (mut cpu, mut bus) = setup_test!(&[0x58]);
    let old_pc = cpu.pc;
    cpu.registers.b = 0x11;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.e, 0x11);
}

#[test]
#[ignore]
fn test_0x59_ld_e_c() {
    let (mut cpu, mut bus) = setup_test!(&[0x59]);
    let old_pc = cpu.pc;
    cpu.registers.c = 0x22;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.e, 0x22);
}

#[test]
#[ignore]
fn test_0x5a_ld_e_d() {
    let (mut cpu, mut bus) = setup_test!(&[0x5A]);
    let old_pc = cpu.pc;
    cpu.registers.d = 0x33;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.e, 0x33);
}

#[test]
#[ignore]
fn test_0x5b_ld_e_e() {
    let (mut cpu, mut bus) = setup_test!(&[0x5B]);
    let old_pc = cpu.pc;
    cpu.registers.e = 0x44;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
}

#[test]
#[ignore]
fn test_0x5c_ld_e_h() {
    let (mut cpu, mut bus) = setup_test!(&[0x5C]);
    let old_pc = cpu.pc;
    cpu.registers.h = 0x55;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.e, 0x55);
}

#[test]
#[ignore]
fn test_0x5d_ld_e_l() {
    let (mut cpu, mut bus) = setup_test!(&[0x5D]);
    let old_pc = cpu.pc;
    cpu.registers.l = 0x66;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.e, 0x66);
}

#[test]
#[ignore]
fn test_0x5e_ld_e_hl_mem() {
    let (mut cpu, mut bus) = setup_test!(&[0x5E]);
    let old_pc = cpu.pc;
    cpu.registers.set_hl(0xC000);
    bus.write_byte(0xC000, 0x77);
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.e, 0x77);
}

#[test]
#[ignore]
fn test_0x5f_ld_e_a() {
    let (mut cpu, mut bus) = setup_test!(&[0x5F]);
    let old_pc = cpu.pc;
    cpu.registers.a = 0x88;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.e, 0x88);
}

#[test]
#[ignore]
fn test_0x60_ld_h_b() {
    let (mut cpu, mut bus) = setup_test!(&[0x60]);
    let old_pc = cpu.pc;
    cpu.registers.b = 0x11;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.h, 0x11);
}

#[test]
#[ignore]
fn test_0x61_ld_h_c() {
    let (mut cpu, mut bus) = setup_test!(&[0x61]);
    let old_pc = cpu.pc;
    cpu.registers.c = 0x22;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.h, 0x22);
}

#[test]
#[ignore]
fn test_0x62_ld_h_d() {
    let (mut cpu, mut bus) = setup_test!(&[0x62]);
    let old_pc = cpu.pc;
    cpu.registers.d = 0x33;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.h, 0x33);
}

#[test]
#[ignore]
fn test_0x63_ld_h_e() {
    let (mut cpu, mut bus) = setup_test!(&[0x63]);
    let old_pc = cpu.pc;
    cpu.registers.e = 0x44;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc - old_pc, 1);
    assert_eq!(cpu.registers.h, 0x44);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xD3 encountered")]
fn test_0xd3_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xD3]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xDB encountered")]
fn test_0xdb_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xDB]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xDD encountered")]
fn test_0xdd_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xDD]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xE3 encountered")]
fn test_0xe3_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xE3]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xE4 encountered")]
fn test_0xe4_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xE4]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xEB encountered")]
fn test_0xeb_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xEB]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xEC encountered")]
fn test_0xec_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xEC]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xED encountered")]
fn test_0xed_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xED]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xF4 encountered")]
fn test_0xf4_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xF4]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xFC encountered")]
fn test_0xfc_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xFC]);
    cpu.step(&mut bus);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xFD encountered")]
fn test_0xfd_illegal_opcode() {
    let (mut cpu, mut bus) = setup_test!(&[0xFD]);
    cpu.step(&mut bus);
}
