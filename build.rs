use rustc_version::{version_meta, Channel};

fn main() {
    // HACK: Ideally we would test for features instead of release channel.
    let nightly = version_meta().unwrap().channel == Channel::Nightly;
    if nightly {
        println!("cargo:rustc-cfg=has_generic_const_exprs");
        println!("cargo:rustc-cfg=has_cfg_doc");
        println!("cargo:rustc-cfg=has_core_intrinsics");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
