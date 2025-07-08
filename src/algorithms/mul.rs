#![allow(clippy::module_name_repetitions)]

use crate::algorithms::{ops::sbb, DoubleWord};

/// ⚠️ Computes `result += a * b` and checks for overflow.
///
/// **Warning.** This function is not part of the stable API.
///
/// Arrays are in little-endian order. All arrays can be arbitrary sized.
///
/// # Algorithm
///
/// Trims zeros from inputs, then uses the schoolbook multiplication algorithm.
/// It takes the shortest input as the outer loop.
///
/// # Examples
///
/// ```
/// # use ruint::algorithms::addmul;
/// let mut result = [0];
/// let overflow = addmul(&mut result, &[3], &[4]);
/// assert_eq!(overflow, false);
/// assert_eq!(result, [12]);
/// ```
#[inline(always)]
pub fn addmul(mut lhs: &mut [u64], mut a: &[u64], mut b: &[u64]) -> bool {
    // Trim zeros from `a`
    while let [0, rest @ ..] = a {
        a = rest;
        if let [_, rest @ ..] = lhs {
            lhs = rest;
        }
    }
    a = super::trim_end_zeros(a);
    if a.is_empty() {
        return false;
    }

    // Trim zeros from `b`
    while let [0, rest @ ..] = b {
        b = rest;
        if let [_, rest @ ..] = lhs {
            lhs = rest;
        }
    }
    b = super::trim_end_zeros(b);
    if b.is_empty() {
        return false;
    }

    if lhs.is_empty() {
        return true;
    }

    let (a, b) = if b.len() > a.len() { (b, a) } else { (a, b) };

    // Iterate over limbs of `b` and add partial products to `lhs`.
    let mut overflow = false;
    for &b in b {
        if lhs.len() >= a.len() {
            let (target, rest) = lhs.split_at_mut(a.len());
            let carry = addmul_nx1(target, a, b);
            let carry = add_nx1(rest, carry);
            overflow |= carry != 0;
        } else {
            overflow = true;
            if lhs.is_empty() {
                break;
            }
            addmul_nx1(lhs, &a[..lhs.len()], b);
        }
        lhs = &mut lhs[1..];
    }
    overflow
}

const ADDMUL_N_SMALL_LIMIT: usize = 8;

/// ⚠️ Computes wrapping `result += a * b`, with a fast-path for when all inputs
/// are the same length and small enough.
///
/// **Warning.** This function is not part of the stable API.
///
/// See [`addmul`] for more details.
#[inline(always)]
pub fn addmul_n(lhs: &mut [u64], a: &[u64], b: &[u64]) {
    let n = lhs.len();
    if n <= ADDMUL_N_SMALL_LIMIT && a.len() == n && b.len() == n {
        addmul_n_small(lhs, a, b);
    } else {
        let _ = addmul(lhs, a, b);
    }
}

#[inline(always)]
fn addmul_n_small(lhs: &mut [u64], a: &[u64], b: &[u64]) {
    let n = lhs.len();
    assume!(n <= ADDMUL_N_SMALL_LIMIT);
    assume!(a.len() == n);
    assume!(b.len() == n);

    for j in 0..n {
        let mut carry = 0;
        for i in 0..(n - j) {
            (lhs[j + i], carry) = u128::muladd2(a[i], b[j], carry, lhs[j + i]).split();
        }
    }
}

/// Computes `lhs += a` and returns the carry.
#[inline(always)]
pub fn add_nx1(lhs: &mut [u64], mut a: u64) -> u64 {
    if a == 0 {
        return 0;
    }
    for lhs in lhs {
        (*lhs, a) = u128::add(*lhs, a).split();
        if a == 0 {
            return 0;
        }
    }
    a
}

/// Computes `lhs *= a` and returns the carry.
#[inline(always)]
pub fn mul_nx1(lhs: &mut [u64], a: u64) -> u64 {
    let mut carry = 0;
    for lhs in lhs {
        (*lhs, carry) = u128::muladd(*lhs, a, carry).split();
    }
    carry
}

/// Computes `lhs += a * b` and returns the carry.
///
/// Requires `lhs.len() == a.len()`.
///
/// $$
/// \begin{aligned}
/// \mathsf{lhs'} &= \mod{\mathsf{lhs} + \mathsf{a} ⋅ \mathsf{b}}_{2^{64⋅N}}
/// \\\\ \mathsf{carry} &= \floor{\frac{\mathsf{lhs} + \mathsf{a} ⋅ \mathsf{b}
/// }{2^{64⋅N}}} \end{aligned}
/// $$
#[inline(always)]
pub fn addmul_nx1(lhs: &mut [u64], a: &[u64], b: u64) -> u64 {
    assume!(lhs.len() == a.len());
    let mut carry = 0;
    for i in 0..a.len() {
        (lhs[i], carry) = u128::muladd2(a[i], b, carry, lhs[i]).split();
    }
    carry
}

/// Computes `lhs -= a * b` and returns the borrow.
///
/// Requires `lhs.len() == a.len()`.
///
/// $$
/// \begin{aligned}
/// \mathsf{lhs'} &= \mod{\mathsf{lhs} - \mathsf{a} ⋅ \mathsf{b}}_{2^{64⋅N}}
/// \\\\ \mathsf{borrow} &= \floor{\frac{\mathsf{a} ⋅ \mathsf{b} -
/// \mathsf{lhs}}{2^{64⋅N}}} \end{aligned}
/// $$
// OPT: `carry` and `borrow` can probably be merged into a single var.
#[inline(always)]
pub fn submul_nx1(lhs: &mut [u64], a: &[u64], b: u64) -> u64 {
    assume!(lhs.len() == a.len());
    let mut carry = 0;
    let mut borrow = 0;
    for i in 0..a.len() {
        // Compute product limbs
        let limb;
        (limb, carry) = u128::muladd(a[i], b, carry).split();

        // Subtract
        (lhs[i], borrow) = sbb(lhs[i], limb, borrow);
    }
    borrow + carry
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{collection, num::u64, proptest};

    #[allow(clippy::cast_possible_truncation)] // Intentional truncation.
    fn addmul_ref(result: &mut [u64], a: &[u64], b: &[u64]) -> bool {
        let mut overflow = 0;
        for (i, a) in a.iter().copied().enumerate() {
            let mut result = result.iter_mut().skip(i);
            let mut b = b.iter().copied();
            let mut carry = 0_u128;
            loop {
                match (result.next(), b.next()) {
                    // Partial product.
                    (Some(result), Some(b)) => {
                        carry += u128::from(*result) + u128::from(a) * u128::from(b);
                        *result = carry as u64;
                        carry >>= 64;
                    }
                    // Carry propagation.
                    (Some(result), None) => {
                        carry += u128::from(*result);
                        *result = carry as u64;
                        carry >>= 64;
                    }
                    // Excess product.
                    (None, Some(b)) => {
                        carry += u128::from(a) * u128::from(b);
                        overflow |= carry as u64;
                        carry >>= 64;
                    }
                    // Fin.
                    (None, None) => {
                        break;
                    }
                }
            }
            overflow |= carry as u64;
        }
        overflow != 0
    }

    #[test]
    fn test_addmul() {
        let any_vec = collection::vec(u64::ANY, 0..10);
        proptest!(|(mut lhs in &any_vec, a in &any_vec, b in &any_vec)| {
            // Reference
            let mut ref_lhs = lhs.clone();
            let ref_overflow = addmul_ref(&mut ref_lhs, &a, &b);

            // Test
            let overflow = addmul(&mut lhs, &a, &b);
            assert_eq!(lhs, ref_lhs);
            assert_eq!(overflow, ref_overflow);
        });
    }

    fn test_vals(lhs: &[u64], rhs: &[u64], expected: &[u64], expected_overflow: bool) {
        let mut result = vec![0; expected.len()];
        let overflow = addmul(&mut result, lhs, rhs);
        assert_eq!(overflow, expected_overflow);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty() {
        test_vals(&[], &[], &[], false);
        test_vals(&[], &[1], &[], false);
        test_vals(&[1], &[], &[], false);
        test_vals(&[1], &[1], &[], true);
        test_vals(&[], &[], &[0], false);
        test_vals(&[], &[1], &[0], false);
        test_vals(&[1], &[], &[0], false);
        test_vals(&[1], &[1], &[1], false);
    }

    #[test]
    fn test_submul_nx1() {
        let mut lhs = [
            15520854688669198950,
            13760048731709406392,
            14363314282014368551,
            13263184899940581802,
        ];
        let a = [
            7955980792890017645,
            6297379555503105007,
            2473663400150304794,
            18362433840513668572,
        ];
        let b = 17275533833223164845;
        let borrow = submul_nx1(&mut lhs, &a, b);
        assert_eq!(lhs, [
            2427453526388035261,
            7389014268281543265,
            6670181329660292018,
            8411211985208067428
        ]);
        assert_eq!(borrow, 17196576577663999042);
    }
}
