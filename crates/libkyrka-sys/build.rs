use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set")).join("bindings.rs");

    if std::env::var_os("DOCS_RS").is_some() {
        std::fs::copy("binds_example.rs", &out).unwrap();
        return;
    }

    let library = pkg_config::Config::new().probe("libkyrka").unwrap_or_else(|error| {
        panic!("Failed to find libkyrka via pkg-config: {}\n\n
        
        Try setting the PKG_CONFIG_PATH environment variable to the directory containing libkyrka.pc (e.g. /usr/local/lib/pkgconfig).\n
        If you haven't compiled libkyrka yet, you can do so by looking at https://github.com/a-rvid/libkyrka-rs/tree/main/crates/libkyrka-sys/",
        error);
    });
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

    let mut builder = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .allowlist_function("kyrka_.*")
        .allowlist_type("kyrka.*|KYRKA")
        .allowlist_var("KYRKA_.*")
        .generate_comments(true);

    for path in &library.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate libkyrka bindings");

    bindings
        .write_to_file(&out)
        .expect("Failed to write bindings");
}
