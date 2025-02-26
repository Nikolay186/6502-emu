use chrono::Utc;
use std::fmt;

#[derive(Debug)]
pub enum MemoryError {
    AddressOutOfBounds(u16),
    RomWriteAttempt(u16),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let now_date = Utc::now()
            .date_naive()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        match self {
            MemoryError::AddressOutOfBounds(address) => {
                writeln!(f, "\x1b[93m[ERROR]\x1b[0m [{:#}]: Tried to access memory at {:#06X}, address is out of bounds", now_date, address)
            }
            MemoryError::RomWriteAttempt(address) => {
                writeln!(f, "\x1b[93m[ERROR]\x1b[0m [{:#}]: Tried to write memory at {:#06X}, address belongs to ROM", now_date, address)
            }
        }
    }
}
