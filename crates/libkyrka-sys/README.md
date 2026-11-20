**THIS PROJECT IS WORK IN PROGRESS AT THIS COMMIT AND SHOULD NOT BE USED AS 0.1.x!!**

Raw FFI bindings for the jorisvink library. You are probably looking for the safe library, **[libkyrka](https://crates.io/crates/libkyrka)**.

## Building `libkyrka`

`libkyrka-sys` requires the native C library. It does not bundle or build
`libkyrka` itself.

Install the required dependencies, then build and install libkyrka:

```bash
sudo apt install build-essential pkg-config libsodium-dev
git clone https://github.com/jorisvink/libkyrka
cd libkyrka
make
sudo make install
```

You may need to add `/usr/local/lib/pkgconfig` to PKG_CONFIG_PATH:
```bash
export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig"
# to make it permanent:
# echo 'export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig"' >> "$HOME/.bashrc"