#![allow(dead_code)] // TODO

use super::ops::{adc, sbb};
use core::cmp::Ordering;

#[inline(always)]
#[must_use]
pub fn cmp(lhs: &[u64], rhs: &[u64]) -> Ordering {
    debug_assert_eq!(lhs.len(), rhs.len());
    for (l, r) in lhs.iter().rev().zip(rhs.iter().rev()) {
        match l.cmp(r) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

/// `lhs += rhs + carry`
#[inline(always)]
pub fn adc_n(lhs: &mut [u64], rhs: &[u64], mut carry: u64) -> u64 {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        let (result, new_carry) = adc(*l, *r, carry);
        *l = result;
        carry = new_carry;
    }
    carry
}

/// `lhs -= rhs + carry`
#[inline(always)]
pub fn sbb_n(lhs: &mut [u64], rhs: &[u64], mut carry: u64) -> u64 {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        let (result, new_carry) = sbb(*l, *r, carry);
        *l = result;
        carry = new_carry;
    }
    carry
}
