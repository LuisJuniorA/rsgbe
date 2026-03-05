use gb_core::{cpu::Cpu, memory::Bus};

#[test]
fn test_cpu_nop() {
    let mut rom = vec![0x00; 0x0100];
    rom.push(0x00);

    let mut bus = Bus::new(rom);
    let mut cpu = Cpu::new();

    let old_pc = cpu.pc;

    cpu.step(&mut bus);

    assert_eq!(cpu.pc - old_pc, 1, "PC should increment by 1 after NOP");
}

#[test]
fn test_cpu_ld_bc_d16() {
    let mut rom = vec![0x00; 0x0100];

    rom.extend_from_slice(&[0x01, 0x34, 0x12]);

    let mut bus = Bus::new(rom);
    let mut cpu = Cpu::new();

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
}

#[test]
#[should_panic(expected = "Illegal opcode 0xD3 encountered")]
fn test_cpu_illegal_opcode_d3() {
    let mut rom = vec![0x00; 0x0100];

    rom.extend_from_slice(&[0xD3]);

    let mut bus = Bus::new(rom);
    let mut cpu = Cpu::new();

    cpu.step(&mut bus);
}
