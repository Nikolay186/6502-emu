pub enum StatusRegisterBitFlag {
    C = 0,
    Z,
    I,
    D,
    B,
    // 5 bit is ignored
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
