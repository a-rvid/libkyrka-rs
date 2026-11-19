use std::path::PathBuf;

fn main() {
    let library = pkg_config::Config::new().probe("libkyrka").expect(
        "Could not find libkyrka via pkg-config; set PKG_CONFIG_PATH to its .pc file directory",
    );

    let header = library
        .include_paths
        .iter()
        .map(|dir| dir.join("libkyrka/libkyrka.h"))
        .chain(
            library
                .include_paths
                .iter()
                .map(|dir| dir.join("libkyrka.h")),
        )
        .find(|path| path.exists())
        .expect("pkg-config found libkyrka, but its header was not found");

    let mut builder = bindgen::Builder::default().header(header.to_string_lossy());

    for path in &library.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate libkyrka bindings");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write bindings");
}
