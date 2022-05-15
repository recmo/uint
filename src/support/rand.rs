// TODO: Implement the Uniform distribution.

#![cfg(feature = "rand")]
use crate::{nlimbs, Uint};
use rand::{
    distributions::{Distribution, Standard, Uniform},
    Rng,
};

impl<const BITS: usize> Distribution<Uint<BITS>> for Standard
where
    [(); nlimbs(BITS)]:,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Uint<BITS> {
        let mut limbs = [0; nlimbs(BITS)];
        if let Some((last, rest)) = limbs.split_last_mut() {
            for limb in rest.iter_mut() {
                *limb = rng.gen();
            }
            *last = Uniform::new_inclusive(0, Uint::<BITS>::MASK).sample(rng);
        }
        Uint::<BITS>::from_limbs(limbs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::const_for;

    #[test]
    fn test_rand() {
        let mut rng = rand::thread_rng();
        const_for!(BITS in SIZES {
            for _ in 0..1000 {
                let _: Uint<BITS> = rng.gen();
            }
        });
    }
}
