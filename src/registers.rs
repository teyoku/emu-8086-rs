#[derive(Default)]
pub struct Registers {
    // general-purpose registers
    pub ax: u16, // primary accumulator
    pub bx: u16, // base, accumulator
    pub cx: u16, // counter, accumulator
    pub dx: u16, // accumulaotr, other functions

    // index registers
    pub si: u16, // Source Index
    pub di: u16, // Destination Index

    // pointer registers
    pub sp: u16, // Stack Pointer
    pub bp: u16, // Base Pointer

    // instruction pointer
    pub ip: u16,

    // segment registers
    pub cs: u16, // Code Segment
    pub ds: u16, // Data Segment
    pub ss: u16, // Stack Segment
    pub es: u16, // Extra Segment

    // flags
    pub flags: u16,
}

#[derive(Debug, PartialEq)]
pub enum Register8Bit {
    Ah,
    Al,
    Bh,
    Bl,
    Ch,
    Cl,
    Dh,
    Dl,
}

#[derive(Debug, PartialEq)]
pub enum Register16Bit {
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Sp,
    Bp,
}

impl Registers {
    pub fn get_8bit(&self, reg: &Register8Bit) -> u8 {
        match reg {
            Register8Bit::Ah => (self.ax >> 8) as u8,
            Register8Bit::Al => (self.ax & 0x00FF) as u8,
            Register8Bit::Bh => (self.bx >> 8) as u8,
            Register8Bit::Bl => (self.bx & 0x00FF) as u8,
            Register8Bit::Ch => (self.cx >> 8) as u8,
            Register8Bit::Cl => (self.cx & 0x00FF) as u8,
            Register8Bit::Dh => (self.dx >> 8) as u8,
            Register8Bit::Dl => (self.dx & 0x00FF) as u8,
        }
    }

    pub fn set_8bit(&mut self, reg: &Register8Bit, value: u8) {
        match reg {
            Register8Bit::Ah => self.ax = (self.ax & 0x00FF) | ((value as u16) << 8),
            Register8Bit::Al => self.ax = (self.ax & 0xFF00) | (value as u16),
            Register8Bit::Bh => self.bx = (self.bx & 0x00FF) | ((value as u16) << 8),
            Register8Bit::Bl => self.bx = (self.bx & 0xFF00) | (value as u16),
            Register8Bit::Ch => self.cx = (self.cx & 0x00FF) | ((value as u16) << 8),
            Register8Bit::Cl => self.cx = (self.cx & 0xFF00) | (value as u16),
            Register8Bit::Dh => self.dx = (self.dx & 0x00FF) | ((value as u16) << 8),
            Register8Bit::Dl => self.dx = (self.dx & 0xFF00) | (value as u16),
        }
    }

    pub fn get_16bit(&self, reg: &Register16Bit) -> u16 {
        match reg {
            Register16Bit::Ax => self.ax,
            Register16Bit::Bx => self.bx,
            Register16Bit::Cx => self.cx,
            Register16Bit::Dx => self.dx,
            Register16Bit::Si => self.si,
            Register16Bit::Di => self.di,
            Register16Bit::Sp => self.sp,
            Register16Bit::Bp => self.bp,
        }
    }

    pub fn set_16bit(&mut self, reg: &Register16Bit, value: u16) {
        match reg {
            Register16Bit::Ax => self.ax = value,
            Register16Bit::Bx => self.bx = value,
            Register16Bit::Cx => self.cx = value,
            Register16Bit::Dx => self.dx = value,
            Register16Bit::Si => self.si = value,
            Register16Bit::Di => self.di = value,
            Register16Bit::Sp => self.sp = value,
            Register16Bit::Bp => self.bp = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut registers = Registers::default();
        registers.set_16bit(&Register16Bit::Ax, 0x1234);

        assert_eq!(registers.get_8bit(&Register8Bit::Ah), 0x12);
        assert_eq!(registers.get_8bit(&Register8Bit::Al), 0x34);

        registers.set_8bit(&Register8Bit::Ah, 0xAB);
        registers.set_8bit(&Register8Bit::Al, 0xCD);

        assert_eq!(registers.get_16bit(&Register16Bit::Ax), 0xABCD);
    }
}
