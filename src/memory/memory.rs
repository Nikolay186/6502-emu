use crate::shared::types::{Byte, Word};

use super::memory_errors::MemoryError;

const MEMORY_SIZE: usize = 1024 * 64;
// map ROM address space as in NES - 0x8000 - 0xFFFF
const ROM_ADDRESS_RANGE_START: usize = 0x8000;
const ROM_ADDRESS_RANGE_END: usize = 0xFFFF;

pub struct Memory {
    // 6502 has 16 bit address bus, which gives it 64K(65536B) address space
    data: [Byte; MEMORY_SIZE],
}

impl Memory {
    pub fn new(path_to_rom: Option<&str>) -> Self {
        let memory = Memory {
            data: [0x0000; MEMORY_SIZE],
        };

        // TODO: optionally load rom file on initialization, will decide loading flow later
        match path_to_rom {
            Some(str) => memory.load_bin(str),
            None => (),
        }

        memory
    }

    // load rom file from OS fs
    fn load_bin(&self, path: &str) {
        todo!()
    }

    pub fn read(&self, address: Word) -> Result<Byte, MemoryError> {
        if (address as usize) >= MEMORY_SIZE {
            return Err(MemoryError::AddressOutOfBounds(address));
        }

        Ok(self.data[address as usize])
    }

    pub fn write(&mut self, address: Word, value: Byte) -> Result<(), MemoryError> {
        if (address as usize) >= MEMORY_SIZE {
            return Err(MemoryError::AddressOutOfBounds(address));
        }

        if (address as usize) >= ROM_ADDRESS_RANGE_START {
            return Err(MemoryError::RomWriteAttempt(address));
        }

        self.data[address as usize] = value;

        Ok(())
    }
}
