use std;
use std::ops::{Shr, Shl};
use std::u64::MAX;

// Emulate a 128 bit uint using two 64 bit uints.

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct emu128 {
    pub hi: u64,
    pub lo: u64,
}

// TODO: Proper testing to make sure these functions work correctly.

impl emu128 {
    pub fn saturating_add(self, other: emu128) -> emu128 {
        let (lo, ov) = self.lo.overflowing_add(other.lo);
        let res = self.hi.checked_add(other.hi).and_then(|x| x.checked_add(if ov { 1 } else { 0 }));
        match res {
            Some(hi) => emu128 { hi: hi, lo: lo },
            None => emu128 { hi: MAX, lo: MAX },
        }
    }

    pub fn saturating_sub(self, other: emu128) -> emu128 {
        let (lo, ov) = self.lo.overflowing_sub(other.lo);
        let res = self.hi.checked_sub(other.hi).and_then(|x| x.checked_sub(if ov { 1 } else { 0 }));
        match res {
            Some(hi) => emu128 { hi: hi, lo: lo },
            None => emu128 { hi: 0, lo: 0 },
        }
    }

    pub fn leading_zeros(self) -> u32 {
        if self.hi > 0 { self.hi.leading_zeros() } else { self.lo.leading_zeros() + 64 } 
    }

    pub fn trailing_zeros(self) -> u32 {
        if self.lo > 0 { self.lo.trailing_zeros() } else { self.hi.trailing_zeros() + 64 } 
    }
}

// TODO: Convert to macro to implement for other RHS types.
impl Shl<u8> for emu128 {
    type Output = Self;

    fn shl(self, rhs: u8) -> emu128 {
        if rhs < 64 {
            emu128 {
                hi: self.hi << rhs | self.lo >> (64-rhs),
                lo: self.lo << rhs
            }
        }
        else {
            emu128 {
                hi: self.lo << (rhs-64),
                lo: 0
            }
        }
    }
}

impl Shr<u8> for emu128 {
    type Output = Self;

    fn shr(self, rhs: u8) -> emu128 {
        if rhs < 64 {
            emu128 {
                hi: self.hi >> rhs,
                lo: self.lo >> rhs | self.hi << (64-rhs),
            }
        }
        else {
            emu128 {
                hi: 0,
                lo: self.hi >> (rhs-64)
            }
        }
    }
}
