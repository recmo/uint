use rustc_version::{version_meta, Channel};

fn main() {
    let has_generic_const_exprs = version_meta().unwrap().channel == Channel::Nightly;
    if has_generic_const_exprs {
        println!("cargo:rustc-cfg=has_generic_const_exprs");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
