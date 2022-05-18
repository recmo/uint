use crate::Uint;
use core::fmt::{Binary, Debug, Display, Error, Formatter, LowerHex, Octal, Result, UpperHex};

// TODO: Respect width parameter in formatters.
// TODO: Decimal, Octal

impl<const BITS: usize, const LIMBS: usize> Debug for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:#x}_U{}", self, BITS)
    }
}

impl<const BITS: usize, const LIMBS: usize> LowerHex for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for limb in self.limbs.iter().rev() {
            write!(f, "{:016x}", limb)?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> UpperHex for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for limb in self.limbs.iter().rev() {
            write!(f, "{:016X}", limb)?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> Binary for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        for limb in self.limbs.iter().rev() {
            write!(f, "{:064b}", limb)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn test_hex() {
        proptest!(|(value: u64)| {
            let n: Uint<64, 1> = Uint::from(value);
            assert_eq!(format!("{:x}", n), format!("{:016x}", value));
            assert_eq!(format!("{:#x}", n), format!("{:#018x}", value));
            assert_eq!(format!("{:X}", n), format!("{:016X}", value));
            assert_eq!(format!("{:#X}", n), format!("{:#018X}", value));
            assert_eq!(format!("{:b}", n), format!("{:064b}", value));
            assert_eq!(format!("{:#b}", n), format!("{:#066b}", value));
        });
    }
}
