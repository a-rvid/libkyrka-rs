use std::fmt;

pub(crate) fn check(code: u32) -> Result<(), KyrkaError> {
    if code == 0 {
        Ok(())
    } else {
        Err(KyrkaError::from_code(code))
    }
}

/// Errors that can occur within a `Kyrka` context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KyrkaError {
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

impl fmt::Display for KyrkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KyrkaError::None => write!(f, "No error occurred."),
            KyrkaError::System => write!(f, "A system error occurred."),
            KyrkaError::Parameter => write!(f, "A parameter error occurred."),
            KyrkaError::NoContext => write!(f, "No context error."),
            KyrkaError::NoCallback => write!(f, "No callback error occurred."),
            KyrkaError::FileError => write!(f, "A file related error occurred."),
            KyrkaError::Internal => write!(f, "An internal error occurred."),
            KyrkaError::Integrity => write!(f, "An integrity error occurred."),
            KyrkaError::NoTxKey => write!(f, "No TX key."),
            KyrkaError::NoRxKey => write!(f, "No RX key."),
            KyrkaError::NoKek => write!(f, "No KEK Secret."),
            KyrkaError::NoSecret => write!(f, "No secret error."),
            KyrkaError::NoConfig => write!(f, "No config error."),
            KyrkaError::CathedralConfig => write!(f, "Cathedral config error."),
            KyrkaError::PacketError => write!(f, "Packet error."),
            KyrkaError::NoCosk => write!(f, "No COSK error."),
            KyrkaError::Timeout => write!(f, "Timeout error."),
            KyrkaError::ShroudNoKeys => write!(f, "Shroud no keys error."),
            KyrkaError::MtuSize => write!(f, "MTU size error."),
            KyrkaError::Unknown(code) => write!(f, "An unknown error. Code: {}", code),
        }
    }
}

impl std::error::Error for KyrkaError {}

impl KyrkaError {
    pub(crate) fn from_code(code: u32) -> Self {
        match code {
            libkyrka_sys::KYRKA_ERROR_NONE => KyrkaError::None,
            libkyrka_sys::KYRKA_ERROR_SYSTEM => KyrkaError::System,
            libkyrka_sys::KYRKA_ERROR_PARAMETER => KyrkaError::Parameter,
            libkyrka_sys::KYRKA_ERROR_NO_CONTEXT => KyrkaError::NoContext,
            libkyrka_sys::KYRKA_ERROR_NO_CALLBACK => KyrkaError::NoCallback,
            libkyrka_sys::KYRKA_ERROR_FILE_ERROR => KyrkaError::FileError,
            libkyrka_sys::KYRKA_ERROR_INTERNAL => KyrkaError::Internal,
            libkyrka_sys::KYRKA_ERROR_INTEGRITY => KyrkaError::Integrity,
            libkyrka_sys::KYRKA_ERROR_NO_TX_KEY => KyrkaError::NoTxKey,
            libkyrka_sys::KYRKA_ERROR_NO_RX_KEY => KyrkaError::NoRxKey,
            libkyrka_sys::KYRKA_ERROR_NO_KEK => KyrkaError::NoKek,
            libkyrka_sys::KYRKA_ERROR_NO_SECRET => KyrkaError::NoSecret,
            libkyrka_sys::KYRKA_ERROR_NO_CONFIG => KyrkaError::NoConfig,
            libkyrka_sys::KYRKA_ERROR_CATHEDRAL_CONFIG => KyrkaError::CathedralConfig,
            libkyrka_sys::KYRKA_ERROR_PACKET_ERROR => KyrkaError::PacketError,
            libkyrka_sys::KYRKA_ERROR_NO_COSK => KyrkaError::NoCosk,
            libkyrka_sys::KYRKA_ERROR_TIMEOUT => KyrkaError::Timeout,
            libkyrka_sys::KYRKA_ERROR_SHROUD_NO_KEYS => KyrkaError::ShroudNoKeys,
            libkyrka_sys::KYRKA_ERROR_MTU_SIZE => KyrkaError::MtuSize,
            _ => KyrkaError::Unknown(code),
        }
    }
}
