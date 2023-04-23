extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    // Run cmake to build nng
    let dst = Config::new("libiwasm")
        .env("CC", "clang")
        .env("CXX", "clang++")
        .generator("Unix Makefiles")
        .no_build_target(true)
        .build();
    // Check output of `cargo build --verbose`, should see something like:
    // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
    // That contains output from cmake
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build").display()
    );
    //println!("cargo:rustc-link-lib=iwasm");
    println!("cargo:rustc-link-lib=vmlib_untrusted");
    //println!("cargo:rustc-link-search=/home/nskernel/linux-sgx/linux/installer/bin/sgxsdk/sdk_libs");
    println!("cargo:rustc-link-lib=sgx_urts");
    println!("cargo:rustc-link-lib=sgx_uae_service");
    println!("cargo:rustc-link-lib=sgx_dcap_ql");
    println!("cargo:rustc-link-lib=sgx_dcap_quoteverify");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=pthread");
    

    let bindings = bindgen::Builder::default()
        .header("wasm-micro-runtime/core/iwasm/include/wasm_sgx_export.h")
        // This is needed if use `#include <nng.h>` instead of `#include "path/nng.h"`
        //.clang_arg("-Inng/src/")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
