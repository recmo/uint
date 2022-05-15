/// Compile time for loops with a `const` variable for testing.
///
/// Repeats a block of code with different values assigned to a constant.
///
/// ```rust
/// # #![allow(incomplete_features)]
/// # #![feature(generic_const_exprs)]
/// # use ruint::{const_for, Uint};
/// const_for!(BITS in [0, 10, 100] {
///     println!("{:?}", Uint::<BITS>::MAX);
/// });
/// ```
///
/// is equivalent to
///
/// ```rust
/// # #![allow(incomplete_features)]
/// # #![feature(generic_const_exprs)]
/// # use ruint::{const_for, Uint};
/// println!("{:?}", Uint::<0>::MAX);
/// println!("{:?}", Uint::<10>::MAX);
/// println!("{:?}", Uint::<100>::MAX);
/// ```
///
/// It comes with two build-in lists: `NON_ZERO` which is equivalent to
///
/// ```text
/// [1, 2, 63, 64, 65, 127, 128, 129, 256, 384, 512, 4096]
/// ```
///
/// and `SIZES` which is the same but also has `0` as a value.
///
/// In combination with [`proptest!`][proptest::proptest] this allows for
/// testing over a large range of [`Uint`][crate::Uint] types and values:
///
/// ```rust
/// # #![allow(incomplete_features)]
/// # #![feature(generic_const_exprs)]
/// # use proptest::prelude::*;
/// # use ruint::{const_for, Uint};
/// const_for!(BITS in SIZES {
///    proptest!(|(value: Uint<BITS>)| {
///         // ... test code
///     });
/// });
/// ```
#[macro_export]
macro_rules! const_for {
    ($C:ident in [ $( $n:literal ),* ] $x:block) => {
        $({
            const $C: usize = $n;
            $x
        })*
    };
    ($C:ident in SIZES $x:block) => {
        const_for!($C in [0] $x);
        const_for!($C in NON_ZERO $x);
    };
    ($C:ident in NON_ZERO $x:block) => {
        const_for!($C in [1, 2, 63, 64, 65, 127,128,129,256,384,512,4096] $x);
    };
}
