pub enum StatusRegisterBitFlag {
    C = 0,
    Z,
    I,
    D,
    B,
    // 5 bit is ignored, always set as 1
    V = 6,
    N,
}

impl From<u8> for StatusRegisterBitFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => StatusRegisterBitFlag::C,
            1 => StatusRegisterBitFlag::Z,
            2 => StatusRegisterBitFlag::I,
            3 => StatusRegisterBitFlag::D,
            4 => StatusRegisterBitFlag::B,
            6 => StatusRegisterBitFlag::V,
            7 => StatusRegisterBitFlag::N,
            _ => panic!("Invalid status register bit: {value}"),
        }
    }
}

impl From<StatusRegisterBitFlag> for u8 {
    fn from(flag: StatusRegisterBitFlag) -> Self {
        match flag {
            StatusRegisterBitFlag::C => 0,
            StatusRegisterBitFlag::Z => 1,
            StatusRegisterBitFlag::I => 2,
            StatusRegisterBitFlag::D => 3,
            StatusRegisterBitFlag::B => 4,
            StatusRegisterBitFlag::V => 6,
            StatusRegisterBitFlag::N => 7,
        }
    }
}
