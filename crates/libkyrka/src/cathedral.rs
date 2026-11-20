use crate::packet::send_trampoline;
use std::os::unix::ffi::OsStrExt;
use std::{ffi::CString, path::PathBuf};

/// Configuration structure for cathedrals
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CathedralConfig {
    /// Path to the KEK file.
    pub kek: Option<PathBuf>,
    /// Path to the COSK file.
    pub cosk: Option<PathBuf>,
    /// Path to the secret file.
    pub secret: Option<PathBuf>,
    /// flock source identifier.
    ///
    /// 0 means unspecified/default.
    pub flock_src: u64,
    /// flock destination identifier.
    ///
    /// required and must be nonzero
    pub flock_dst: u64,
    /// amount of commixtion hops between cathedrals, 0 means disabled, 1-3 hops allowed. Shroud must be enabled if hops are greater than 0.
    ///
    /// Must be between 0 and 3 inclusive.
    pub hops: u8,
    /// liturgy group identifier to identify peers with the same flock and group.
    pub group: u16,
    /// An id for the tunnel endpoint.
    ///
    /// In two-peer cathedral mode, the IDs pair the two endpoints—for example, 0x0102 on one side and 0x0201 on the other. In liturgy mode, each peer uses its own endpoint ID, such as 0x01.
    /// Numeric identifier, `0` may be valid
    pub tunnel: u16,
    /// a 32-bit cathedral identity. Incoming cathedral messages are checked against this identity.
    ///
    /// Must be nonzero.
    pub identity: u32,
    /// `hidden` controls liturgy discovery visibility.
    pub hidden: bool,
    /// enable / disable remembrance
    pub remembrance: bool,
}

pub(crate) struct FfiCathedralCfg {
    raw: libkyrka_sys::kyrka_cathedral_cfg,
    _kek: Option<CString>,
    _cosk: Option<CString>,
    _secret: Option<CString>,
}

impl FfiCathedralCfg {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut libkyrka_sys::kyrka_cathedral_cfg {
        &mut self.raw
    }
}

impl CathedralConfig {
    pub(crate) fn to_ffi(
        &self,
        udata: *mut std::ffi::c_void,
    ) -> Result<FfiCathedralCfg, std::ffi::NulError> {
        let kek = self
            .kek
            .as_ref()
            .map(|path| CString::new(path.as_os_str().as_bytes()))
            .transpose()?;
        let cosk = self
            .cosk
            .as_ref()
            .map(|path| CString::new(path.as_os_str().as_bytes()))
            .transpose()?;
        let secret = self
            .secret
            .as_ref()
            .map(|path| CString::new(path.as_os_str().as_bytes()))
            .transpose()?;
        let raw = libkyrka_sys::kyrka_cathedral_cfg {
            kek: kek.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            cosk: cosk.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            secret: secret.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            flock_src: self.flock_src,
            flock_dst: self.flock_dst,
            hops: self.hops,
            group: self.group,
            tunnel: self.tunnel,
            identity: self.identity,
            hidden: i32::from(self.hidden),
            remembrance: i32::from(self.remembrance),
            udata,
            send: Some(send_trampoline),
        };

        Ok(FfiCathedralCfg {
            raw,
            _kek: kek,
            _cosk: cosk,
            _secret: secret,
        })
    }
}
