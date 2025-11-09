use crate::{
    cpu::{cpu::CPU, status_register::status_register_bitflag_enum::StatusRegisterBitFlag},
    shared::{traits::ToWord, types::Byte},
};

impl CPU {
    // 2-byte instruction, $nn
    pub fn lda_zero_page(&mut self) -> Byte {
        let target_address = self.fetch_byte().to_word();

        let value = self.read_byte(target_address);

        self.acc = value;

        let updated_zero_flag = value == 0x00;
        self.status_reg
            .set_val(StatusRegisterBitFlag::Z, updated_zero_flag);

        let updated_negative_flag = (value & 0x80) != 0;
        self.status_reg
            .set_val(StatusRegisterBitFlag::N, updated_negative_flag);

        3
    }
}
