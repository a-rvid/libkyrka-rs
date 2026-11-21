include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        let version: *const std::os::raw::c_char = unsafe { crate::kyrka_version() };
        let version_str = unsafe { std::ffi::CStr::from_ptr(version) }
            .to_str()
            .expect("Failed to convert kyrka_version to &str");
        assert!(
            !version_str.is_empty(),
            "kyrka_version returned an empty string"
        );
    }
}
