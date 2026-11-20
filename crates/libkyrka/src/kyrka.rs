use crate::cathedral::CathedralConfig;
use crate::error::KyrkaError;
use crate::packet::CallbackState;
use std::error::Error;
use std::ffi::c_void;
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

    /// Build the `KyrkaConfig` from the builder.
    ///
    /// Returns an error if required fields are not set.
    pub fn build(self) -> Result<KyrkaConfig, Box<dyn Error>> {
        if self.mtu == 0 {
            return Err("MTU must be set. Use `mtu()` on the builder to set it.".into());
        }
        Ok(KyrkaConfig {
            shroud: self.shroud,
            mtu: self.mtu,
            secret: self.secret,
            cathedral_config: self.cathedral_config,
        })
    }

    /// Return a `Kyrka` context based on the builder configuration.
    pub fn load(self) -> Result<Kyrka, Box<dyn Error>> {
        Kyrka::load(self.build()?)
    }
}

impl Kyrka {
    /// Obtain a builder for `KyrkaConfig` to construct configuration. Must set MTU.
    pub fn builder() -> KyrkaConfig {
        KyrkaConfig::default()
    }

    /// Allocate a new KYRKA context that will represent a single sanctum tunnel.
    pub fn new() -> Result<Kyrka, Box<dyn Error>> {
        let ctx = unsafe { libkyrka_sys::kyrka_ctx_alloc(None, std::ptr::null_mut()) };
        if ctx.is_null() {
            return Err("Failed to allocate KYRKA context".into());
        }
        Ok(Kyrka {
            ctx: NonNull::new(ctx).ok_or_else(|| Box::new(KyrkaError::System))?,
            callback_state: None,
        })
    }

    /// Process pending events for a thread associated with this context. You must call this every event loop.
    pub fn event(&mut self) -> Result<(), KyrkaError> {
        let ret = unsafe { libkyrka_sys::kyrka_key_manage(self.ctx.as_ptr()) as i32 };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Obtain the last error that in the KYRKA context.
    pub fn last_error(&mut self) -> KyrkaError {
        KyrkaError::from_code(unsafe { libkyrka_sys::kyrka_last_error(self.ctx.as_ptr()) })
    }

    /// Configure the cathedral for the context.
    ///
    /// This sets up the cathedral with the provided configuration.
    pub fn cathedral_config(&mut self, config: CathedralConfig) -> Result<(), Box<dyn Error>> {
        if self.callback_state.is_none() {
            self.callback_state = Some(Box::new(CallbackState::new(Box::new(|_data, _magic| {}))));
        }
        let callback_state = self.callback_state.as_mut().unwrap();
        let udata = (&mut **callback_state) as *mut CallbackState as *mut c_void;
        let mut ffi = config.to_ffi(udata)?;
        let ret = unsafe { libkyrka_sys::kyrka_cathedral_config(self.ctx.as_ptr(), ffi.as_mut_ptr()) as i32 };
        if ret != 0 {
            return Err(self.last_error().into());
        }
        Ok(())
    }

    /// Enable shroud on a context, cannot be turned off once enabled.
    pub fn enable_shroud(&mut self) -> Result<(), KyrkaError> {
        let ret = unsafe { libkyrka_sys::kyrka_shroud_enable(self.ctx.as_ptr()) as i32 };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Set the Maximum Transmission Unit (MTU) size for the context.
    ///
    /// This defines the largest packet size that can be transmitted over the tunnel.
    pub fn set_mtu(&mut self, mtu: u16) -> Result<(), KyrkaError> {
        let ret = unsafe { libkyrka_sys::kyrka_mtu_size(self.ctx.as_ptr(), mtu) as i32 };
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
        let ret = unsafe {
            libkyrka_sys::kyrka_secret_load(
                self.ctx.as_ptr(),
                secret.as_ptr() as *const c_void,
                secret.len(),
            ) as i32
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
        let ret = unsafe {
            libkyrka_sys::kyrka_cathedral_secret_load(
                self.ctx.as_ptr(),
                secret.as_ptr() as *const c_void,
                secret.len(),
            ) as i32
        };
        if ret != 0 {
            return Err(self.last_error());
        }
        Ok(())
    }

    /// Load a KYRKA context and configure it based on the provided `KyrkaConfig`.
    pub fn load(config: KyrkaConfig) -> Result<Self, Box<dyn Error>> {
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
                ctx.secret_load(
                    std::fs::read(secret_path)?
                        .try_into()
                        .map_err(|_| "Invalid secret length")?,
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
        unsafe { libkyrka_sys::kyrka_ctx_free(self.ctx.as_ptr()) }
    }
}
