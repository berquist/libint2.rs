use bindgen;
use cxx_build::CFG;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=static=int2");

    // Binding to parts of the C++ API.
    //
    CFG.exported_header_dirs
        .push(Path::new("/usr/include/eigen3"));
    cxx_build::bridge("src/lib.rs")
        .file("src/libint2_c.cpp")
        .file("src/libint2_wrapper.cpp")
        .flag_if_supported("-std=c++14")
        .compile("libint2");

    println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rerun-if-changed=include/libint2_c.h");
    println!("cargo:rerun-if-changed=include/libint2_wrapper.hpp");
    println!("cargo:rerun-if-changed=src/libint2_c.cpp");
    println!("cargo:rerun-if-changed=src/libint2_wrapper.cpp");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Bindgen just does not work with the C++ API, probably because of all
    // the template specialization.

    // Automatic binding to the C API.
    let bindings_libint2 = bindgen::Builder::default()
        .header("/usr/include/libint2.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(true)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");
    bindings_libint2
        .write_to_file(out_dir.join("bindings_libint2.rs"))
        .expect("Couldn't write bindings!");
}
