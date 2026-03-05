use crate::registers::Registers;

struct Cpu {
    register: Registers, // Register
    pc : u8, // Program Counter
    sp: u8 // Stack Pointer
}