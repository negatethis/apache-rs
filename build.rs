extern crate bindgen;

use std::{env};
use std::path::PathBuf;

fn main() {

    // apache2 and apr header files used to build bindings
    let apache_headers = pkg_config::probe_library("apache2").unwrap();
    let apr_headers = pkg_config::probe_library("apr-1").unwrap();

    // TODO: fix dependency
    // Remove specific header files because of known issues.
    // header_files.retain(|header_file| !header_file.eq("mod_xml2enc.h"));

    // Tell cargo to invalidate the built crate whenever the wrapper changes.
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder
    // Derived from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let bindings = bindgen::Builder::default()

        // The includes for C header files.
        .clang_args(apache_headers.include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())))
        .clang_args(apr_headers.include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())))

        // The input header we would like to generate bindings for.
        .header("wrapper.h")

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))

        // Pretty print the bindings.
        .rustfmt_bindings(true)

        // Finish the builder and generate the bindings.
        .generate()

        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
