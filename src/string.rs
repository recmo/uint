use crate::{base_convert::BaseConvertError, utils::rem_up, Uint};
use core::{fmt, str::FromStr};

// FEATURE: Respect width parameter in formatters.

// TODO: Do we want to write `0` for `BITS == 0`.

#[cfg(feature = "alloc")]
impl<const BITS: usize, const LIMBS: usize> fmt::Display for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Base convert 19 digits at a time
        const BASE: u64 = 10_000_000_000_000_000_000_u64;
        let mut spigot = self.to_base_be(BASE);
        write!(f, "{}", spigot.next().unwrap_or(0))?;
        for digits in spigot {
            write!(f, "{digits:019}")?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Debug for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#x}_U{BITS}")
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::LowerHex for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<false>(f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::UpperHex for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<true>(f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Binary for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        if LIMBS == 0 || *self == Self::ZERO {
            return f.write_str("0");
        }

        for (i, &limb) in self.limbs.iter().rev().enumerate() {
            let width = if i == 0 { rem_up(Self::BITS, 64) } else { 64 };
            write!(f, "{limb:0width$b}")?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<const BITS: usize, const LIMBS: usize> fmt::Octal for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Base convert 21 digits at a time
        const BASE: u64 = 0x8000_0000_0000_0000_u64;
        let mut spigot = self.to_base_be(BASE);
        write!(f, "{:o}", spigot.next().unwrap_or(0))?;
        for digits in spigot {
            write!(f, "{digits:021o}")?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    fn fmt_hex<const UPPER: bool>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        if LIMBS == 0 || *self == Self::ZERO {
            return f.write_str("0");
        }

        for (i, &limb) in self.limbs.iter().rev().enumerate() {
            let width = if i == 0 {
                2 * rem_up(Self::BITS, 8)
            } else {
                16
            };
            if UPPER {
                write!(f, "{limb:0width$X}")?;
            } else {
                write!(f, "{limb:0width$x}")?;
            }
        }
        Ok(())
    }
}

/// Error for [`from_str_radix`](Uint::from_str_radix).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Invalid digit in string.
    InvalidDigit(char),

    /// Invalid radix, up to base 64 is supported.
    InvalidRadix(u64),

    /// Error from [`Uint::from_base_be`].
    BaseConvertError(BaseConvertError),
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BaseConvertError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<BaseConvertError> for ParseError {
    #[inline]
    fn from(value: BaseConvertError) -> Self {
        Self::BaseConvertError(value)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BaseConvertError(e) => fmt::Display::fmt(e, f),
            Self::InvalidDigit(c) => write!(f, "Invalid digit: {c}"),
            Self::InvalidRadix(r) => write!(f, "Invalid radix {r}, up to 64 is supported"),
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Parse a string into a [`Uint`].
    ///
    /// For bases 2 to 36, the case-agnostic alphabet 0—1, a—b is used and `_`
    /// are ignored. For bases 37 to 64, the case-sensitive alphabet a—z, A—Z,
    /// 0—9, {+-}, {/,_} is used. That is, for base 64 it is compatible with
    /// all the common base64 variants.
    ///
    /// # Errors
    ///
    /// * [`ParseError::InvalidDigit`] if the string contains a non-digit.
    /// * [`ParseError::InvalidRadix`] if the radix is larger than 64.
    /// * [`ParseError::BaseConvertError`] if [`Uint::from_base_be`] fails.
    // FEATURE: Support proper unicode. Ignore zero-width spaces, joiners, etc.
    // Recognize digits from other alphabets.
    pub fn from_str_radix(src: &str, radix: u64) -> Result<Self, ParseError> {
        if radix > 64 {
            return Err(ParseError::InvalidRadix(radix));
        }
        let mut err = None;
        let digits = src.chars().filter_map(|c| {
            if err.is_some() {
                return None;
            }
            let digit = if radix <= 36 {
                // Case insensitive 0—9, a—z.
                match c {
                    '0'..='9' => u64::from(c) - u64::from('0'),
                    'a'..='z' => u64::from(c) - u64::from('a') + 10,
                    'A'..='Z' => u64::from(c) - u64::from('A') + 10,
                    '_' => return None, // Ignored character.
                    _ => {
                        err = Some(ParseError::InvalidDigit(c));
                        return None;
                    }
                }
            } else {
                // The Base-64 alphabets
                match c {
                    'A'..='Z' => u64::from(c) - u64::from('A'),
                    'a'..='f' => u64::from(c) - u64::from('a') + 26,
                    '0'..='9' => u64::from(c) - u64::from('0') + 52,
                    '+' | '-' => 62,
                    '/' | ',' | '_' => 63,
                    '=' | '\r' | '\n' => return None, // Ignored characters.
                    _ => {
                        err = Some(ParseError::InvalidDigit(c));
                        return None;
                    }
                }
            };
            Some(digit)
        });
        let value = Self::from_base_be(radix, digits)?;
        err.map_or(Ok(value), Err)
    }
}

impl<const BITS: usize, const LIMBS: usize> FromStr for Uint<BITS, LIMBS> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        if src.is_char_boundary(2) {
            let (prefix, rest) = src.split_at(2);
            match prefix {
                "0x" | "0X" => return Self::from_str_radix(rest, 16),
                "0o" | "0O" => return Self::from_str_radix(rest, 8),
                "0b" | "0B" => return Self::from_str_radix(rest, 2),
                _ => {}
            }
        }
        Self::from_str_radix(src, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
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
            format!("{N:x}"),
            "c85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4"
        );
        assert_eq!(
            format!("{N:b}"),
            "1100100001011110111101111101011110010110100100011111111001111001010101110011101100011010011100000110010011000001100111000001101010011000000110011110101111011011110100011111101010101010101100011010100011101100100100100011010001000100001110001010101011110100"
        );
        assert_eq!(
            format!("{N:o}"),
            "14413675753626443771712563543234062301470152300636573364375252543243544443210416125364"
        );
    }

    #[test]
    fn test_hex() {
        proptest!(|(value: u64)| {
            let n: Uint<64, 1> = Uint::from(value);
            assert_eq!(format!("{n:x}"), format!("{value:016x}"));
            assert_eq!(format!("{n:#x}"), format!("{value:#018x}"));
            assert_eq!(format!("{n:X}"), format!("{value:016X}"));
            assert_eq!(format!("{n:#X}"), format!("{value:#018X}"));
            assert_eq!(format!("{n:b}"), format!("{value:064b}"));
            assert_eq!(format!("{n:#b}"), format!("{value:#066b}"));
        });
    }
}
