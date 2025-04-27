use crate::shared::traits::ToByte;
use crate::shared::types::Byte;

use super::status_register_bitflag_enum::StatusRegisterBitFlag;

pub(in crate::cpu) struct StatusRegister {
    // data used as u8 storage
    // status register flags (bit 7-0)
    // N - negative  (7)
    // V - overflow  (6)
    // _ - ignored   (5)
    // B - break     (4)
    // D - decimal   (3)
    // I - interrupt (2)
    // Z - zero      (1)
    // C - carry     (0)
    _data: Byte,
}

impl StatusRegister {
    // init new empty status register
    pub fn new() -> Self {
        Self { _data: 0b00000000 }
    }

    pub fn set_val(&mut self, bit: StatusRegisterBitFlag, value: bool) {
        let shift: Byte = bit.into();
        // to be sure that function accepts only 0 or 1
        let value = value.to_byte();

        self._data =
            (self._data & !((1 << shift) & !((value & 1) << shift))) | ((value & 1) << shift);
    }

    pub fn get_val(&self, bit: StatusRegisterBitFlag) -> Byte {
        let shift: Byte = bit.into();
        (self._data & (1 << shift)) >> shift & 1
    }

    // set all status bits to zero
    pub fn clear(&mut self) {
        self._data = 0b00000000;
    }
}
