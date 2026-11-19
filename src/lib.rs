#![warn(missing_docs)]

//! Safe Rust bindings for libkyrka.
//! 
//! libkyrka is a library based implementation of the sanctum VPN protocol.
//! 
//! [github:jorisvink/sanctum](https://github.com/jorisvink/libkyrka)
//! IMPORTANT: Work in progress, API may change without notice, and features will be added 

#[cfg(not(any(target_os = "linux", target_os = "openbsd")))]
compile_error!("Unsupported kernel. Only Linux and OpenBSD are supported by libkyrka.");

use std::fmt;
use std::ffi::{CStr, c_void};
use std::path::PathBuf;
use std::ptr::NonNull;
use std::error::Error;
#[allow(dead_code, non_camel_case_types, non_upper_case_globals)]
pub(crate) mod ffi;
pub mod error;
pub mod cathedral;
pub mod kyrka;
pub mod prelude;
use error::KyrkaError;

/// Returns the libkyrka its version string, including the build date and
/// the revision it was compiled from.
pub fn kyrka_version() -> String {
    unsafe {
        CStr::from_ptr(ffi::kyrka_version())
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
    unsafe { ffi::kyrka_emergency_erase() }
}