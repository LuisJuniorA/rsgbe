pub trait MBC {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);
    fn get_save_data(&self) -> Option<&[u8]> {
        None
    }
}
