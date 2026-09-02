#[derive(Debug, PartialEq)]
pub enum MemoryError {
    OutOfBounds,
}
impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::OutOfBounds => write!(f, "Memory out of bounds"),
        }
    }
}

impl std::error::Error for MemoryError {}

#[derive(Debug, PartialEq)]
pub enum CpuError {
    MemoryError(MemoryError),
    UnknownOpcode(u8),
    InvalidOperands,
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::MemoryError(memory_error) => memory_error.fmt(f),
            CpuError::UnknownOpcode(opcode) => write!(f, "Unknown opcode: 0x{:02X}", opcode),
            CpuError::InvalidOperands => write!(f, "Invalid operands"),
        }
    }
}

impl std::error::Error for CpuError {}

impl From<MemoryError> for CpuError {
    fn from(value: MemoryError) -> Self {
        CpuError::MemoryError(value)
    }
}
