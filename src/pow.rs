use crate::{impl_bin_op, Uint};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    pub const fn checked_log(self, base: u64) -> Option<u32> {
        todo!()
    }

    pub const fn checked_log10(self) -> Option<u32> {
        todo!()
    }

    pub const fn checked_log2(self) -> Option<u32> {
        todo!()
    }

    pub const fn checked_next_multiple_of(self, rhs: u64) -> Option<u64> {
        todo!()
    }

    pub const fn checked_next_power_of_two(self) -> Option<u64> {
        todo!()
    }

    pub const fn checked_pow(self, exp: u32) -> Option<u64> {
        todo!()
    }
    
    /// Returns `true` if and only if `self == 2^k` for some `k`.
    pub const fn is_power_of_two(self) -> bool {
        self.count_ones() == 1
    }

    pub const fn log(self, base: u64) -> u32 {
        todo!()
    }

    pub const fn log10(self) -> u32 {
        todo!()
    }

    pub const fn log2(self) -> u32 {
        todo!()
    }

    pub const fn next_multiple_of(self, rhs: u64) -> u64 {
        todo!()
    }

    pub const fn next_power_of_two(self) -> u64 {
        todo!()
    }

    pub const fn overflowing_pow(self, exp: u32) -> (u64, bool) {
        todo!()
    }

    pub const fn pow(self, exp: u32) -> u64 {
        todo!()
    }

    pub const fn saturating_pow(self, exp: u32) -> u64 {
        todo!()
    }

    pub const fn wrapping_pow(self, exp: u32) -> u64 {
        todo!()
    }
}
