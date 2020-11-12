// use bindgen;
// // use cmake;
// use std::env;
// use std::path::PathBuf;

// fn main() {
//     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

//     println!("cargo:rustc-link-search=native=/usr/lib");
//     println!("cargo:rustc-link-lib=static=int2");
//     println!("cargo:rerun-if-changed=wrapper.h");
//     let bindings_libint2 = bindgen::Builder::default()
//         .header("wrapper.h")
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         .generate()
//         .expect("Unable to generate bindings");
//     bindings_libint2
//         .write_to_file(out_dir.join("bindings_libint2.rs"))
//         .expect("Couldn't write bindings!");
// }

fn main() {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=static=int2");
    cxx_build::bridge("src/lib.rs")
        .file("src/libint2.cpp")
        .flag_if_supported("-std=c++14")
        .include("/usr/include/eigen3")
        .compile("libint2");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/libint2.cpp");
    println!("cargo:rerun-if-changed=include/wrapper.h");
}
