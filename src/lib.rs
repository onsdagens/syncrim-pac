#![no_std]

use riscv_peripheral::clic::*;

riscv_peripheral::clic_codegen!(base 0x0000);
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    Interrupt0 = 0,
    Interrupt1 = 1,
    Interrupt2 = 2,
    Interrupt3 = 3,
    Interrupt4 = 4,
    Interrupt5 = 5,
    Interrupt6 = 6,
    Interrupt7 = 7,
    Interrupt8 = 8,
}

unsafe impl InterruptNumber for Interrupt {
    const MAX_INTERRUPT_NUMBER: u16 = 3;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        if number == 0 || number > Self::MAX_INTERRUPT_NUMBER {
            Err(number)
        }
        else  {
            // SAFETY: valid interrupt number
            Ok(unsafe { core::mem::transmute(number)})
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
    P0 = 0,
    P1 = 1, 
    P2 = 2, 
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
    P8 = 8,
    P9 = 9,
    //etc...
}

unsafe impl PriorityNumber for Priority {
    const MAX_PRIORITY_NUMBER: u8 = 9;

    #[inline]
    fn number(self) -> u8 {
        self as _
    }

    #[inline]
    fn from_number(number: u8) -> Result<Self, u8> {
        if number > Self::MAX_PRIORITY_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid priority number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}
