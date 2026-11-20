#![warn(missing_docs)]

//! Safe Rust bindings for libkyrka.
//!
//! libkyrka is a library based implementation of the sanctum VPN protocol.
//!
//! [github:jorisvink/sanctum](https://github.com/jorisvink/libkyrka)
//! IMPORTANT: Work in progress, API may change without notice, and features will be added

#[cfg(not(any(target_os = "linux", target_os = "openbsd")))]
compile_error!("Unsupported kernel. Only Linux and OpenBSD are supported by libkyrka.");

use std::ffi::CStr;
/// Provides configuration forcathedrals
///
/// Cathedrals manage the connection to a server that handles peer discovery, traffic relay and key distribution
pub mod cathedral;
pub(crate) mod constants;
/// Error handling for libkyrka.
pub mod error;
/// Provides the context and all user-facing functionality for managing the application.
pub mod kyrka;
pub(crate) mod packet;
/// Prelude module for commonly used items.
///
/// use libkyrka::prelude::*;
pub mod prelude;

/// Returns the libkyrka its version string, including the build date and
/// the revision it was compiled from.
pub fn kyrka_version() -> String {
    unsafe {
        CStr::from_ptr(libkyrka_sys::kyrka_version())
            .to_string_lossy()
            .into_owned()
    }
}

/// Perform an "emergency" erase of all data allocated by all KYRKA contexts.
///
/// You cannot safely use *ANY* allocated KYRKA context after calling
/// this function. Use this with caution.
///
/// Ideally you call this from an error handler in your code when you are
/// going to exit or abort due to a panic.
pub fn kyrka_emergency_erase() {
    unsafe { libkyrka_sys::kyrka_emergency_erase() }
}
