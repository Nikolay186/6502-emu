use crate::{
    cpu::{cpu::CPU, status_register::status_register_bitflag_enum::StatusRegisterBitFlag},
    shared::{traits::ToWord, types::Byte},
};

impl CPU {
    // 2-byte instruction, ($nn,X)
    pub fn lda_indirect_x(&mut self) -> Byte {
        let zero_page_base_address = self.fetch_byte();

        let pointer_address = ((zero_page_base_address + self.x_reg) & 0xFF).to_word();

        let address_low = self.read_byte(pointer_address).to_word();
        let address_high = self.read_byte((pointer_address + 1) & 0xFF).to_word();
        let target_address = (address_high << 8) | address_low;

        let value = self.read_byte(target_address);

        self.acc = value;

        let updated_zero_flag = value == 0x00;
        self.status_reg
            .set_val(StatusRegisterBitFlag::Z, updated_zero_flag);

        let updated_negative_flag = (value & 0x80) != 0;
        self.status_reg
            .set_val(StatusRegisterBitFlag::N, updated_negative_flag);

        6
    }
}
