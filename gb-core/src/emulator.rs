use crate::cpu::Cpu;
use crate::memory::Bus;

pub struct Emulator {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(rom),
        }
    }

    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step(&mut self.bus);
        self.bus.tick(cycles);

        cycles
    }
}
