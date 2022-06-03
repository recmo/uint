#![allow(clippy::module_name_repetitions)]
use core::iter::zip;

/// ⚠️ Computes `result += lhs * rhs` and checks for overflow.
///
/// **Warning.** This function is not part of the stable API.
///
/// Arrays are in little-endian order. All arrays can be arbitrary sized.
///
/// # Examples
///
/// ```
/// # use ruint::algorithms::mul;
/// let mut result = [0];
/// let overflow = mul(&[3], &[4], &mut result);
/// assert_eq!(overflow, false);
/// assert_eq!(result, [12]);
/// ```
pub fn mul(lhs: &[u64], rhs: &[u64], result: &mut [u64]) -> bool {
    mul_inline(lhs, rhs, result)
}

/// ⚠️ Same as [`mul`], but will always inline.
///
/// **Warning.** This function is not part of the stable API.
#[allow(clippy::inline_always)] // We want to decide at the call site.
#[inline(always)]
#[allow(clippy::cast_possible_truncation)] // Intentional truncation.
pub fn mul_inline(lhs: &[u64], rhs: &[u64], result: &mut [u64]) -> bool {
    let mut overflow = 0;
    for (i, lhs) in lhs.iter().copied().enumerate() {
        let mut result = result.iter_mut().skip(i);
        let mut rhs = rhs.iter().copied();
        let mut carry = 0_u128;
        loop {
            match (result.next(), rhs.next()) {
                // Partial product.
                (Some(result), Some(rhs)) => {
                    carry += u128::from(*result) + u128::from(lhs) * u128::from(rhs);
                    *result = carry as u64;
                    carry >>= 64;
                }
                // Carry propagation.
                (Some(result), None) => {
                    carry += u128::from(*result);
                    *result = carry as u64;
                    carry >>= 64;
                }
                // Excess rhs
                (None, Some(rhs)) => {
                    carry += u128::from(lhs) * u128::from(rhs);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vals(lhs: &[u64], rhs: &[u64], expected: &[u64], expected_overflow: bool) {
        let mut result = vec![0; expected.len()];
        let overflow = mul(lhs, rhs, &mut result);
        assert_eq!(overflow, expected_overflow);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty() {
        test_vals(&[], &[], &[], false);
        test_vals(&[], &[], &[0], false);
        test_vals(&[], &[1], &[], false);
        test_vals(&[1], &[], &[], false);
        test_vals(&[1], &[1], &[], true);
        test_vals(&[1], &[], &[0], false);
        test_vals(&[], &[1], &[0], false);
        test_vals(&[1], &[1], &[1], false);
    }
}
