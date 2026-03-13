#[derive(Clone, Copy)]
pub enum JoypadKey {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

pub struct Joypad {
    action: u8,
    dpad: u8,
    selection: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            action: 0x0F, // 0 = pressed, 1 = unpressed
            dpad: 0x0F,
            selection: 0x30,
        }
    }

    pub fn write(&mut self, val: u8) {
        // Bits 4 & 5 determine which buttons the CPU wants to read
        self.selection = val & 0x30;
    }

    pub fn read(&self) -> u8 {
        let mut result = 0xCF; // Hardware quirk: Top 2 bits are always 1

        if self.selection & 0x10 == 0 {
            result &= self.dpad;
        }
        if self.selection & 0x20 == 0 {
            result &= self.action;
        }

        result | self.selection
    }

    pub fn key_down(&mut self, key: JoypadKey) -> bool {
        match key {
            JoypadKey::Right => Self::clear_bit(&mut self.dpad, 0),
            JoypadKey::Left => Self::clear_bit(&mut self.dpad, 1),
            JoypadKey::Up => Self::clear_bit(&mut self.dpad, 2),
            JoypadKey::Down => Self::clear_bit(&mut self.dpad, 3),
            JoypadKey::A => Self::clear_bit(&mut self.action, 0),
            JoypadKey::B => Self::clear_bit(&mut self.action, 1),
            JoypadKey::Select => Self::clear_bit(&mut self.action, 2),
            JoypadKey::Start => Self::clear_bit(&mut self.action, 3),
        }
    }

    pub fn key_up(&mut self, key: JoypadKey) {
        match key {
            JoypadKey::Right => self.dpad |= 1 << 0,
            JoypadKey::Left => self.dpad |= 1 << 1,
            JoypadKey::Up => self.dpad |= 1 << 2,
            JoypadKey::Down => self.dpad |= 1 << 3,
            JoypadKey::A => self.action |= 1 << 0,
            JoypadKey::B => self.action |= 1 << 1,
            JoypadKey::Select => self.action |= 1 << 2,
            JoypadKey::Start => self.action |= 1 << 3,
        }
    }

    fn clear_bit(register: &mut u8, bit: u8) -> bool {
        let was_unpressed = (*register & (1 << bit)) != 0;
        *register &= !(1 << bit);
        was_unpressed
    }
}
