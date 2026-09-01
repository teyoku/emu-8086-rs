use crate::{
    errors::{CpuError, MemoryError},
    instructions::{Instruction, Operand},
    memory::Memory,
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

    pub fn fetch_word(&mut self, memory: &Memory) -> Result<u16, MemoryError> {
        let low = self.fetch_byte(memory)?;
        let high = self.fetch_byte(memory)?;
        Ok(u16::from_le_bytes([low, high]))
    }

    pub fn execute(
        &mut self,
        instruction: Instruction,
        memory: &mut Memory,
    ) -> Result<bool, CpuError> {
        match instruction {
            Instruction::Hlt => Ok(false),
            Instruction::Mov { dest, src } => {
                match (dest, src) {
                    // sets 8bit value to 8bit reg
                    (Operand::Reg8(reg), Operand::Immediate8(value)) => {
                        self.registers.set_8bit(&reg, value);
                    }
                    // sets 16bit value to 16bit reg
                    (Operand::Reg16(reg), Operand::Immediate16(value)) => {
                        self.registers.set_16bit(&reg, value);
                    }
                    _ => return Err(CpuError::InvalidOperands),
                }

                Ok(true)
            }
            Instruction::Add { dest, src } => {
                match (dest, src) {
                    // sets 8bit value to 8bit reg
                    (Operand::Reg8(reg), Operand::Immediate8(value)) => {
                        let reg_value = self.registers.get_8bit(&reg);
                        let result = reg_value.wrapping_add(value);
                        self.registers.set_8bit(&reg, result);
                        self.registers.update_flags_8bit(result);
                    }
                    _ => return Err(CpuError::InvalidOperands),
                }

                Ok(true)
            }
        }
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
