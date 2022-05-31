// FEATURE: Integer and module square root and higher roots.

//          1         A
// a[i+1] = - * ( --------- + (n-1)*a[i] )
//          n     a[i]^(n-1)

use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Computes the floor of the nth root of the number.
    ///
    /// $$
    /// \floor{\sqrt[\mathtt index]{\mathtt{self}}}
    /// $$
    ///
    /// # Panics
    ///
    /// Panics if `index` is zero.
    #[must_use]
    pub fn root(self, index: usize) -> Self {
        assert!(index > 0);

        // Create a first guess.
        // Root should be less than the value, so approx_pow2 should always succeed.
        #[allow(clippy::cast_precision_loss)] // Approximation is good enough.
        #[allow(clippy::cast_sign_loss)] // Result should be positive.
        let mut result = Self::approx_pow2(self.approx_log2() / index as f64).unwrap();

        // Iterate using Newton's method
        // See <https://en.wikipedia.org/wiki/Integer_square_root#Algorithm_using_Newton's_method>
        // See <https://gmplib.org/manual/Nth-Root-Algorithm>
        let mut first = true;
        loop {
            // OPT: This could benefit from single-limb multiplication and division. The
            // division can be turned into bit-shifts when the index is a power
            // of two.
            let iter = (self / result.pow(Self::from(index - 1)) + Self::from(index - 1) * result)
                / Self::from(index);
            if !first && iter >= result {
                break result;
            }
            first = false;
            result = iter;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_root() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U, index in 1_usize..=5)| {
                let root = value.root(index);
                let lower = root.pow(U::from(index));
                let upper = (root + U::from(1)).pow(U::from(index));
                assert!(value >= lower);
                assert!(value < upper);
            });
        });
    }
}
