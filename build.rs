extern crate bindgen;

use std::env;
use std::path::PathBuf;

const LIB_NAME: &str = "opengrn";
const LIB_PATH: &str = "thirdparty/opengr2/build/libopengrn/";
const LIB_WRAPPER_PATH: &str = "wrapper.h";


fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    //let manifest_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={}", LIB_PATH);
    println!("cargo:rustc-link-lib={}", LIB_NAME);
    println!("cargo:rerun-if-changed={}", LIB_WRAPPER_PATH);

    let bindings = bindgen::Builder::default()
        .header(LIB_WRAPPER_PATH)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
