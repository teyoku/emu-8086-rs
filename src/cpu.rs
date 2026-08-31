use crate::{
    memory::{Memory, MemoryError},
    registers::Registers,
};

#[derive(Default)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    fn call_physical_address(segment: u16, offset: u16) -> usize {
        ((segment as usize) * 16 + offset as usize) as usize
    }

    pub fn fetch_byte(&mut self, memory: &Memory) -> Result<u8, MemoryError> {
        let address = Self::call_physical_address(self.registers.cs, self.registers.ip);
        let byte = memory.read_byte(address)?;

        self.registers.ip = self.registers.ip.wrapping_add(1);
        Ok(byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        let mut memory = Memory::new();
        memory.write_byte(0x10005, 0xAB).unwrap();

        let mut cpu = Cpu::default();
        cpu.registers.cs = 0x1000;
        cpu.registers.ip = 0x0005;

        cpu.fetch_byte(&memory).unwrap();

        assert_eq!(cpu.registers.ip, 0x0006);
    }
}
