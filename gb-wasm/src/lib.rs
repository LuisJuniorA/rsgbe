use gb_core::emulator::Emulator;
use gb_core::joypad::JoypadKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmEmulator {
    emu: Emulator,
}

#[wasm_bindgen]
impl WasmEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: &[u8], save_data: Option<Vec<u8>>) -> Self {
        console_error_panic_hook::set_once();
        Self {
            emu: Emulator::new(rom.to_vec(), save_data),
        }
    }

    pub fn get_save_data(&self) -> Option<Vec<u8>> {
        self.emu.get_save_data()
    }

    pub fn get_audio_buffer(&mut self) -> Vec<f32> {
        self.emu.get_audio_buffer()
    }

    pub fn clock_frame(&mut self) {
        let mut cycles_count: u32 = 0;
        while cycles_count < 70224 {
            cycles_count += self.emu.step() as u32;
        }
    }

    pub fn get_frame_buffer_ptr(&self) -> *const u8 {
        self.emu.bus.ppu.framebuffer.as_ptr()
    }

    pub fn key_down(&mut self, key: u8) {
        if let Some(k) = Self::map_key(key) {
            let was_unpressed = self.emu.bus.joypad.key_down(k);
            if was_unpressed {
                self.emu.bus.if_reg |= 0x10; // Trigger interruption Joypad
            }
        }
    }

    pub fn key_up(&mut self, key: u8) {
        if let Some(k) = Self::map_key(key) {
            self.emu.bus.joypad.key_up(k);
        }
    }

    fn map_key(key: u8) -> Option<JoypadKey> {
        match key {
            0 => Some(JoypadKey::Right),
            1 => Some(JoypadKey::Left),
            2 => Some(JoypadKey::Up),
            3 => Some(JoypadKey::Down),
            4 => Some(JoypadKey::A),
            5 => Some(JoypadKey::B),
            6 => Some(JoypadKey::Select),
            7 => Some(JoypadKey::Start),
            _ => None,
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.emu.cpu.pc
    }
    pub fn get_sp(&self) -> u16 {
        self.emu.cpu.sp
    }
    pub fn get_ly(&self) -> u8 {
        self.emu.bus.ppu.ly
    }
    pub fn get_mode(&self) -> u8 {
        self.emu.bus.ppu.stat & 0x03
    }
}
