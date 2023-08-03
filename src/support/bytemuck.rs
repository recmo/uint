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
mod tests {}
