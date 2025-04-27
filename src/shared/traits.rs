use super::types::{Byte, Word};

pub trait ToByte {
    fn to_byte(self) -> Byte;
}

pub trait ToWord {
    fn to_word(self) -> Word;
}

impl ToByte for bool {
    fn to_byte(self) -> Byte {
        u8::from(self)
    }
}

impl ToWord for bool {
    fn to_word(self) -> Word {
        u16::from(self)
    }
}

impl ToWord for u8 {
    fn to_word(self) -> Word {
        u16::from(self)
    }
}
