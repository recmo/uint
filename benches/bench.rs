#![allow(clippy::incompatible_msrv)]

use criterion::{criterion_group, criterion_main};

mod benches;
mod prelude;

criterion_group!(benches, benches::group);
criterion_main!(benches);
