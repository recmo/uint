mod benches;

mod prelude {
    pub use criterion::{BatchSize, Criterion};
    pub use proptest::{
        arbitrary::Arbitrary,
        strategy::{Strategy, ValueTree},
        test_runner::TestRunner,
    };
    pub use rand::prelude::*;
    pub use ruint2::{const_for, nlimbs, uint, Bits, Uint, UintTryFrom, UintTryTo};
    pub use std::hint::black_box;
}

fn main() {
    let mut c = criterion::Criterion::default().configure_from_args();
    benches::group(&mut c);
    c.final_summary();
}
