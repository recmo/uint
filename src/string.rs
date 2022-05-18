use crate::Uint;
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex};

// TODO: Respect width parameter in formatters.

impl<const BITS: usize, const LIMBS: usize> Display for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Base convert 19 digits at a time
        const BASE: u64 = 10_000_000_000_000_000_000_u64;
        let mut spigot = self.to_base_be(BASE);
        write!(f, "{}", spigot.next().unwrap_or(0))?;
        for digits in spigot {
            write!(f, "{:019}", digits)?;
        }
        Ok(())
    }
}

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
        let mut limbs = self.as_limbs().iter().rev();
        if let Some(first) = limbs.next() {
            let width = 2 * if Self::BYTES % 8 == 0 {
                8
            } else {
                Self::BYTES % 8
            };
            write!(f, "{:0width$x}", first, width = width)?;
        }
        for limb in limbs {
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
        let mut limbs = self.as_limbs().iter().rev();
        if let Some(first) = limbs.next() {
            let width = 2 * if Self::BYTES % 8 == 0 {
                8
            } else {
                Self::BYTES % 8
            };
            write!(f, "{:0width$X}", first, width = width)?;
        }
        for limb in limbs {
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
        let mut limbs = self.as_limbs().iter().rev();
        if let Some(first) = limbs.next() {
            let width = if Self::BITS % 64 == 0 {
                64
            } else {
                Self::BITS % 64
            };
            write!(f, "{:0width$b}", first, width = width)?;
        }
        for limb in limbs {
            write!(f, "{:064b}", limb)?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> Octal for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Base convert 21 digits at a time
        const BASE: u64 = 0x8000_0000_0000_0000_u64;
        let mut spigot = self.to_base_be(BASE);
        write!(f, "{:o}", spigot.next().unwrap_or(0))?;
        for digits in spigot {
            write!(f, "{:021o}", digits)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[allow(clippy::unreadable_literal)]
    const N: Uint<256, 4> = Uint::from_limbs([
        0xa8ec92344438aaf4_u64,
        0x9819ebdbd1faaab1_u64,
        0x573b1a7064c19c1a_u64,
        0xc85ef7d79691fe79_u64,
    ]);

    #[test]
    fn test_num() {
        assert_eq!(
            N.to_string(),
            "90630363884335538722706632492458228784305343302099024356772372330524102404852"
        );
        assert_eq!(
            format!("{:x}", N),
            "c85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4"
        );
        assert_eq!(
            format!("{:b}", N),
            "1100100001011110111101111101011110010110100100011111111001111001010101110011101100011010011100000110010011000001100111000001101010011000000110011110101111011011110100011111101010101010101100011010100011101100100100100011010001000100001110001010101011110100"
        );
        assert_eq!(
            format!("{:o}", N),
            "14413675753626443771712563543234062301470152300636573364375252543243544443210416125364"
        );
    }

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
