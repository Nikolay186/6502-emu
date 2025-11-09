use crate::{
    cpu::{cpu::CPU, status_register::status_register_bitflag_enum::StatusRegisterBitFlag},
    shared::{traits::ToWord, types::Byte},
};

impl CPU {
    // 2-byte instruction, ($nn),Y
    pub fn lda_indirect_y(&mut self) -> Byte {
        let zero_page_address = self.fetch_byte().to_word();

        let address_low = self.read_byte(zero_page_address).to_word();
        let address_high = self.read_byte((zero_page_address + 1) & 0xFF).to_word();

        let base_address = (address_high << 8) | address_low;

        let target_address = base_address + self.y_reg.to_word();

        let page_crossed = base_address & 0xFF00 != target_address & 0xFF00;

        let value = self.read_byte(target_address);
        self.acc = value;

        let updated_zero_flag = value == 0x00;
        self.status_reg
            .set_val(StatusRegisterBitFlag::Z, updated_zero_flag);

        let updated_negative_flag = (value & 0x80) != 0;
        self.status_reg
            .set_val(StatusRegisterBitFlag::N, updated_negative_flag);

        match page_crossed {
            true => 6,
            false => 5,
        }
    }
}
