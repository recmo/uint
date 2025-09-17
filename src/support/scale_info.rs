//! Support for the [`sale-info`](https://crates.io/crates/scale-info) crate.

#![cfg(feature = "scale-info")]
#![cfg_attr(docsrs, doc(cfg(feature = "scale-info")))]

use crate::Uint;
use scale_info::{Type, TypeInfo};

impl<const BITS: usize, const LIMBS: usize> TypeInfo for Uint<BITS, LIMBS> {
    type Identity = Self;
    fn type_info() -> ::scale_info::Type {
        Type::builder()
            .path(::scale_info::Path::new_with_replace(
                "Uint",
                ::core::module_path!(),
                &[],
            ))
            .composite(
                scale_info::build::Fields::unnamed()
                    .field(|f| f.ty::<[u64; LIMBS]>().type_name("[u64; LIMBS]")),
            )
    }
}

#[test]
fn typeinfo_works() {
    use crate::aliases::U256;
    use scale_info::TypeInfo;
    let info = &<U256 as TypeInfo>::type_info();
    assert_eq!(info.path.segments.last().unwrap().to_string(), "Uint");
}
