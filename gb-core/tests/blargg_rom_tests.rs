use gb_core::{cpu::Cpu, memory::Bus};
use std::fs;
use std::path::PathBuf;

fn run_blargg_test(rom_name: &str) {
    // The test ROMs are stored at the root of the workspace in `test/roms`
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/roms");
    path.push(rom_name);

    let rom = match fs::read(&path) {
        Ok(data) => data,
        Err(e) => {
            panic!("Skipping Blargg test, ROM not found at {:?}: {}", path, e);
        }
    };

    let mut bus = Bus::new(rom, None);
    let mut cpu = Cpu::new();

    let mut cycles = 0;
    let max_cycles = 250_000_000;

    while cycles < max_cycles {
        let step_cycles = cpu.step(&mut bus);
        bus.tick(step_cycles);
        cycles += step_cycles as u64;

        if bus.serial_output.contains("Passed") {
            println!("Test {} output:\n{}", rom_name, bus.serial_output);
            return; // Success
        }

        if bus.serial_output.contains("Failed") {
            panic!(
                "Blargg test {} failed!\nOutput:\n{}",
                rom_name, bus.serial_output
            );
        }
    }

    panic!(
        "Blargg test {} timed out! Output so far:\n{}",
        rom_name, bus.serial_output
    );
}

#[test]
fn test_01_special() {
    run_blargg_test("01-special.gb");
}

#[test]
fn test_02_interrupts() {
    run_blargg_test("02-interrupts.gb");
}

#[test]
fn test_03_op_sp_hl() {
    run_blargg_test("03-op sp,hl.gb");
}

#[test]
fn test_04_op_r_imm() {
    run_blargg_test("04-op r,imm.gb");
}

#[test]
fn test_05_op_rp() {
    run_blargg_test("05-op rp.gb");
}

#[test]
fn test_06_ld_r_r() {
    run_blargg_test("06-ld r,r.gb");
}

#[test]
fn test_07_jr_jp_call_ret_rst() {
    run_blargg_test("07-jr,jp,call,ret,rst.gb");
}

#[test]
fn test_08_misc_instrs() {
    run_blargg_test("08-misc instrs.gb");
}

#[test]
fn test_09_op_r_r() {
    run_blargg_test("09-op r,r.gb");
}

#[test]
fn test_10_bit_ops() {
    run_blargg_test("10-bit ops.gb");
}

#[test]
fn test_11_op_a_hl() {
    run_blargg_test("11-op a,(hl).gb");
}
