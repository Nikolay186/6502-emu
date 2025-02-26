use crate::{
    memory::memory::Memory,
    shared_types::{Byte, Word},
};

use super::status_register::status_register::StatusRegister;

// LE
pub struct CPU {
    // registers
    status_reg: StatusRegister, // SR
    acc: Byte,                  // AC
    x_reg: Byte,                // XR
    y_reg: Byte,                // YR
    program_counter: Word,      // PC
    stack_ptr: Byte,            // SP

    memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            stack_ptr: 0,
            acc: 0,
            x_reg: 0,
            y_reg: 0,
            status_reg: StatusRegister::new(),

            memory: Memory::new(None),
        }
    }

    pub fn reset(&mut self) {
        // TODO: implement specific memory addresses enum?
        self.program_counter = self.memory.read(0xFFFC).unwrap().into(); // u8(Byte) -> u16(Word)

        // Stack 8 bit range 0x0100 - 0x01FF
        self.stack_ptr = 0x0000;

        self.status_reg.clear();

        self.acc = 0b00000000;
        self.x_reg = 0b00000000;
        self.y_reg = 0b00000000;
    }
}
