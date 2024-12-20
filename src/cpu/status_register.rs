use crate::shared_types::Byte;

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
    pub(super) _data: Byte,
}

impl StatusRegister {
    // init new empty status register
    pub(super) fn new() -> Self {
        Self { _data: 0 }
    }

    // shift by 7
    pub(super) fn get_n(&self) -> u8 {
        let shift: u8 = 7;

        (*self)._data & (1 << shift)
    }
    pub(super) fn set_n(&mut self, value: bool) -> u8 {
        let shift: u8 = 7;

        self.set_val(shift, value)
    }

    // shift by 6
    pub(super) fn get_v(&self) -> u8 {
        let shift: u8 = 6;

        (*self)._data & (1 << shift)
    }
    pub(super) fn set_v(&mut self, value: bool) -> u8 {
        let shift: u8 = 6;

        self.set_val(shift, value)
    }

    // shift by 4
    pub(super) fn get_b(&self) -> u8 {
        let shift: u8 = 4;

        (*self)._data & (1 << shift)
    }
    pub(super) fn set_b(&mut self, value: bool) -> u8 {
        let shift: u8 = 4;

        self.set_val(shift, value)
    }

    // shift by 3
    pub(super) fn get_d(&self) -> u8 {
        let shift: u8 = 3;

        (*self)._data & (1 << shift)
    }
    pub(super) fn set_d(&mut self, value: bool) -> u8 {
        let shift: u8 = 3;

        self.set_val(shift, value)
    }

    // shift by 2
    pub(super) fn get_i(&self) -> u8 {
        let shift: u8 = 2;

        (*self)._data & (1 << shift)
    }
    pub(super) fn set_i(&mut self, value: bool) -> u8 {
        let shift: u8 = 2;

        self.set_val(shift, value)
    }

    // shift by 1
    pub(super) fn get_z(&self) -> u8 {
        let shift: u8 = 1;

        (*self)._data & (1 << shift)
    }
    pub(super) fn set_z(&mut self, value: bool) -> u8 {
        let shift: u8 = 1;

        self.set_val(shift, value)
    }

    // shift by 0
    pub(super) fn get_c(&self) -> u8 {
        let shift: u8 = 0;

        self._data & (1 << shift)
    }
    pub(super) fn set_c(&mut self, value: bool) -> u8 {
        let shift: u8 = 0;

        self.set_val(shift, value)
    }

    fn set_val(&mut self, shift: u8, value: bool) -> u8 {
        if value {
            self._data |= 1 << shift;
        } else {
            self._data &= !(1 << shift);
        }

        (*self)._data & (1 << shift)
    }
}
