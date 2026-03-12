#[derive(Default)]
pub struct Ppu {
    pub llcd: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, cycles: u8) -> bool {
        todo!()
    }
}
