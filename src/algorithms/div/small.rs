//! Small division using reciprocals from [MG10].
//!
//! [MG10]: https://gmplib.org/~tege/division-paper.pdf

// Following naming from paper.
#![allow(clippy::many_single_char_names, clippy::similar_names)]
// Truncation is intentional
#![allow(clippy::cast_possible_truncation)]
#![allow(dead_code)] // TODO

use super::reciprocal::{reciprocal, reciprocal_2};
use core::intrinsics::unlikely;

pub use self::{div_2x1_mg10 as div_2x1, div_3x2_mg10 as div_3x2};

/// Compute single limb division.
///
/// The divisor must be normalized. See algorithm 7 from [MG10].
///
/// [MG10]: https://gmplib.org/~tege/division-paper.pdf
#[inline(always)]
pub fn div_nx1(u: &mut [u64], d: u64) -> u64 {
    debug_assert!(d >= (1 << 63));

    let v = reciprocal(d);
    let mut r: u64 = 0;
    for u in u.iter_mut().rev() {
        let n = (u128::from(r) << 64) | u128::from(*u);
        let (q, r0) = div_2x1(n, d, v);
        *u = q;
        r = r0;
    }
    r
}

/// Compute double limb division.
///
/// The divisor must be normalized. This is a variant of [`div_nx1`] using
/// [`div_3x2`] internally.
#[inline(always)]
pub fn div_nx2(u: &mut [u64], d: u128) -> u128 {
    debug_assert!(d >= (1 << 127));

    let v = reciprocal_2(d);
    let mut remainder: u128 = 0;
    for u in u.iter_mut().rev() {
        let (q, r) = div_3x2(remainder, *u, d, v);
        *u = q;
        remainder = r;
    }
    remainder
}

#[inline(always)]
pub fn div_2x1_ref(u: u128, d: u64) -> (u64, u64) {
    debug_assert!(d >= (1 << 63));
    debug_assert!((u >> 64) < u128::from(d));
    let d = u128::from(d);
    let q = (u / d) as u64;
    let r = (u % d) as u64;
    (q, r)
}

/// Computes the quotient and remainder of a `u128` divided by a `u64`.
///
/// Requires
/// * `u < d * 2**64`,
/// * `d >= 2**63`, and
/// * `v = reciprocal(d)`.
///
/// Implements algorithm 4 from [MG10]. The running time is 2.7 ns versus 18 ns
/// for [`div_2x1_ref`].
///
/// [MG10]: https://gmplib.org/~tege/division-paper.pdf
#[inline(always)]
pub fn div_2x1_mg10(u: u128, d: u64, v: u64) -> (u64, u64) {
    debug_assert!(d >= (1 << 63));
    debug_assert!((u >> 64) < u128::from(d));
    debug_assert_eq!(v, reciprocal(d));

    let q = u + (u >> 64) * u128::from(v);
    let q0 = q as u64;
    let q1 = ((q >> 64) as u64).wrapping_add(1);
    let r = (u as u64).wrapping_sub(q1.wrapping_mul(d));
    let (q1, r) = if r > q0 {
        (q1.wrapping_sub(1), r.wrapping_add(d))
    } else {
        (q1, r)
    };
    let (q1, r) = if unlikely(r >= d) {
        (q1.wrapping_add(1), r.wrapping_sub(d))
    } else {
        (q1, r)
    };
    (q1, r)
}

/// TODO: This implementation is off by one.
#[inline(always)]
pub fn div_3x2_ref(n21: u128, n0: u64, d: u128) -> u64 {
    debug_assert!(d >= (1 << 127));
    debug_assert!(n21 < d);

    let n2 = (n21 >> 64) as u64;
    let n1 = n21 as u64;
    let d1 = (d >> 64) as u64;
    let d0 = d as u64;

    if unlikely(n2 == d1) {
        // From [n2 n1] < [d1 d0] and n2 = d1 it follows that n[1] < d[0].
        debug_assert!(n1 < d0);
        // We start by subtracting 2^64 times the divisor, resulting in a
        // negative remainder. Depending on the result, we need to add back
        // in one or two times the divisor to make the remainder positive.
        // (It can not be more since the divisor is > 2^127 and the negated
        // remainder is < 2^128.)
        let neg_remainder = u128::from(d0).wrapping_sub((u128::from(n1) << 64) | u128::from(n0));
        if neg_remainder > d {
            0xffff_ffff_ffff_fffe_u64
        } else {
            0xffff_ffff_ffff_ffff_u64
        }
    } else {
        // Compute quotient and remainder
        let (mut q, mut r) = div_2x1_ref(n21, d1);

        let t1 = u128::from(q) * u128::from(d0);
        let t2 = (u128::from(n0) << 64) | u128::from(r);
        if t1 > t2 {
            q -= 1;
            r = r.wrapping_add(d1);
            let overflow = r < d1;
            if !overflow {
                let t1 = u128::from(q) * u128::from(d0);
                let t2 = (u128::from(n0) << 64) | u128::from(r);
                if t1 > t2 {
                    q -= 1;
                    // UNUSED: r += d[1];
                }
            }
        }
        q
    }
}

#[inline(always)]
pub fn div_3x2_mg10(u21: u128, u0: u64, d10: u128, v: u64) -> (u64, u128) {
    debug_assert!(d10 >= (1 << 127));
    debug_assert!(u21 < d10);
    debug_assert_eq!(v, reciprocal_2(d10));

    let u1 = u21 as u64;
    let d1 = (d10 >> 64) as u64;
    let d0 = d10 as u64;
    let q10 = u128::from(v) * (u21 >> 64) + u21;
    let mut q1 = (q10 >> 64) as u64;
    let q0 = q10 as u64;
    let r1 = u1.wrapping_sub(q1.wrapping_mul(d1));
    let t10 = u128::from(d0) * u128::from(q1);
    let mut r10 = ((u128::from(r1) << 64) | u128::from(u0))
        .wrapping_sub(t10)
        .wrapping_sub(d10);
    let r1 = (r10 >> 64) as u64;
    q1 = q1.wrapping_add(1);
    if r1 >= q0 {
        q1 = q1.wrapping_sub(1);
        r10 = r10.wrapping_add(d10);
    }
    if unlikely(r10 >= d10) {
        q1 = q1.wrapping_add(1);
        r10 = r10.wrapping_sub(d10);
    }
    (q1, r10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn test_div_2x1_mg10() {
        proptest!(|(q: u64, r: u64, mut d: u64)| {
            let d = d | (1 << 63);
            let r = r % d;
            let n = u128::from(q) * u128::from(d) + u128::from(r);
            let v = reciprocal(d);
            assert_eq!(div_2x1_mg10(n, d, v), (q,r));
        });
    }

    #[test]
    fn test_div_3x2_ref() {
        proptest!(|(q: u64, r: u128, mut d: u128)| {
            let d = d | (1 << 127);
            let r = r % d;
            let (n21, n0) = {
                let d1 = (d >> 64) as u64;
                let d0 = d as u64;
                let r1 = (r >> 64) as u64;
                let r0 = r as u64;
                // n = q * d + r
                let n10 = u128::from(q) * u128::from(d0) + u128::from(r0);
                let n0 = n10 as u64;
                let n21 = (n10 >> 64) + u128::from(q) * u128::from(d1) + u128::from(r1);
                (n21, n0)
            };
            assert_eq!(div_3x2_ref(n21, n0, d), q);
        });
    }

    #[test]
    fn test_div_3x2_mg10() {
        proptest!(|(q: u64, r: u128, mut d: u128)| {
            let d = d | (1 << 127);
            let r = r % d;
            let (n21, n0) = {
                let d1 = (d >> 64) as u64;
                let d0 = d as u64;
                let r1 = (r >> 64) as u64;
                let r0 = r as u64;
                // n = q * d + r
                let n10 = u128::from(q) * u128::from(d0) + u128::from(r0);
                let n0 = n10 as u64;
                let n21 = (n10 >> 64) + u128::from(q) * u128::from(d1) + u128::from(r1);
                (n21, n0)
            };
            let v = reciprocal_2(d);
            assert_eq!(div_3x2_mg10(n21, n0, d, v), (q, r));
        });
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::{black_box, BatchSize, Criterion};
    use rand::{thread_rng, Rng};

    pub fn group(criterion: &mut Criterion) {
        bench_div_2x1_ref(criterion);
        bench_div_2x1_mg10(criterion);
        bench_div_3x2_ref(criterion);
        bench_div_3x2_mg10(criterion);
    }

    fn bench_div_2x1_ref(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("algo/div/2x1/ref", move |bencher| {
            bencher.iter_batched(
                || {
                    let q: u64 = rng.gen();
                    let r: u64 = rng.gen();
                    let d = rng.gen::<u64>() | (1 << 63);
                    let r = r % d;
                    let n = u128::from(q) * u128::from(d) + u128::from(r);
                    (n, d)
                },
                |(u, d)| black_box(div_2x1_ref(u, d)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_div_2x1_mg10(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("algo/div/2x1/mg10", move |bencher| {
            bencher.iter_batched(
                || {
                    let q: u64 = rng.gen();
                    let r: u64 = rng.gen();
                    let d = rng.gen::<u64>() | (1 << 63);
                    let r = r % d;
                    let n = u128::from(q) * u128::from(d) + u128::from(r);
                    let v = reciprocal(d);
                    (n, d, v)
                },
                |(u, d, v)| black_box(div_2x1_mg10(u, d, v)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_div_3x2_ref(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("algo/div/3x2/ref", move |bencher| {
            bencher.iter_batched(
                || {
                    let q: u64 = rng.gen();
                    let r: u128 = rng.gen();
                    let d = rng.gen::<u128>() | (1 << 127);
                    let r = r % d;
                    let (n21, n0) = {
                        let d1 = (d >> 64) as u64;
                        let d0 = d as u64;
                        let r1 = (r >> 64) as u64;
                        let r0 = r as u64;
                        // n = q * d + r
                        let n10 = u128::from(q) * u128::from(d0) + u128::from(r0);
                        let n0 = n10 as u64;
                        let n21 = (n10 >> 64) + u128::from(q) * u128::from(d1) + u128::from(r1);
                        (n21, n0)
                    };
                    (n21, n0, d)
                },
                |(n21, n0, d)| black_box(div_3x2_ref(n21, n0, d)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_div_3x2_mg10(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("algo/div/3x2/mg10", move |bencher| {
            bencher.iter_batched(
                || {
                    let q: u64 = rng.gen();
                    let r: u128 = rng.gen();
                    let d = rng.gen::<u128>() | (1 << 127);
                    let r = r % d;
                    let (n21, n0) = {
                        let d1 = (d >> 64) as u64;
                        let d0 = d as u64;
                        let r1 = (r >> 64) as u64;
                        let r0 = r as u64;
                        // n = q * d + r
                        let n10 = u128::from(q) * u128::from(d0) + u128::from(r0);
                        let n0 = n10 as u64;
                        let n21 = (n10 >> 64) + u128::from(q) * u128::from(d1) + u128::from(r1);
                        (n21, n0)
                    };
                    let v = reciprocal_2(d);
                    (n21, n0, d, v)
                },
                |(n21, n0, d, v)| black_box(div_3x2_mg10(n21, n0, d, v)),
                BatchSize::SmallInput,
            );
        });
    }
}
