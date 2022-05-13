#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
// Required
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(specialization)]

mod uint;

pub use uint::*;

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        uint::bench::group(criterion);
    }
}
