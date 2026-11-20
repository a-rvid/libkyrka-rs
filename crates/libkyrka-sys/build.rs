use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set")).join("bindings.rs");

    if std::env::var_os("DOCS_RS").is_some() {
        std::fs::copy("binds_example.rs", &out).unwrap();
        return;
    }

    let library = pkg_config::Config::new()
        .probe("libkyrka")
        .unwrap_or_else(|error| {
            panic!(
                "Failed to find libkyrka via pkg-config: {error}\n\n\
             Build and install the native libkyrka library first:\n\
             git clone https://github.com/jorisvink/libkyrka.git\n\
             cd libkyrka\n\
             make\n\
             sudo make install\n\n\
             Then ensure pkg-config can find libkyrka.pc:\n\
             export PKG_CONFIG_PATH=\"/usr/local/lib/pkgconfig:${{PKG_CONFIG_PATH:-}}\"\n\
             pkg-config --modversion libkyrka\n\n\
             If libkyrka was installed elsewhere, add that installation's\n\
             lib/pkgconfig directory to PKG_CONFIG_PATH."
            );
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
        .unwrap_or_else(|| {
            panic!(
                "pkg-config found libkyrka, but the libkyrka.h header was not found.\n\n\
                 Reinstall libkyrka with its development headers:\n\
                 git clone https://github.com/jorisvink/libkyrka.git\n\
                 cd libkyrka\n\
                 make\n\
                 sudo make install\n\n\
                 If libkyrka was installed outside /usr/local, add both:\n\
                 export PKG_CONFIG_PATH=\"/your/prefix/lib/pkgconfig:${{PKG_CONFIG_PATH:-}}\"\n\
                 export BINDGEN_EXTRA_CLANG_ARGS=\"-I/your/prefix/include\"\n\n\
                 Verify the pkg-config package:\n\
                 pkg-config --cflags --libs libkyrka"
            )
        });

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
