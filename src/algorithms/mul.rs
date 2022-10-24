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
/// Uses the schoolbook multiplication algorithm.
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
#[allow(clippy::cast_possible_truncation)] // Intentional truncation.
pub fn addmul(result: &mut [u64], a: &[u64], b: &[u64]) -> bool {
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
    debug_assert_eq!(lhs.len(), a.len());
    let mut carry = 0;
    let mut borrow = 0;
    for (lhs, a) in lhs.iter_mut().zip(a.iter().copied()) {
        // Compute product limbs
        let limb = {
            let product = u128::muladd(a, b, carry);
            carry = product.high();
            product.low()
        };

        // Subtract
        let (new, b) = sbb(*lhs, limb, borrow);
        *lhs = new;
        borrow = b;
    }
    borrow + carry
}

#[cfg(test)]
mod tests {
    use super::*;

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
