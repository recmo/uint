use bytemuck::Pod;
use ruint::Uint;

macro_rules! impl_pod {
    ($($bits:expr),+ $(,)?) => {
        $(
            unsafe impl<const LIMBS: usize> bytemuck::Zeroable for Uint<{$bits}, LIMBS> {}
            unsafe impl<const LIMBS: usize> Pod for Uint<{$bits}, LIMBS> where
                [(); {$bits % 64}]: ,
                [(); LIMBS]: ,
            {
            }
        )+
    };
}

impl_pod! {
    64, 128, 192, 256, 320, 384, 448, 512, 576, 640, 704, 768, 832, 896, 960, 1024,
}

#[cfg(test)]
mod tests {
    use bytemuck::{Pod, Zeroable};
    use ruint::Uint;

    #[test]
    fn test_uint_pod() {
        test_pod::<64, 1>();
        test_pod::<128, 2>();
        test_pod::<192, 3>();
        test_pod::<256, 4>();
        test_pod::<320, 5>();
        test_pod::<384, 6>();
        test_pod::<448, 7>();
        test_pod::<512, 8>();
        test_pod::<576, 9>();
        test_pod::<640, 10>();
        test_pod::<704, 11>();
        test_pod::<768, 12>();
        test_pod::<832, 13>();
        test_pod::<896, 14>();
        test_pod::<960, 15>();
        test_pod::<1024, 16>();
    }

    fn test_pod<const BITS: usize, const LIMBS: usize>()
    where
        Uint<{ BITS }, { LIMBS }>: Zeroable + Pod + Eq + Default,
    {
        let val = Uint::<{ BITS }, { LIMBS }>::default();
        let bytes = bytemuck::bytes_of(&val);

        assert_eq!(
            bytes.len(),
            std::mem::size_of::<Uint<{ BITS }, { LIMBS }>>()
        );

        let zeroed_val: Uint<{ BITS }, { LIMBS }> = Zeroable::zeroed();
        assert_eq!(zeroed_val, Uint::<{ BITS }, { LIMBS }>::default());
    }
}
