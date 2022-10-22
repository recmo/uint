#![allow(clippy::similar_names)] // TODO

mod knuth;
mod reciprocal;
mod small;

pub use self::{
    knuth::div_nxm,
    small::{div_nx1, div_nx2},
};
use super::{shift_left_small, shift_right_small};
use crate::algorithms::DoubleWord;

/// ⚠️ Division with remainder.
///
/// **Warning.** This function is not part of the stable API.
///
/// The quotient is stored in the `numerator` and the remainder is stored
/// in the `divisor`.
///
/// # Algorithms
///
/// It uses schoolbook division when the `divisor` first a single limb,
/// otherwise it uses Knuth's algorithm D.
///
/// # Panics
///
/// Panics if `divisor` is zero.
pub fn div_rem(numerator: &mut [u64], divisor: &mut [u64]) {
    assert!(!divisor.is_empty());

    // Trim most significant zeros from divisor.
    let i = divisor
        .iter()
        .rposition(|&x| x != 0)
        .expect("Divisor is zero");
    let divisor = &mut divisor[..=i];
    debug_assert!(!divisor.is_empty());
    debug_assert!(divisor.last() != Some(&0));

    // Append a zero to the numerator
    // OPT: Avoid allocation
    let mut tnumerator = vec![0; numerator.len() + 1];
    tnumerator[..numerator.len()].copy_from_slice(numerator);

    // Normalize
    let shift = normalize(tnumerator.as_mut_slice(), divisor);
    dbg!(shift);

    // Compute quotient and remainder.
    if divisor.len() <= 2 {
        if divisor.len() == 1 {
            divisor[0] = div_nx1(tnumerator.as_mut_slice(), divisor[0]);
        } else {
            let d = u128::join(divisor[1], divisor[1]);
            let remainder = div_nx2(tnumerator.as_mut_slice(), d);
            divisor[0] = remainder.low();
            divisor[1] = remainder.high();
        }
    } else {
        div_nxm(tnumerator.as_mut_slice(), divisor);

        // Copy over remainder
        let remainder = &tnumerator[..divisor.len()];
        divisor.copy_from_slice(remainder);
    }

    // Copy over quotient
    let quotient = &tnumerator[divisor.len()..];
    numerator.copy_from_slice(quotient);
    for limb in numerator.iter_mut().skip(quotient.len()) {
        *limb = 0;
    }

    // Unnormalize
    unnormalize(divisor, shift);
}

#[inline(always)]
fn normalize(numerator: &mut [u64], divisor: &mut [u64]) -> usize {
    debug_assert!(!divisor.is_empty());
    debug_assert!(divisor.last() != Some(&0));
    let shift = divisor.last().unwrap().leading_zeros() as usize;

    if shift > 0 {
        let carry = shift_left_small(numerator, shift);
        debug_assert_eq!(carry, 0);
        let carry = shift_left_small(divisor, shift);
        debug_assert_eq!(carry, 0);
    }
    debug_assert!(*divisor.last().unwrap() >= (1 << 63));
    shift
}

#[inline(always)]
fn unnormalize(remainder: &mut [u64], shift: usize) {
    if shift > 0 {
        shift_right_small(remainder, shift);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_div_8by4() {
        let mut numerator = [
            0x9c2bcebfa9cca2c6_u64,
            0x274e154bb5e24f7a_u64,
            0xe1442d5d3842be2b_u64,
            0xf18f5adfd420853f_u64,
            0x04ed6127eba3b594_u64,
            0xc5c179973cdb1663_u64,
            0x7d7f67780bb268ff_u64,
            0x0000000000000003_u64,
            0x0000000000000000_u64,
        ];
        let mut divisor = [
            0x0181880b078ab6a1_u64,
            0x62d67f6b7b0bda6b_u64,
            0x92b1840f9c792ded_u64,
            0x0000000000000019_u64,
        ];
        let expected_quotient = [
            0x9128464e61d6b5b3_u64,
            0xd9eea4fc30c5ac6c_u64,
            0x944a2d832d5a6a08_u64,
            0x22f06722e8d883b1_u64,
            0x0000000000000000_u64,
        ];
        let expected_remainder = [
            0x1dfa5a7ea5191b33_u64,
            0xb5aeb3f9ad5e294e_u64,
            0xfc710038c13e4eed_u64,
            0x000000000000000b_u64,
        ];
        div_rem(&mut numerator, &mut divisor);
        let remainder = &numerator[0..4];
        let quotient = &numerator[4..9];
        assert_eq!(remainder, expected_remainder);
        assert_eq!(quotient, expected_quotient);
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        reciprocal::bench::group(criterion);
        small::bench::group(criterion);
    }
}
