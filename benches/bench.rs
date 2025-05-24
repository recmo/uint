#![allow(clippy::incompatible_msrv)]

use criterion::criterion_main;

mod benches;
mod prelude;

criterion_main!(benches::group);
