//! Knuth division
#![allow(
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::diverging_sub_expression,
    unreachable_code,
    unused_variables
)] // TODO

use crate::algorithms::{add::adc_n, mul::submul_nx1};

use super::{reciprocal::reciprocal_2, small::div_3x2, DoubleWord};
use core::{intrinsics::unlikely, u64};

/// Knuth division
///
/// Turns numerator into remainder, returns quotient.
///
/// Implements Knuth's division algorithm.
/// See D. Knuth "The Art of Computer Programming". Sec. 4.3.1. Algorithm D.
/// See <https://github.com/chfast/intx/blob/master/lib/intx/div.cpp>
///
/// `divisor` must have non-zero first limbs. Consequently, the remainder is
/// length at most `divisor.len()`, and the quotient is at most
/// `numerator.len() - divisor.len()` limbs.
///
/// NOTE: numerator must have one additional zero at the end.
/// The result will be computed in-place in numerator.
/// The divisor will be normalized.
///
/// TODO <https://janmr.com/blog/2014/04/basic-multiple-precision-long-division/>
///
/// [gmp]: https://gmplib.org/manual/Basecase-Division
/// [intx]: https://github.com/chfast/intx/blob/8b5f4748a7386a9530769893dae26b3273e0ffe2/include/intx/intx.hpp#L1736
#[inline(never)]
pub fn div_nxm(numerator: &mut [u64], divisor: &[u64]) {
    debug_assert!(divisor.len() >= 2);
    debug_assert!(numerator.len() >= divisor.len());
    debug_assert!(*divisor.last().unwrap() >= (1 << 63));
    // debug_assert!(*numerator.last().unwrap() == 0);

    let n = divisor.len();
    let m = numerator.len() - n - 1;

    // Compute the divisor double limb and reciprocal
    let d = u128::join(divisor[n - 1], divisor[n - 2]);
    let v = reciprocal_2(d);

    // Compute the quotient one limb at a time.
    for j in (0..=m).rev() {
        // Fetch the first three limbs of the numerator.
        // OPT: Re-use
        let n21 = u128::join(numerator[j + n], numerator[j + n - 1]);
        let n0 = numerator[j + n - 2];

        // Division overflow check
        assert!(n21 < d);
        // TODO: Handle
        // if unlikely(n21 == d) {}

        // Calculate 3x2 approximate quotient word.
        // By using 3x2 limbs we get a quotient that is very likely correct
        // and at most one too large. In the process we also get the first
        // two remainder limbs.
        let (mut q, r) = div_3x2(n21, n0, d, v);

        // Subtract the quotient times the divisor from the remainder.
        // We already have the highest two limbs, so we can reduce the
        // computation. We still need to carry propagate into these limbs.
        let borrow = submul_nx1(&mut numerator[j..j + n - 2], &divisor[..n - 2], q);
        let (r, borrow) = r.overflowing_sub(u128::from(borrow));
        numerator[j + n - 2] = r.low();
        numerator[j + n - 1] = r.high();

        // If we have a carry then the quotient was one too large.
        // We correct by decrementing the quotient and adding one divisor back.
        if unlikely(borrow) {
            dbg!();
            q = q.wrapping_sub(1);
            let _ = adc_n(numerator, divisor, 0);
        }

        // Store remainder in the unused bits of numerator
        numerator[j + n] = q;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::{
        add::{cmp, sbb_n},
        mul,
    };
    use proptest::{
        collection, num, proptest,
        strategy::{Just, Strategy},
    };
    use std::cmp::Ordering;

    // Basic test without exceptional paths
    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_divrem_8by4() {
        let mut numerator = [
            0x3000000000000000,
            0xd4e15e75fd4e6516,
            0x593a70aa5daf127b,
            0xff0a216ae9c215f1,
            0xa78c7ad6fea10429,
            0x18276b093f5d1dac,
            0xfe2e0bccb9e6d8b3,
            0x1bebfb3bc05d9347,
        ];
        let mut divisor = [
            0x800000000000000,
            0x580c0c40583c55b5,
            0x6b16b3fb5bd85ed3,
            0xcc958c207ce3c96f,
        ];
        let expected_quotient = [
            0x9128464e61d6b5b3_u64,
            0xd9eea4fc30c5ac6c_u64,
            0x944a2d832d5a6a08_u64,
            0x22f06722e8d883b1_u64,
        ];
        let expected_remainder = [
            0x9800000000000000,
            0x70efd2d3f528c8d9,
            0x6dad759fcd6af14a,
            0x5fe38801c609f277,
        ];
        div_nxm(&mut numerator, &mut divisor);
        let remainder = &numerator[0..4];
        let quotient = &numerator[4..8];
        assert_eq!(remainder, expected_remainder);
        assert_eq!(quotient, expected_quotient);
    }

    // TODO: Test with unlikely q too large.

    // TODO: Test with n21 == d

    // Proptest without forced exceptional paths
    #[test]
    fn test_div() {
        let quotient = collection::vec(num::u64::ANY, 1..10);
        let divisor = collection::vec(num::u64::ANY, 2..10).prop_map(|mut vec| {
            *vec.last_mut().unwrap() |= 1 << 63;
            vec
        });
        let dr = divisor.prop_flat_map(|divisor| {
            let d = divisor.clone();
            let remainder =
                collection::vec(num::u64::ANY, divisor.len()).prop_map(move |mut vec| {
                    if cmp(&vec, &d) != Ordering::Less {
                        let carry = sbb_n(&mut vec, &d, 0);
                        assert_eq!(carry, 0);
                    }
                    vec
                });
            (Just(divisor), remainder)
        });
        proptest!(|(quotient in quotient, (divisor, remainder) in dr)| {
            let mut numerator: Vec<u64> = vec![0; divisor.len() + quotient.len()];
            numerator[..remainder.len()].copy_from_slice(&remainder);
            mul(quotient.as_slice(), divisor.as_slice(), &mut numerator);

            div_nxm(numerator.as_mut_slice(), divisor.as_slice());
            let (r, q) = numerator.split_at(divisor.len());
            assert_eq!(r, remainder);
            assert_eq!(q, quotient);
        });
    }
}
