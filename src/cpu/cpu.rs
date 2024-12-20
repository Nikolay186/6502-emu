use crate::shared_types::{Byte, Word};

use super::status_register::StatusRegister;

// LE
pub struct CPU {
    // registers
    status_reg: StatusRegister, // SR
    acc: Byte,                  // AC
    x_reg: Byte,                // XR
    y_reg: Byte,                // YR
    program_counter: Word,      // PC
    stack_ptr: Byte,            // SP
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
        }
    }
}
