// Repeat a block with different values substituted for `N`.
#[macro_export]
macro_rules! repeat {
    ( non_zero, $x:block ) => {
        repeat!($x, 1, 2, 63, 64, 65, 127,128,129,256,384,512,4096);
    };
    ( $x:block ) => {
        repeat!($x, 0, 1, 2, 63, 64, 65, 127,128,129,256,384,512,4096);
    };
    ( $x:block, $( $n:literal ),* ) => {
        $({
            const N: usize = $n;
            $x
        })*
    };
}
