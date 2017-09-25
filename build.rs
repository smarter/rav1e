extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("aom_build/aom/aom_mem/aom_mem.c")
        .file("aom_build/aom/aom_dsp/entenc.c")
        .file("aom_build/aom/aom_dsp/entcode.c")
        .file("aom_build/aom/aom_dsp/fwd_txfm.c")
        .file("aom_build/aom/aom_dsp/inv_txfm.c")
        .file("aom_build/aom/aom_dsp/intrapred.c")
        .file("aom_build/aom/av1/common/odintrin.c")
        .file("aom_build/aom/av1/common/entropymode.c")
        .file("aom_build/aom/av1/common/entropy.c")
        .file("aom_build/aom/av1/common/scan.c")
        .file("aom_build/aom/av1/common/quant_common.c")
        .file("aom_build/aom/av1/common/av1_inv_txfm1d.c")
        .file("aom_build/aom/av1/common/av1_inv_txfm2d.c")
        .include("aom_build")
        .include("aom_build/aom")
        .flag("-std=c99")
        .compile("libntr.a");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Iaom_build")
        .clang_arg("-Iaom_build/aom")
        .clang_arg("-Iaom_build/aom/av1/common")
        .clang_arg("-Iaom_build/aom/av1/decoder")
        .clang_arg("-Iaom_build/aom/av1/encoder")
        // See #687
        .hide_type("FP_NAN")
        .hide_type("FP_INFINITE")
        .hide_type("FP_ZERO")
        .hide_type("FP_SUBNORMAL")
        .hide_type("FP_NORMAL")
        // See #577
        .hide_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
