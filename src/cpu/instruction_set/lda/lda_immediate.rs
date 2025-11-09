use crate::{
    cpu::{cpu::CPU, status_register::status_register_bitflag_enum::StatusRegisterBitFlag},
    shared::types::Byte,
};

impl CPU {
    // 2-byte instruction, #$nn
    pub fn lda_immediate(&mut self) -> Byte {
        let value = self.fetch_byte();

        self.acc = value;

        let updated_zero_flag = value == 0x00;
        self.status_reg
            .set_val(StatusRegisterBitFlag::Z, updated_zero_flag);

        let updated_negative_flag = (value & 0x80) != 0;
        self.status_reg
            .set_val(StatusRegisterBitFlag::N, updated_negative_flag);

        2
    }
}
