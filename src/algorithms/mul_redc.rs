use core::iter::zip;

/// Computes
///
/// (a * b) / 2^BITS mod modulus
#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn mul_redc<const N: usize>(
    a: &[u64; N],
    b: &[u64; N],
    modulus: &[u64; N],
    inv: u64,
) -> [u64; N] {
    // Coarsely Integrated Operand Scanning (CIOS)
    // See <https://www.microsoft.com/en-us/research/wp-content/uploads/1998/06/97Acar.pdf>
    // See <https://hackmd.io/@gnark/modular_multiplication#fn1>
    let mut result = [0; N];
    let mut carry = 0;
    for &b in b {
        // Add limb product
        let c0 = addmul1(&mut result, a, b);

        // Compute reduction factor
        let m = result[0].wrapping_mul(inv);

        // Add m * modulus to acc to clear acc[0]
        let c1 = addmul1(&mut result, modulus, m);
        debug_assert_eq!(result[0], 0);

        // Shift result
        // TODO: Merge with above addmul1 loop. Or merge both loops to get finely
        // integrated operand scanning (FIOS)
        for i in 0..N - 1 {
            result[i] = result[i + 1];
        }

        // Add carries
        // TODO: Can skip this step if modulus meets certain criteria.
        // TODO: Is carry alwasy 0 or 1?
        let r = (carry as u128) + (c0 as u128) + (c1 as u128);
        result[N - 1] = r as u64;
        carry = (r >> 64) as u64;
        debug_assert!(carry == 0 || carry == 1);
    }

    // Compute reduced product.
    let (reduced, borrow) = sub(&result, modulus);
    if carry == 1 || !borrow {
        reduced
    } else {
        result
    }
}

#[inline]
pub fn cmov<const N: usize>(dst: &mut [u64; N], src: &[u64; N], condition: bool) {
    let mask = (condition as u64).wrapping_neg();
    let mask = core::hint::black_box(mask);
    for (dst, &src) in zip(dst, src) {
        *dst ^= (*dst ^ src) & mask;
    }
}

/// Compute `acc += a * b` for a single word `b`, returing the carry.
#[allow(clippy::cast_possible_truncation)]
#[inline]
#[must_use]
pub fn addmul1<const N: usize>(acc: &mut [u64; N], a: &[u64; N], b: u64) -> u64 {
    let mut carry = 0;
    for i in 0..N {
        let r = (acc[i] as u128) + (a[i] as u128) * (b as u128) + (carry as u128);
        acc[i] = r as u64;
        carry = (r >> 64) as u64;
    }
    carry
}

#[allow(clippy::cast_possible_truncation)]
#[inline]
#[must_use]
fn sub<const N: usize>(a: &[u64; N], b: &[u64; N]) -> ([u64; N], bool) {
    let mut result = [0; N];
    let mut borrow = false;
    for (r, (&a, &b)) in zip(&mut result, zip(a, b)) {
        let (s, b) = a.overflowing_sub(b);
        let (s, d) = s.overflowing_sub(borrow as u64);
        *r = s;
        borrow = b || d;
    }
    (result, borrow)
}

fn borrowing_sub(lhs: u64, rhs: u64, borrow: bool) -> (u64, bool) {
    let (a, b) = lhs.overflowing_sub(rhs);
    let (c, d) = a.overflowing_sub(borrow as u64);
    (c, b || d)
}
