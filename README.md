# libkyrka-rs

Rust bindings for [libkyrka](https://github.com/jorisvink/libkyrka/). quote:
> libkyrka implements the sanctum protocol in library form allowing you to establish p2p e2ee tunnels directly from your application.

Note that libkyrka is not thread-safe and you may not run different KYRKA contexts in different threads. It was designed to be run on an event loop using single-threaded i/o multiplexing.

To use libkyrka-rs you need to install some dependencies:

## Dependencies

* You need `libsodium-dev` headers, or one of the other cryptography backends (advanced), see [github:jorisvink/sanctum](https://github.com/jorisvink/sanctum/blob/master/docs/building.md).
* A C compiler + make 
* pkg-config

`libkyrka`:

```bash
# sudo apt install build-essential pkg-config libsodium-dev # for Debian
git clone https://github.com/jorisvink/libkyrka/
cd libkyrka
make
sudo make install
```

(in project):
```bash
cargo add libkyrka
```

To compile you may need to add /usr/local/share to PKG_CONFIG_PATH:
```bash
export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"
```

## Features

* [serde](https://docs.rs/crate/serde/latest): 
  allows for serialization of data structures that might be used for configuration.

  Implements Serialize and Deserialize for some structs.

[Documentation](https://docs.rs/libkyrka)