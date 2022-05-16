//! Feature detection for `generic_const_exprs`. This exists mainly to avoid
//! compilation failures when using `--all-features` on stable.

use autocfg::AutoCfg;

fn main() {
    let has_generic_const_exprs = AutoCfg::new()
        .unwrap()
        .probe_features(&["generic_const_exprs"], "");
    if has_generic_const_exprs {
        println!("cargo:rustc-cfg=has_generic_const_exprs");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
