use crate::shared_types::Byte;

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
        Self { _data: 0 }
    }

    pub fn set_val(&mut self, bit: StatusRegisterBitFlag, value: bool) -> u8 {
        let shift: u8 = bit as u8;

        if value {
            self._data |= 1 << shift;
        } else {
            self._data &= !(1 << shift);
        }

        self._data & (1 << shift)
    }

    pub fn get_val(&self, bit: StatusRegisterBitFlag) -> u8 {
        let shift: u8 = bit as u8;

        self._data & (1 << shift)
    }

    // set all status bits to zero
    pub fn clear(&mut self) {
        self._data = 0b00000000;
    }
}
