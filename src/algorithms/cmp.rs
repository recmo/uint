use super::borrowing_sub;
use core::cmp::Ordering;

/// Compare two limb slices in reverse order.
///
/// Assumes that if the slices are of different length, the longer slice is
/// always greater than the shorter slice.
#[inline(always)]
#[must_use]
pub fn cmp(a: &[u64], b: &[u64]) -> Ordering {
    match a.len().cmp(&b.len()) {
        Ordering::Equal => {}
        non_eq => return non_eq,
    }
    for i in (0..a.len()).rev() {
        match i8::from(a[i] > b[i]) - i8::from(a[i] < b[i]) {
            -1 => return Ordering::Less,
            0 => {}
            1 => return Ordering::Greater,
            _ => unsafe { core::hint::unreachable_unchecked() },
        }

        // Equivalent to the following code, but on older rustc versions
        // performs better:
        // match a[i].cmp(&b[i]) {
        //     Ordering::Equal => {}
        //     non_eq => return non_eq,
        // }
    }
    Ordering::Equal
}

macro_rules! cmp_fns {
    ($($name:ident, $op:literal => |$a:ident, $b:ident| $impl:expr),* $(,)?) => {
        $(
            /// Compare two limb slices in reverse order, returns `true` if
            #[doc = concat!("`a ", $op, " b`.")]
            ///
            /// Assumes that if the slices are of different length, the longer slice is
            /// always greater than the shorter slice.
            #[inline(always)]
            #[must_use]
            pub fn $name($a: &[u64], $b: &[u64]) -> bool {
                $impl
            }
        )*
    };
}

cmp_fns! {
    // lt, "<"  => |a, b| match a.len().cmp(&b.len()) {
    //     Ordering::Equal => lt_thru_sub(a, b),
    //     non_eq => non_eq.is_lt(),
    // },
    lt, "<"  => |a, b| cmp(a, b).is_lt(),
    gt, ">"  => |a, b| lt(b, a),
    ge, ">=" => |a, b| !lt(a, b),
    le, "<=" => |a, b| !lt(b, a),
}

#[inline]
#[allow(dead_code)]
fn lt_thru_sub(a: &[u64], b: &[u64]) -> bool {
    assume!(a.len() == b.len());
    let mut borrow = false;
    let mut acc = 0;
    for i in 0..a.len() {
        let x;
        (x, borrow) = borrowing_sub(a[i], b[i], borrow);
        acc |= x;
    }
    unsafe { core::ptr::write_volatile(&mut acc, acc) };
    borrow
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prop_assert_eq;

    proptest::proptest! {
        #[test]
        fn test_cmp_u64(a: u64, b: u64) {
            let x = &[a];
            let y = &[b];
            prop_assert_eq!(cmp(x, y), a.cmp(&b));
            prop_assert_eq!(lt(x, y), a < b);
            prop_assert_eq!(gt(x, y), a > b);
            prop_assert_eq!(ge(x, y), a >= b);
            prop_assert_eq!(le(x, y), a <= b);
        }

        #[test]
        fn test_cmp_u128(a: u128, b: u128) {
            let x = &[a as u64, (a >> 64) as u64];
            let y = &[b as u64, (b >> 64) as u64];
            prop_assert_eq!(cmp(x, y), a.cmp(&b));
            prop_assert_eq!(lt(x, y), a < b);
            prop_assert_eq!(gt(x, y), a > b);
            prop_assert_eq!(ge(x, y), a >= b);
            prop_assert_eq!(le(x, y), a <= b);
        }
    }
}
