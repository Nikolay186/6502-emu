use crate::{
    memory::memory::Memory,
    shared::{
        logger::LoggingHw,
        traits::ToWord,
        types::{Byte, Word},
    },
};

use super::status_register::status_register::StatusRegister;

// LE
pub struct CPU {
    // registers
    pub(super) status_reg: StatusRegister, // SR
    pub(super) acc: Byte,                  // AC
    pub(super) x_reg: Byte,                // XR
    pub(super) y_reg: Byte,                // YR
    pub(super) program_counter: Word,      // PC
    pub(super) stack_ptr: Byte,            // SP

    // TODO: add memory bus to decouple it from CPU
    pub(super) memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            program_counter: 0x0000,
            stack_ptr: 0x00,
            acc: 0x00,
            x_reg: 0x00,
            y_reg: 0x00,
            status_reg: StatusRegister::new(),
            memory: Memory::new(None),
        }
    }

    pub fn reset(&mut self) {
        // TODO: implement specific memory addresses enum?
        self.program_counter = self.read_byte(0xFFFC).to_word();

        // Stack 8 bit range 0x0100 - 0x01FF
        self.stack_ptr = 0xFF;

        self.status_reg.clear();

        self.acc = 0x00;
        self.x_reg = 0x00;
        self.y_reg = 0x00;
    }

    pub fn execute_loop(&mut self) {
        let opcode = self.fetch_byte();

        let consumed_cycles = match opcode {
            0xA9 => self.lda_immediate(),
            0xA5 => self.lda_zero_page(),
            0xB5 => self.lda_zero_page_x(),
            0xAD => self.lda_absolute(),
            0xBD => self.lda_absolute_x(),
            0xB9 => self.lda_absolute_y(),
            0xA1 => self.lda_indirect_x(),
            0xB1 => self.lda_indirect_y(),
            _ => {
                todo!()
            }
        };
    }

    pub fn fetch_byte(&mut self) -> Byte {
        let pc_value = self.program_counter;
        let fetch_result = self.memory.read(pc_value);
        self.program_counter += 1;

        match fetch_result {
            Ok(value) => value,
            Err(error) => {
                self.log_error("fetch_byte", error.to_string().as_str());
                panic!("Memory read error");
            }
        }
    }

    pub fn fetch_word(&mut self) -> Word {
        let low_byte = self.fetch_byte().to_word();
        let high_byte = self.fetch_byte().to_word();

        (high_byte << 8) | low_byte
    }

    pub fn read_byte(&self, addr: Word) -> Byte {
        match self.memory.read(addr) {
            Ok(value) => value,
            Err(error) => {
                self.log_error("read_byte", error.to_string().as_str());
                panic!("Memory read error");
            }
        }
    }

    pub fn read_word(&self, addr: Word) -> Word {
        let low_byte = self.read_byte(addr);
        let high_byte = self.read_byte(addr + 1);

        (high_byte.to_word() << 8) | low_byte.to_word()
    }
}

impl LoggingHw for CPU {
    fn hw_name(&self) -> &'static str {
        "CPU"
    }

    // TODO: add cycle counting
    fn get_ctx(&self) -> Option<String> {
        Some(format!(
            "SP={}, PC={}, REG_A={}, REG_X={}, REG_Y={}, STATUS_REG={}",
            self.stack_ptr, self.program_counter, self.acc, self.x_reg, self.y_reg, self.status_reg
        ))
    }
}
