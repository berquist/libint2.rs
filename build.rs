use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=static=int2");

    // Binding to parts of the C++ API.
    //
    cxx_build::bridge("src/lib.rs")
        .file("src/libint2_c.cpp")
        .file("src/libint2_wrapper.cpp")
        .flag_if_supported("-std=c++14")
        .include("/usr/include/eigen3")
        .compile("libint2");

    // println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rerun-if-changed=include/libint2_c.h");
    println!("cargo:rerun-if-changed=include/libint2_wrapper.hpp");
    println!("cargo:rerun-if-changed=src/libint2_c.cpp");
    println!("cargo:rerun-if-changed=src/libint2_wrapper.cpp");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Bindgen just does not work with the C++ API, probably because of all
    // the template specialization.
    //
    // let bindings_libint2 = bindgen::Builder::default()
    //     .header("/usr/include/libint2.hpp")
    //     .header("/usr/include/libint2/boys.h")
    //     .clang_arg("-xc++")
    //     .clang_arg("-std=c++14")
    //     .clang_arg("-I/usr/include/eigen3")
    //     // .opaque_type("std::.*")
    //     // .opaque_type("Eigen::.*")
    //     .whitelist_type("libint2::.*")
    //     // .whitelist_type("libint2::Atom")
    //     // .whitelist_type("libint2::chemistry::.*")
    //     // .whitelist_type("libint2::constants::.*")
    //     // .blacklist_type("std::.*")
    //     // .blacklist_type("boost::.*")
    //     // .blacklist_item("basic_ostream.*")
    //     // .blacklist_item("basic_istream.*")
    //     .enable_cxx_namespaces()
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate_comments(true)
    //     .generate()
    //     .expect("Unable to generate bindings");

    // Automatic binding to the C API.
    let bindings_libint2 = bindgen::Builder::default()
        .header("/usr/include/libint2.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");
    bindings_libint2
        .write_to_file(out_dir.join("bindings_libint2.rs"))
        .expect("Couldn't write bindings!");
}
