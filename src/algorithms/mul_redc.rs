use core::{hint::black_box, iter::zip};

/// Computes
///
/// (a * b) / 2^BITS mod modulus
///
/// TODO: Do the inputs need to be reduced?
#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn mul_redc<const N: usize>(a: [u64; N], b: [u64; N], modulus: [u64; N], inv: u64) -> [u64; N] {
    debug_assert_eq!(inv.wrapping_mul(modulus[0]), u64::MAX);
    // debug_assert!(a[N - 1] <= modulus[N - 1]);
    // debug_assert!(b[N - 1] <= modulus[N - 1]);

    // Coarsely Integrated Operand Scanning (CIOS)
    // See <https://www.microsoft.com/en-us/research/wp-content/uploads/1998/06/97Acar.pdf>
    // See <https://hackmd.io/@gnark/modular_multiplication#fn1>
    let mut result = [0; N];
    let mut carry = false;
    for b in b {
        let mut m = 0;
        let mut carry_1 = 0;
        let mut carry_2 = 0;
        for i in 0..N {
            // Add limb product
            let (value, next_carry) = carrying_mul_add(a[i], b, result[i], carry_1);
            carry_1 = next_carry;

            if i == 0 {
                // Compute reduction factor
                m = value.wrapping_mul(inv);
            }

            // Add m * modulus to acc to clear next_result[0]
            let (value, next_carry) = carrying_mul_add(modulus[i], m, value, carry_2);
            carry_2 = next_carry;

            // Shift result
            if i > 0 {
                result[i - 1] = value;
            } else {
                debug_assert_eq!(value, 0);
            }
        }

        // Add carries
        let (value, next_carry) = carrying_add(carry_1, carry_2, carry);
        result[N - 1] = value;
        if modulus[N - 1] >= 0x7fff_ffff_ffff_ffff {
            carry = next_carry;
        } else {
            debug_assert!(!next_carry);
        }
    }

    // Compute reduced product.
    reduce1_carry(result, modulus, carry)
}

/// Computes
///
/// a^2 / 2^BITS mod modulus
///
/// TODO: Do the inputs need to be reduced?
#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn square_redc<const N: usize>(a: [u64; N], modulus: [u64; N], inv: u64) -> [u64; N] {
    debug_assert_eq!(inv.wrapping_mul(modulus[0]), u64::MAX);
    // debug_assert!(a[N - 1] <= modulus[N - 1]);

    // Coarsely Integrated Operand Scanning (CIOS)
    // See <https://www.microsoft.com/en-us/research/wp-content/uploads/1998/06/97Acar.pdf>
    // See <https://hackmd.io/@gnark/modular_multiplication#fn1>
    let mut result = [0; N];
    let mut carry = false;
    for limb in a {
        let mut m = 0;
        let mut carry_1 = 0;
        let mut carry_2 = 0;
        for i in 0..N {
            // Add limb product
            let (value, next_carry) = carrying_mul_add(a[i], limb, result[i], carry_1);
            carry_1 = next_carry;

            if i == 0 {
                // Compute reduction factor
                m = value.wrapping_mul(inv);
            }

            // Add m * modulus to acc to clear next_result[0]
            let (value, next_carry) = carrying_mul_add(modulus[i], m, value, carry_2);
            carry_2 = next_carry;

            // Shift result
            if i > 0 {
                result[i - 1] = value;
            } else {
                debug_assert_eq!(value, 0);
            }
        }

        // Add carries
        let (value, next_carry) = carrying_add(carry_1, carry_2, carry);
        result[N - 1] = value;
        if modulus[N - 1] >= 0x7fff_ffff_ffff_ffff {
            carry = next_carry;
        } else {
            debug_assert!(!next_carry);
        }
    }

    // Compute reduced product.
    reduce1_carry(result, modulus, carry)
}

pub fn reduce1_carry<const N: usize>(value: [u64; N], modulus: [u64; N], carry: bool) -> [u64; N] {
    let (reduced, borrow) = sub(value, modulus);
    // Using `black_box` here does the oposite of `likely` and `unlikely` and causes
    // the compiler to generate conditional moves/selects instead of a branch.
    if black_box(carry || !borrow) {
        reduced
    } else {
        value
    }
}

pub fn reduce1_carry_constime<const N: usize>(
    value: [u64; N],
    modulus: [u64; N],
    carry: bool,
) -> [u64; N] {
    let (reduced, borrow) = sub(value, modulus);
    let mut result = value;
    cmov(&mut result, &reduced, carry || !borrow);
    result
}

#[inline]
pub fn cmov<const N: usize>(dst: &mut [u64; N], src: &[u64; N], condition: bool) {
    let mask = (condition as u64).wrapping_neg();
    let mask = core::hint::black_box(mask);
    for (dst, &src) in zip(dst, src) {
        *dst ^= (*dst ^ src) & mask;
    }
}

#[inline]
#[must_use]
fn add<const N: usize>(lhs: [u64; N], rhs: [u64; N]) -> ([u64; N], bool) {
    let mut result = [0; N];
    let mut carry = false;
    for (result, (lhs, rhs)) in zip(&mut result, zip(lhs, rhs)) {
        let (value, next_carry) = borrowing_sub(lhs, rhs, carry);
        *result = value;
        carry = next_carry;
    }
    (result, carry)
}

#[inline]
#[must_use]
fn sub<const N: usize>(lhs: [u64; N], rhs: [u64; N]) -> ([u64; N], bool) {
    let mut result = [0; N];
    let mut borrow = false;
    for (result, (lhs, rhs)) in zip(&mut result, zip(lhs, rhs)) {
        let (value, next_borrow) = borrowing_sub(lhs, rhs, borrow);
        *result = value;
        borrow = next_borrow;
    }
    (result, borrow)
}

/// Compute `accumulator + lhs * rhs` for a single word `rhs`, returing the
/// result and carry.
#[inline]
#[must_use]
pub fn mul_add_small<const N: usize>(lhs: [u64; N], rhs: u64, add: [u64; N]) -> ([u64; N], u64) {
    let mut result = [0; N];
    let mut carry = 0;
    for (result, (lhs, add)) in zip(&mut result, zip(lhs, add)) {
        let (value, next_carry) = carrying_mul_add(lhs, rhs, add, carry);
        *result = value;
        carry = next_carry;
    }
    (result, carry)
}

const fn carrying_mul_add(lhs: u64, rhs: u64, add: u64, carry: u64) -> (u64, u64) {
    let wide = (lhs as u128)
        .wrapping_mul(rhs as u128)
        .wrapping_add(add as u128)
        .wrapping_add(carry as u128);
    (wide as u64, (wide >> 64) as u64)
}

// Helper while [Rust#85532](https://github.com/rust-lang/rust/issues/85532) stabilizes.
#[must_use]
#[inline]
const fn carrying_add(lhs: u64, rhs: u64, carry: bool) -> (u64, bool) {
    let (result, carry_1) = lhs.overflowing_add(rhs);
    let (result, carry_2) = result.overflowing_add(carry as u64);
    (result, carry_1 || carry_2)
}

// Helper while [Rust#85532](https://github.com/rust-lang/rust/issues/85532) stabilizes.
#[must_use]
#[inline]
const fn borrowing_sub(lhs: u64, rhs: u64, borrow: bool) -> (u64, bool) {
    let (result, borrow_1) = lhs.overflowing_sub(rhs);
    let (result, borrow_2) = result.overflowing_sub(borrow as u64);
    (result, borrow_1 || borrow_2)
}

// Helper while [Rust#85532](https://github.com/rust-lang/rust/issues/85532) stabilizes.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation)]
const fn carrying_mul(lhs: u64, rhs: u64, carry: u64) -> (u64, u64) {
    let wide = (lhs as u128)
        .wrapping_mul(rhs as u128)
        .wrapping_add(carry as u128);
    (wide as u64, (wide >> 64) as u64)
}

#[cfg(test)]
mod test {
    use super::{
        super::{addmul, div},
        *,
    };
    use crate::{aliases::U64, const_for, nlimbs, Uint};
    use proptest::{prop_assert_eq, prop_assume, proptest};

    fn reference<const N: usize>(a: [u64; N], b: [u64; N], modulus: [u64; N]) -> [u64; N] {
        // Compute a * b * 2^(64 * N)
        let mut product = vec![0; 3 * N];
        addmul(&mut product[N..], &a, &b);

        // Compute product mod modulus
        let mut reduced = modulus;
        div(&mut product, &mut reduced);
        reduced
    }

    #[test]
    fn test_mul_redc() {
        const_for!(BITS in NON_ZERO if (BITS >= 16) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, m: U)| {
                let a = *a.as_limbs();
                let b = *b.as_limbs();
                let m = *m.as_limbs();

                // Make sure m has an inverse.
                let inv = U64::from(m[0]).inv_ring();
                prop_assume!(inv.is_some());
                let inv = (-inv.unwrap()).as_limbs()[0];

                let result = mul_redc(a, b, m, inv);
                let expected = reference(a, b, m);

                prop_assert_eq!(result, expected);
            });
        });
    }

    #[test]
    fn test_square_redc() {
        const_for!(BITS in NON_ZERO if (BITS >= 16) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, m: U)| {
                let a = *a.as_limbs();
                let m = *m.as_limbs();

                // Make sure m has an inverse.
                let inv = U64::from(m[0]).inv_ring();
                prop_assume!(inv.is_some());
                let inv = (-inv.unwrap()).as_limbs()[0];

                let result = square_redc(a, m, inv);
                let expected = reference(a, a, m);

                prop_assert_eq!(result, expected);
            });
        });
    }
}
