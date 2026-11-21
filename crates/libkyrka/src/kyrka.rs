use crate::cathedral::CathedralConfig;
use crate::constants::*;
use crate::error::KyrkaError::{self};
use crate::error::KyrkaNativeError;
use crate::packet::CallbackState;
use std::ffi::c_void;
#[cfg(feature = "config")]
use std::path::PathBuf;
use std::ptr::NonNull;

/// A structure containing all data needed to run the application.
pub struct Kyrka {
    ctx: NonNull<libkyrka_sys::KYRKA>,
    callback_state: Option<Box<CallbackState>>,
}

/// Configuration for a KYRKA context.
///
/// Use `KyrkaConfigBuilder` to construct an instance of this configuration.
/// Alternatively you can load the configuration directly into a `Kyrka` context using `Kyrka::load()`.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KyrkaConfig {
    /// Wether to shroud the transmitted data or not
    ///
    /// Shrouding is a method to obfuscate the transmitted data, providing additional privacy.
    shroud: bool,
    /// Maximum transmission unit variable. Recommended: `1422` (bytes)
    mtu: u16,
    #[cfg_attr(feature = "serde", serde(skip))]
    /// Set the shared secret directly by copying in the given secret.
    ///
    /// This is not required for cathedral setups as the shared secret comes
    /// from the cathedral in ambry form.
    secret: Option<[u8; 32]>,
    #[cfg(feature = "config")]
    secret_path: Option<PathBuf>,
    /// Cathedral configuration for the context.
    cathedral_config: Option<crate::cathedral::CathedralConfig>,
}

/// Builder for `KyrkaConfig` to facilitate configuration setup.
impl KyrkaConfig {
    /// Enable or disable shroud
    pub fn shroud(mut self, shroud: bool) -> Self {
        self.shroud = shroud;
        self
    }

    /// Set the maximum transmission unit.
    pub fn mtu(mut self, mtu: u16) -> Self {
        self.mtu = mtu;
        self
    }

    /// Set the shared secret directly by copying in the given secret.
    pub fn secret(mut self, secret: [u8; 32]) -> Self {
        self.secret = Some(secret);
        self
    }

    /// Set the cathedral configuration for the builder.
    pub fn cathedral_config(mut self, config: CathedralConfig) -> Self {
        self.cathedral_config = Some(config);
        self
    }

    /// Validate and obtain the `KyrkaConfig`.
    ///
    /// Returns an error if required fields are not set.
    pub fn validate(self) -> Result<Self, KyrkaError> {
        if self.mtu == 0 {
            return Err(KyrkaError::MtuRequired);
        }
        if self.mtu >= libkyrka_sys::KYRKA_PACKET_DATA_LEN as u16 {
            return Err(KyrkaError::InvalidMtuSize(self.mtu));
        }
        Ok(self)
    }

    /// Return a `Kyrka` context based on the configuration.
    pub fn load(self) -> Result<Kyrka, KyrkaError> {
        Kyrka::load(self.validate()?)
    }
}

impl Kyrka {
    fn ensure_active(&self) -> Result<(), KyrkaError> {
        if crate::EMERGENCY_ERASED.load(std::sync::atomic::Ordering::SeqCst) {
            Err(KyrkaError::ErasedContext)
        } else {
            Ok(())
        }
    }

    /// Obtain a builder for `KyrkaConfig` to construct configuration. Must set MTU.
    pub fn builder() -> KyrkaConfig {
        KyrkaConfig::default()
    }

    /// Allocate a new KYRKA context that will represent a single sanctum tunnel.
    pub fn new() -> Result<Kyrka, KyrkaError> {
        if crate::EMERGENCY_ERASED.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(KyrkaError::ErasedContext);
        }
        let ctx = unsafe { libkyrka_sys::kyrka_ctx_alloc(None, std::ptr::null_mut()) };
        Ok(Kyrka {
            ctx: NonNull::new(ctx).ok_or_else(|| {
                KyrkaError::AllocFailed("Failed to allocate KYRKA context".into())
            })?,
            callback_state: None,
        })
    }

    /// Process pending events for a thread associated with this context. You must call this every event loop.
    pub fn event(&mut self) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe { libkyrka_sys::kyrka_key_manage(self.ctx.as_ptr()) };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Obtain the last error that in the KYRKA context.
    pub fn last_error(&mut self) -> KyrkaError {
        if crate::EMERGENCY_ERASED.load(std::sync::atomic::Ordering::SeqCst) {
            return KyrkaError::ErasedContext;
        }
        KyrkaNativeError::from_code(unsafe { libkyrka_sys::kyrka_last_error(self.ctx.as_ptr()) })
            .into()
    }

    /// The application thinks the peer timed out, so we erase all keys
    ///
    /// send an event back if we did so.
    /// IMPORTANT: Not implemented.
    pub fn peer_timeout(&mut self) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe { libkyrka_sys::kyrka_peer_timeout(self.ctx.as_ptr()) };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Lets the context know if we are in an active p2p tunnel or if
    /// we are perhaps being relayed.
    ///
    /// This is important when using shroud
    /// so internally generated offers can be shrouded with the right key.
    ///
    /// If you get this wrong, at worst the receiving party will not be able
    /// to unshroud the packet, but no meta-data is leaked.
    ///
    /// Do not use this function if you are using commixtion as commixtion
    /// requires your packet to travel *through* the cathedrals.
    pub fn p2p_active(&mut self, active: bool) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe { libkyrka_sys::kyrka_p2p_active(self.ctx.as_ptr(), active as i32) };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Sets the device KEK secret directly by copying in the given secret.
    ///
    /// Only call this if you did not specify the kek in the `CathedralConfig`.
    pub fn device_kek_load(&mut self, kek: [u8; KYRKA_KEY_LENGTH]) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe {
            libkyrka_sys::kyrka_device_kek_load(
                self.ctx.as_ptr(),
                kek.as_ptr() as *const c_void,
                kek.len(),
            )
        };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Sets the cathedral offer signing key directly by copying in the given key.
    ///
    /// Only call this if you did not specify the signing key in the `CathedralConfig`.
    pub fn cathedral_cosk_load(
        &mut self,
        key: [u8; KYRKA_ED25519_SIGN_SECRET_LENGTH],
    ) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe {
            libkyrka_sys::kyrka_cathedral_cosk_load(
                self.ctx.as_ptr(),
                key.as_ptr() as *const c_void,
                key.len(),
            )
        };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Configure the cathedral for the context.
    ///
    /// This sets up the cathedral with the provided configuration.
    pub fn cathedral_config(&mut self, config: CathedralConfig) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        config.validate()?;
        if self.callback_state.is_none() {
            self.callback_state = Some(Box::new(CallbackState::new(Box::new(|_data, _magic| {}))));
        }
        let callback_state = self.callback_state.as_mut().unwrap();
        let udata = (&mut **callback_state) as *mut CallbackState as *mut c_void;
        let mut ffi = config.to_ffi(udata)?;
        let ret =
            unsafe { libkyrka_sys::kyrka_cathedral_config(self.ctx.as_ptr(), ffi.as_mut_ptr()) };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Enable shroud on a context, cannot be turned off once enabled.
    pub fn enable_shroud(&mut self) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe { libkyrka_sys::kyrka_shroud_enable(self.ctx.as_ptr()) };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Set the Maximum Transmission Unit (MTU) size for the context.
    ///
    /// This defines the largest packet size that can be transmitted over the tunnel.
    pub fn set_mtu(&mut self, mtu: u16) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe { libkyrka_sys::kyrka_mtu_size(self.ctx.as_ptr(), mtu) };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Set the shared secret directly by copying in the given secret.
    ///
    /// This is not required for cathedral setups as the shared secret comes
    /// from the cathedral in ambry form.
    pub fn secret_load(&mut self, secret: [u8; 32]) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe {
            libkyrka_sys::kyrka_secret_load(
                self.ctx.as_ptr(),
                secret.as_ptr() as *const c_void,
                secret.len(),
            )
        };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Sets the cathedral secret directly by copying in the given secret.
    ///
    /// Only call this if you did not specify the secret in the `KyrkaCathedralCfg`
    /// data structure when calling `kyrka_cathedral_config()` or did not call
    /// `kyrka_vicar_load()`.
    pub fn cathedral_secret_load(&mut self, secret: [u8; 32]) -> Result<(), KyrkaError> {
        self.ensure_active()?;
        let ret = unsafe {
            libkyrka_sys::kyrka_cathedral_secret_load(
                self.ctx.as_ptr(),
                secret.as_ptr() as *const c_void,
                secret.len(),
            )
        };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Load a KYRKA context and configure it based on the provided `KyrkaConfig`.
    pub fn load(config: KyrkaConfig) -> Result<Self, KyrkaError> {
        let mut ctx = Self::new()?;
        ctx.set_mtu(config.mtu)?;
        if config.shroud {
            ctx.enable_shroud()?;
        }
        if let Some(secret) = config.secret {
            ctx.secret_load(secret)?;
        } else {
            #[cfg(feature = "config")]
            if let Some(secret_path) = config.secret_path {
                let secret = std::fs::read(secret_path)?;
                ctx.secret_load(
                    secret
                        .as_slice()
                        .try_into()
                        .map_err(|_| KyrkaError::InvalidSecretLength(secret.len()))?,
                )?;
            }
        }
        if let Some(cathedral_config) = config.cathedral_config {
            ctx.cathedral_config(cathedral_config)?;
        }
        Ok(ctx)
    }
}

impl Drop for Kyrka {
    /// Zeroizes and frees the `Kyrka` context.
    fn drop(&mut self) {
        if !crate::EMERGENCY_ERASED.load(std::sync::atomic::Ordering::SeqCst) {
            unsafe { libkyrka_sys::kyrka_ctx_free(self.ctx.as_ptr()) }
        }
    }
}
