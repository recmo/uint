#![cfg(any(test, feature = "bench"))]

/// Repeat a block with different values for a constant.
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
