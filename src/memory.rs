use crate::errors::MemoryError;

const MEMORY_SIZE: usize = 1_048_576; // 1 MB

pub struct Memory {
    data: Box<[u8]>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![0; MEMORY_SIZE].into_boxed_slice(),
        }
    }

    pub fn read_byte(&self, address: usize) -> Result<u8, MemoryError> {
        if address >= self.data.len() {
            Err(MemoryError::OutOfBounds)
        } else {
            Ok(self.data[address])
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) -> Result<(), MemoryError> {
        if address >= self.data.len() {
            Err(MemoryError::OutOfBounds)
        } else {
            self.data[address] = value;
            Ok(())
        }
    }

    pub fn read_word(&self, address: usize) -> Result<u16, MemoryError> {
        if address + 1 >= self.data.len() {
            return Err(MemoryError::OutOfBounds);
        } else {
            let low = self.read_byte(address)?;
            let high = self.read_byte(address + 1)?;

            Ok(u16::from_le_bytes([low, high]))
        }
    }

    pub fn write_word(&mut self, address: usize, value: u16) -> Result<(), MemoryError> {
        if address + 1 >= self.data.len() {
            return Err(MemoryError::OutOfBounds);
        } else {
            let bytes = value.to_le_bytes();

            self.write_byte(address, bytes[0])?; // low
            self.write_byte(address + 1, bytes[1])?; // high

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_byte() {
        let mut memory = Memory::new();

        assert_eq!(memory.read_byte(0).unwrap(), 0);

        memory.write_byte(14, 123).unwrap();
        assert_eq!(memory.read_byte(14).unwrap(), 123);

        assert!(matches!(
            memory.read_byte(MEMORY_SIZE),
            Err(MemoryError::OutOfBounds)
        ));
    }

    #[test]
    fn test_write_byte() {
        let mut memory = Memory::new();

        memory.write_byte(12, 24).unwrap();
        assert_eq!(memory.read_byte(12).unwrap(), 24);

        assert!(matches!(
            memory.write_byte(MEMORY_SIZE, 64),
            Err(MemoryError::OutOfBounds)
        ));
    }

    #[test]
    fn test_read_word() {
        let mut memory = Memory::new();

        memory.write_word(0, 0x1234).unwrap();

        assert_eq!(memory.read_word(0).unwrap(), 0x1234);
        assert!(matches!(
            memory.read_word(MEMORY_SIZE - 1),
            Err(MemoryError::OutOfBounds)
        ))
    }

    #[test]
    fn test_write_word() {
        let mut memory = Memory::new();

        memory.write_word(0, 0x1234).unwrap();
        assert_eq!(memory.read_word(0).unwrap(), 0x1234);

        assert_eq!(memory.read_byte(0).unwrap(), 0x34);
        assert_eq!(memory.read_byte(1).unwrap(), 0x12);

        assert!(matches!(
            memory.write_word(MEMORY_SIZE - 1, 123),
            Err(MemoryError::OutOfBounds)
        ));
    }
}
