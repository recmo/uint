use crate::Uint;

// TODO: Special functions
// * Factorial
// * Extended GCD and LCM
// * https://en.wikipedia.org/wiki/Euler%27s_totient_function
// * https://en.wikipedia.org/wiki/Carmichael_function
// * https://en.wikipedia.org/wiki/Jordan%27s_totient_function
// * Feature parity with GMP:
//   * https://gmplib.org/manual/Integer-Functions.html#Integer-Functions

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Returns `true` if and only if `self == 2^k` for some `k`.
    #[must_use]
    pub fn is_power_of_two(self) -> bool {
        self.count_ones() == 1
    }

    /// Returns the smallest power of two greater than or equal to self.
    ///
    /// # Panics
    ///
    /// Panics if the value overlfows.
    #[must_use]
    pub fn next_power_of_two(self) -> Self {
        self.checked_next_power_of_two().unwrap()
    }

    /// Returns the smallest power of two greater than or equal to `self`. If
    /// the next power of two is greater than the type’s maximum value,
    /// [`None`] is returned, otherwise the power of two is wrapped in
    /// [`Some`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruint::{Uint, uint, aliases::U64};
    /// # uint!{
    /// assert_eq!(0_U64.checked_next_power_of_two(), Some(1_U64));
    /// assert_eq!(1_U64.checked_next_power_of_two(), Some(1_U64));
    /// assert_eq!(2_U64.checked_next_power_of_two(), Some(2_U64));
    /// assert_eq!(3_U64.checked_next_power_of_two(), Some(4_U64));
    /// assert_eq!(U64::MAX.checked_next_power_of_two(), None);
    /// # }
    /// ```
    #[must_use]
    pub fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_power_of_two() {
            return Some(self);
        }
        let exp = self.bit_len();
        if exp >= BITS {
            return None;
        }
        Some(Self::from(1) << exp)
    }
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Calculates the smallest value greater than or equal to `self` that is a
    /// multiple of `rhs`. Returns [`None`] is `rhs` is zero or the
    /// operation would result in overflow.
    #[must_use]
    pub fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        todo!()
    }

    #[must_use]
    pub fn next_multiple_of(self, rhs: Self) -> Self {
        self.checked_next_multiple_of(rhs).unwrap();
        todo!()
    }
}
