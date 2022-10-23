//! Knuth division

use super::{reciprocal::reciprocal_2, small::div_3x2, DoubleWord};
use crate::algorithms::{add::adc_n, mul::submul_nx1};
use core::{intrinsics::unlikely, u64};

/// In-place Knuth long division
#[allow(clippy::many_single_char_names)]
pub fn div_nxm(numerator: &mut [u64], divisor: &[u64]) {
    debug_assert!(divisor.len() >= 2);
    debug_assert!(numerator.len() >= divisor.len());
    debug_assert!(*divisor.last().unwrap() >= (1 << 63));

    let n = divisor.len();
    let m = numerator.len() - n - 1;

    // Compute the divisor double limb and reciprocal
    let d = u128::join(divisor[n - 1], divisor[n - 2]);
    let v = reciprocal_2(d);

    // Compute the quotient one limb at a time.
    for j in (0..=m).rev() {
        // Fetch the first three limbs of the numerator.
        let n21 = u128::join(numerator[j + n], numerator[j + n - 1]);
        let n0 = numerator[j + n - 2];
        debug_assert!(n21 <= d);

        // Overflow case
        if n21 == d {
            let q = u64::MAX;
            let _carry = submul_nx1(&mut numerator[j..j + n], divisor, q);
            numerator[j + n] = q;
            continue;
        }

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
            q = q.wrapping_sub(1);
            let carry = adc_n(&mut numerator[j..j + n], &divisor[..n], 0);
            // Expect carry because we flip sign back to positive.
            debug_assert_eq!(carry, 1);
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
        let divisor = [
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
        div_nxm(&mut numerator, &divisor);
        let (remainder, quotient) = numerator.split_at(divisor.len());
        assert_eq!(remainder, expected_remainder);
        assert_eq!(quotient, expected_quotient);
    }

    // Test case that forces the `unlikely(borrow)` branch.
    #[test]
    fn test_div_rollback() {
        let mut numerator = [
            0x1656178c14142000,
            0x821415dfe9e81612,
            0x1616561616161616,
            0x96000016820016,
        ];
        let divisor = [0x1415dfe9e8161414, 0x1656161616161682, 0x9600001682001616];
        let expected_quotient = [0xffffffffffffff];
        let expected_remainder = [0x166bf775fc2a3414, 0x1656161616161680, 0x9600001682001616];
        div_nxm(&mut numerator, &divisor);
        let (remainder, quotient) = numerator.split_at(divisor.len());
        assert_eq!(remainder, expected_remainder);
        assert_eq!(quotient, expected_quotient);
    }

    // Test case that forces the `unlikely(borrow)` branch.
    #[test]
    fn test_div_rollback_2() {
        let mut numerator = [
            0x100100000,
            0x81000,
            0x1000000000000000,
            0x0,
            0x0,
            0xfffff00000000000,
            0xffffffffffffffff,
            0xdfffffffffffff,
        ];
        let divisor = [
            0xfffffffffff00000,
            0xffffffffffffffff,
            0xfffffffffffff3ff,
            0xffffffffffffffff,
            0xdfffffffffffffff,
        ];
        let expected_quotient = [0xffffedb6db6db6e9, 0xffffffffffffffff, 0xffffffffffffff];
        let expected_remainder = [
            0xdb6db6dc6ea00000,
            0x80ffe,
            0xf2492492492ec00,
            0x1000,
            0x2000000000000000,
        ];
        div_nxm(&mut numerator, &divisor);
        let (remainder, quotient) = numerator.split_at(divisor.len());
        assert_eq!(quotient, expected_quotient);
        assert_eq!(remainder, expected_remainder);
    }

    #[test]
    fn test_div_overflow() {
        let mut numerator = [0xb200000000000002, 0x1, 0x0, 0xfdffffff00000000];
        let divisor = [0x10002, 0x0, 0xfdffffff00000000];
        let expected_quotient = [0xffffffffffffffff];
        let expected_remainder = [0xb200000000010004, 0xfffffffffffeffff, 0xfdfffffeffffffff];
        div_nxm(&mut numerator, &divisor);
        let (remainder, quotient) = numerator.split_at(divisor.len());
        assert_eq!(quotient, expected_quotient);
        assert_eq!(remainder, expected_remainder);
    }

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
