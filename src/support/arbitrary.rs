#![cfg(feature = "arbitrary")]
use crate::{nlimbs, Uint};
use arbitrary::{Arbitrary, Result, Unstructured};

impl<'a, const BITS: usize> Arbitrary<'a> for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let mut limbs = [0; nlimbs(BITS)];
        if let Some((last, rest)) = limbs.split_last_mut() {
            for limb in rest.iter_mut() {
                *limb = u64::arbitrary(u)?;
            }
            *last = u.int_in_range(0..=Self::MASK)?;
        }
        Ok(Self::from_limbs(limbs))
    }

    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        let bytes = (BITS + 7) / 8;
        (bytes, Some(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repeat;
    use std::iter::repeat;

    #[test]
    fn test_arbitrary() {
        repeat!({
            let (num_bytes, _) = Uint::<N>::size_hint(0);
            let bytes = repeat(0x55u8).take(num_bytes).collect::<Vec<_>>();
            let mut u = arbitrary::Unstructured::new(&bytes);
            let x = Uint::<N>::arbitrary(&mut u).unwrap();
            dbg!(x);
        });
    }
}
