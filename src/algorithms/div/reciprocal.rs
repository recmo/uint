//! Reciprocals and division using reciprocals
//! See [MG10].
//!
//! [MG10]: https://gmplib.org/~tege/division-paper.pdf
//! https://gmplib.org/~tege/divcnst-pldi94.pdf
//! https://homepage.divms.uiowa.edu/~jones/bcd/divide.html
//! https://libdivide.com/
//! https://github.com/ridiculousfish/libdivide
//! https://gmplib.org/list-archives/gmp-devel/2019-October/005590.html
//!
//! https://nnethercote.github.io/perf-book/profiling.html

use std::num::Wrapping;

use core::intrinsics::unlikely;

/// The MG10 algorithm is faster (3ns vs 20ns).
pub use self::reciprocal_mg10 as reciprocal;

/// Computes $\floor{\frac{2^{128} - 1}}{\mathtt{d}}} - 2^{64}$.
///
/// Requires $\mathtt{d} ≥ 2^{127}$, i.e. the highest bit of $\mathtt{d}$ must
/// be set.
#[inline(always)]
pub fn reciprocal_ref(d: u64) -> u64 {
    debug_assert!(d >= (1 << 63));
    let mut r = u128::MAX / u128::from(d);
    debug_assert!(r >= (1 << 64));
    debug_assert!(r < (1 << 65));
    r as u64
}

#[inline(always)]
fn mul_hi(a: Wrapping<u64>, b: Wrapping<u64>) -> Wrapping<u64> {
    let a = u128::from(a.0);
    let b = u128::from(b.0);
    let r = a * b;
    Wrapping((r >> 64) as u64)
}

#[inline(always)]
fn muladd_hi(a: Wrapping<u64>, b: Wrapping<u64>, c: Wrapping<u64>) -> Wrapping<u64> {
    let a = u128::from(a.0);
    let b = u128::from(b.0);
    let c = u128::from(c.0);
    let r = a * b + c;
    Wrapping((r >> 64) as u64)
}

/// Computes $\floor{\frac{2^{128} - 1}}{\mathtt{d}}} - 2^{64}$.
///
/// Using [MG10] algorithm 2. See also the [intx] implementation. Here is a
/// direct translation of the algorithm to Python for reference:
///
/// ```python
/// d0 = d % 2
/// d9 = d // 2**55
/// d40 = d // 2**24 + 1
/// d63 = (d + 1) // 2
/// v0 = (2**19 - 3 * 2**8) // d9
/// v1 = 2**11 * v0 - v0**2 * d40 // 2**40 - 1
/// v2 = 2**13 * v1 + v1 * (2**60 - v1 * d40) // 2**47
/// e = 2**96 - v2 * d63 + (v2 // 2) * d0
/// v3 = (2**31 * v2 +v2 * e // 2**65) % 2**64
/// v4 = (v3 - (v3 + 2**64 + 1) * d // 2**64) % 2**64
/// ```
///
/// [MG10]: https://gmplib.org/~tege/division-paper.pdf
/// [intx]: https://github.com/chfast/intx/blob/8b5f4748a7386a9530769893dae26b3273e0ffe2/include/intx/intx.hpp#L683
#[inline(always)]
pub fn reciprocal_mg10(d: u64) -> u64 {
    debug_assert!(d >= (1 << 63));
    let d = Wrapping(d);
    const ZERO: Wrapping<u64> = Wrapping(0);
    const ONE: Wrapping<u64> = Wrapping(1);

    // Lookup table for $\floor{\frac{2^{19} -3 ⋅ 2^8}{d_9 - 256}}$
    const TABLE: [u16; 256] = [
        2045, 2037, 2029, 2021, 2013, 2005, 1998, 1990, 1983, 1975, 1968, 1960, 1953, 1946, 1938,
        1931, 1924, 1917, 1910, 1903, 1896, 1889, 1883, 1876, 1869, 1863, 1856, 1849, 1843, 1836,
        1830, 1824, 1817, 1811, 1805, 1799, 1792, 1786, 1780, 1774, 1768, 1762, 1756, 1750, 1745,
        1739, 1733, 1727, 1722, 1716, 1710, 1705, 1699, 1694, 1688, 1683, 1677, 1672, 1667, 1661,
        1656, 1651, 1646, 1641, 1636, 1630, 1625, 1620, 1615, 1610, 1605, 1600, 1596, 1591, 1586,
        1581, 1576, 1572, 1567, 1562, 1558, 1553, 1548, 1544, 1539, 1535, 1530, 1526, 1521, 1517,
        1513, 1508, 1504, 1500, 1495, 1491, 1487, 1483, 1478, 1474, 1470, 1466, 1462, 1458, 1454,
        1450, 1446, 1442, 1438, 1434, 1430, 1426, 1422, 1418, 1414, 1411, 1407, 1403, 1399, 1396,
        1392, 1388, 1384, 1381, 1377, 1374, 1370, 1366, 1363, 1359, 1356, 1352, 1349, 1345, 1342,
        1338, 1335, 1332, 1328, 1325, 1322, 1318, 1315, 1312, 1308, 1305, 1302, 1299, 1295, 1292,
        1289, 1286, 1283, 1280, 1276, 1273, 1270, 1267, 1264, 1261, 1258, 1255, 1252, 1249, 1246,
        1243, 1240, 1237, 1234, 1231, 1228, 1226, 1223, 1220, 1217, 1214, 1211, 1209, 1206, 1203,
        1200, 1197, 1195, 1192, 1189, 1187, 1184, 1181, 1179, 1176, 1173, 1171, 1168, 1165, 1163,
        1160, 1158, 1155, 1153, 1150, 1148, 1145, 1143, 1140, 1138, 1135, 1133, 1130, 1128, 1125,
        1123, 1121, 1118, 1116, 1113, 1111, 1109, 1106, 1104, 1102, 1099, 1097, 1095, 1092, 1090,
        1088, 1086, 1083, 1081, 1079, 1077, 1074, 1072, 1070, 1068, 1066, 1064, 1061, 1059, 1057,
        1055, 1053, 1051, 1049, 1047, 1044, 1042, 1040, 1038, 1036, 1034, 1032, 1030, 1028, 1026,
        1024,
    ];

    let d0 = d & ONE;
    let d9 = d >> 55;
    let d40 = ONE + (d >> 24);
    let d63 = (d + ONE) >> 1;
    let v0 = Wrapping(TABLE[(d9.0 - 256) as usize] as u64);
    let v1 = (v0 << 11) - ((v0 * v0 * d40) >> 40) - ONE;
    let v2 = (v1 << 13) + (v1 * ((ONE << 60) - v1 * d40) >> 47);
    let e = ((v2 >> 1) & (ZERO - d0)) - v2 * d63;
    let v3 = (mul_hi(v2, e) >> 1) + (v2 << 31);
    let v4 = v3 - muladd_hi(v3, d, d) - d;

    v4.0
}

#[inline(always)]
pub fn div_2x1_ref(u: u128, d: u64) -> (u64, u64) {
    debug_assert!(d >= (1 << 63));
    debug_assert!((u >> 64) < d as u128);
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
    debug_assert!((u >> 64) < d as u128);
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn test_reciprocal() {
        proptest!(|(n: u64)| {
            let n = n | (1 << 63);
            let expected = reciprocal_ref(n);
            let actual = reciprocal_mg10(n);
            assert_eq!(expected, actual);
        });
    }

    #[test]
    fn test_div_2x1_mg10() {
        proptest!(|(q: u64, r: u64, mut d: u64)| {
            let d = d | (1 << 63);
            let r = r % d;
            let n = u128::from(q) * u128::from(d) + u128::from(r);
            let v = reciprocal(d);
            let actual = div_2x1_mg10(n, d, v);
            assert_eq!((q,r), actual);
        });
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use crate::{const_for, nlimbs};
    use criterion::{black_box, BatchSize, Criterion};
    use rand::{thread_rng, Rng};

    pub fn group(criterion: &mut Criterion) {
        bench_reciprocal_ref(criterion);
        bench_reciprocal_mg10(criterion);
        bench_div_2x1_ref(criterion);
        bench_div_2x1_mg10(criterion);
    }

    fn bench_reciprocal_ref(criterion: &mut Criterion) {
        let mut rng = rand::thread_rng();
        criterion.bench_function("algo/div/reciprocal/ref", move |bencher| {
            bencher.iter_batched(
                || rng.gen::<u64>() | (1 << 63),
                |a| black_box(reciprocal_ref(black_box(a))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_reciprocal_mg10(criterion: &mut Criterion) {
        let mut rng = rand::thread_rng();
        criterion.bench_function("algo/div/reciprocal/mg10", move |bencher| {
            bencher.iter_batched(
                || rng.gen::<u64>() | (1 << 63),
                |a| black_box(reciprocal_mg10(black_box(a))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_div_2x1_ref(criterion: &mut Criterion) {
        let mut rng = rand::thread_rng();
        criterion.bench_function("algo/div/div_2x1/ref", move |bencher| {
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
        let mut rng = rand::thread_rng();
        criterion.bench_function("algo/div/div_2x1/mg10", move |bencher| {
            bencher.iter_batched(
                || {
                    let q: u64 = rng.gen();
                    let r: u64 = rng.gen();
                    let d = rng.gen::<u64>() | (1 << 63);
                    let r = r % d;
                    let n = u128::from(q) * u128::from(d) + u128::from(r);
                    let v = reciprocal_mg10(d);
                    (n, d, v)
                },
                |(u, d, v)| black_box(div_2x1_mg10(u, d, v)),
                BatchSize::SmallInput,
            );
        });
    }
}
