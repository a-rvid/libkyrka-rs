use std::fmt;
use std::io;
use std::ffi::NulError;

/// Errors that can occur near the `Kyrka` structure.
#[derive(Debug)]
pub enum KyrkaError {
    /// Native errors returned by the underlying C KYRKA library.
    Native(KyrkaNativeError),
    /// I/O operation errors.
    Io(io::Error),
    /// `std::ffi::NulError` errors.
    NulError(NulError),
    /// Invalid secret length.
    InvalidSecretLength(usize),
    /// Invalid MTU size (max: `libkyrka_sys::KYRKA_PACKET_DATA_LEN`)
    InvalidMtuSize(u16),
    /// Context has been erased due to an emergency erase operation.
    ErasedContext,
    /// Memory allocation failed.
    AllocFailed(String),
    /// MTU is required but not set.
    MtuRequired,
}

impl From<KyrkaNativeError> for KyrkaError {
    fn from(err: KyrkaNativeError) -> Self {
        KyrkaError::Native(err)
    }
}

impl From<io::Error> for KyrkaError {
    fn from(err: io::Error) -> Self {
        KyrkaError::Io(err)
    }
}

impl From<NulError> for KyrkaError {
    fn from(err: NulError) -> Self {
        KyrkaError::NulError(err)
    }
}

impl fmt::Display for KyrkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KyrkaError::Native(err) => write!(f, "Native error: {}", err),
            KyrkaError::Io(err) => write!(f, "IO error: {}", err),
            KyrkaError::InvalidSecretLength(len) => write!(f, "Invalid secret length: {}", len),
            KyrkaError::InvalidMtuSize(mtu) => write!(f, "Invalid MTU size (maximum {}): {}", libkyrka_sys::KYRKA_PACKET_DATA_LEN, mtu),
            KyrkaError::ErasedContext => write!(f, "Context has been erased."),
            KyrkaError::AllocFailed(msg) => write!(f, "Allocation failed: {}", msg),
            KyrkaError::MtuRequired => write!(f, "MTU is required."),
            KyrkaError::NulError(err) => write!(f, "Nul error: {}", err),
        }
    }
}

impl std::error::Error for KyrkaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            KyrkaError::Native(error) => Some(error),
            KyrkaError::Io(error) => Some(error),
            KyrkaError::NulError(error) => Some(error),
            _ => None,
        }
    }
}

impl PartialEq for KyrkaError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Native(a), Self::Native(b)) => a == b,
            (Self::Io(a), Self::Io(b)) => {
                a.kind() == b.kind()
                    && a.raw_os_error() == b.raw_os_error()
                    && a.to_string() == b.to_string()
            }
            (Self::NulError(a), Self::NulError(b)) => a == b,
            (Self::InvalidSecretLength(a), Self::InvalidSecretLength(b)) => a == b,
            (Self::InvalidMtuSize(a), Self::InvalidMtuSize(b)) => a == b,
            (Self::ErasedContext, Self::ErasedContext) => true,
            (Self::AllocFailed(a), Self::AllocFailed(b)) => a == b,
            (Self::MtuRequired, Self::MtuRequired) => true,
            _ => false,
        }
    }
}

impl Eq for KyrkaError {}

/// Errors from the underlying C KYRKA library.
#[derive(Debug, Eq, PartialEq)]
pub enum KyrkaNativeError {
    /// KYRKA_ERROR_NONE
    None,
    /// KYRKA_ERROR_SYSTEM
    System,
    /// KYRKA_ERROR_PARAMETER
    Parameter,
    /// KYRKA_ERROR_NO_CONTEXT
    NoContext,
    /// KYRKA_ERROR_NO_CALLBACK
    NoCallback,
    /// KYRKA_ERROR_FILE_ERROR
    FileError,
    /// KYRKA_ERROR_INTERNAL
    Internal,
    /// KYRKA_ERROR_INTEGRITY
    Integrity,
    /// KYRKA_ERROR_NO_TX_KEY
    NoTxKey,
    /// KYRKA_ERROR_NO_RX_KEY
    NoRxKey,
    /// KYRKA_ERROR_NO_KEK
    NoKek,
    /// KYRKA_ERROR_NO_SECRET
    NoSecret,
    /// KYRKA_ERROR_NO_CONFIG
    NoConfig,
    /// KYRKA_ERROR_CATHEDRAL_CONFIG
    CathedralConfig,
    /// KYRKA_ERROR_PACKET_ERROR
    PacketError,
    /// KYRKA_ERROR_NO_COSK
    NoCosk,
    /// KYRKA_ERROR_TIMEOUT
    Timeout,
    /// KYRKA_ERROR_SHROUD_NO_KEYS
    ShroudNoKeys,
    /// KYRKA_ERROR_MTU_SIZE
    MtuSize,

    /// An unknown error code.
    Unknown(u32),
}

impl fmt::Display for KyrkaNativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KyrkaNativeError::None => write!(f, "No error occurred."),
            KyrkaNativeError::System => write!(f, "A system error occurred."),
            KyrkaNativeError::Parameter => write!(f, "A parameter error occurred."),
            KyrkaNativeError::NoContext => write!(f, "No context error."),
            KyrkaNativeError::NoCallback => write!(f, "No callback error occurred."),
            KyrkaNativeError::FileError => write!(f, "A file related error occurred."),
            KyrkaNativeError::Internal => write!(f, "An internal error occurred."),
            KyrkaNativeError::Integrity => write!(f, "An integrity error occurred."),
            KyrkaNativeError::NoTxKey => write!(f, "No TX key."),
            KyrkaNativeError::NoRxKey => write!(f, "No RX key."),
            KyrkaNativeError::NoKek => write!(f, "No KEK Secret."),
            KyrkaNativeError::NoSecret => write!(f, "No secret error."),
            KyrkaNativeError::NoConfig => write!(f, "No config error."),
            KyrkaNativeError::CathedralConfig => write!(f, "Cathedral config error."),
            KyrkaNativeError::PacketError => write!(f, "Packet error."),
            KyrkaNativeError::NoCosk => write!(f, "No COSK error."),
            KyrkaNativeError::Timeout => write!(f, "Timeout error."),
            KyrkaNativeError::ShroudNoKeys => write!(f, "Shroud no keys error."),
            KyrkaNativeError::MtuSize => write!(f, "MTU size error."),
            KyrkaNativeError::Unknown(code) => write!(f, "An unknown error. Code: {}", code),
        }
    }
}

impl std::error::Error for KyrkaNativeError {}

impl KyrkaNativeError {
    pub(crate) fn from_code(code: u32) -> Self {
        match code {
            libkyrka_sys::KYRKA_ERROR_NONE => KyrkaNativeError::None,
            libkyrka_sys::KYRKA_ERROR_SYSTEM => KyrkaNativeError::System,
            libkyrka_sys::KYRKA_ERROR_PARAMETER => KyrkaNativeError::Parameter,
            libkyrka_sys::KYRKA_ERROR_NO_CONTEXT => KyrkaNativeError::NoContext,
            libkyrka_sys::KYRKA_ERROR_NO_CALLBACK => KyrkaNativeError::NoCallback,
            libkyrka_sys::KYRKA_ERROR_FILE_ERROR => KyrkaNativeError::FileError,
            libkyrka_sys::KYRKA_ERROR_INTERNAL => KyrkaNativeError::Internal,
            libkyrka_sys::KYRKA_ERROR_INTEGRITY => KyrkaNativeError::Integrity,
            libkyrka_sys::KYRKA_ERROR_NO_TX_KEY => KyrkaNativeError::NoTxKey,
            libkyrka_sys::KYRKA_ERROR_NO_RX_KEY => KyrkaNativeError::NoRxKey,
            libkyrka_sys::KYRKA_ERROR_NO_KEK => KyrkaNativeError::NoKek,
            libkyrka_sys::KYRKA_ERROR_NO_SECRET => KyrkaNativeError::NoSecret,
            libkyrka_sys::KYRKA_ERROR_NO_CONFIG => KyrkaNativeError::NoConfig,
            libkyrka_sys::KYRKA_ERROR_CATHEDRAL_CONFIG => KyrkaNativeError::CathedralConfig,
            libkyrka_sys::KYRKA_ERROR_PACKET_ERROR => KyrkaNativeError::PacketError,
            libkyrka_sys::KYRKA_ERROR_NO_COSK => KyrkaNativeError::NoCosk,
            libkyrka_sys::KYRKA_ERROR_TIMEOUT => KyrkaNativeError::Timeout,
            libkyrka_sys::KYRKA_ERROR_SHROUD_NO_KEYS => KyrkaNativeError::ShroudNoKeys,
            libkyrka_sys::KYRKA_ERROR_MTU_SIZE => KyrkaNativeError::MtuSize,
            _ => KyrkaNativeError::Unknown(code),
        }
    }
}
