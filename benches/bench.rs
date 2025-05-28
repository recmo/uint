#![allow(clippy::incompatible_msrv)]
#![allow(unexpected_cfgs)]

use criterion::{criterion_group, criterion_main};

mod benches;
pub(crate) use benches::prelude;

criterion_group!(benches, benches::group);
criterion_main!(benches);
