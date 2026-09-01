use crate::{
    cpu::Cpu,
    errors::CpuError,
    memory::Memory,
    registers::{Register8Bit, Register16Bit},
};

#[derive(Debug, PartialEq)]
pub enum Operand {
    Reg8(Register8Bit),
    Reg16(Register16Bit),
    Immediate8(u8),
    Immediate16(u16),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { dest: Operand, src: Operand },
    Add { dest: Operand, src: Operand },
    Hlt,
}

pub fn decode(opcode: u8, cpu: &mut Cpu, memory: &Memory) -> Result<Instruction, CpuError> {
    match opcode {
        // HALT
        0xF4 => Ok(Instruction::Hlt),
        // MOV AL, imm8
        0xB0 => {
            let byte = cpu
                .fetch_byte(memory)
                .map_err(|err| CpuError::MemoryError(err))?;

            Ok(Instruction::Mov {
                dest: Operand::Reg8(Register8Bit::Al),
                src: Operand::Immediate8(byte),
            })
        }
        // MOV AX, imm16
        0xB8 => {
            let word = cpu
                .fetch_word(memory)
                .map_err(|err| CpuError::MemoryError(err))?;

            Ok(Instruction::Mov {
                dest: Operand::Reg16(Register16Bit::Ax),
                src: Operand::Immediate16(word),
            })
        }
        // ADD AL, imm8
        0x04 => {
            let byte = cpu
                .fetch_byte(memory)
                .map_err(|err| CpuError::MemoryError(err))?;

            Ok(Instruction::Add {
                dest: Operand::Reg8(Register8Bit::Al),
                src: Operand::Immediate8(byte),
            })
        }
        _ => Err(CpuError::UnknownOpcode(opcode)),
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::Operand::Immediate8;

    use super::*;

    #[test]
    fn test_mov_instruction() {
        let mut memory = Memory::new();
        memory.write_byte(0x0000, 0xB0).unwrap();
        memory.write_byte(0x0001, 0x42).unwrap();

        let mut cpu = Cpu::default();

        let opcode = cpu.fetch_byte(&memory).unwrap();
        assert_eq!(opcode, 0xB0);
        assert_eq!(
            decode(opcode, &mut cpu, &memory).unwrap(),
            Instruction::Mov {
                dest: Operand::Reg8(Register8Bit::Al),
                src: Immediate8(0x42)
            }
        );
    }
}
