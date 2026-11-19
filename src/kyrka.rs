use std::ptr::NonNull;
use std::ffi::c_void;
use std::error::Error;
use crate::ffi;
use crate::error::KyrkaError;

pub(crate) type SendFn = dyn FnMut(&[u8], u64) + Send;
pub struct Kyrka {
    raw: NonNull<ffi::KYRKA>,
    send_fn: Option<Box<SendFn>>,
}

impl Kyrka {
    /// Allocate a new KYRKA context that will represent a single sanctum tunnel.
    pub fn new() -> Result<Kyrka, Box<dyn Error>> {
        let ctx = unsafe { ffi::kyrka_ctx_alloc(None, std::ptr::null_mut()) };
        if ctx.is_null() {
            return Err("Failed to allocate KYRKA context".into());
        }
        Ok(Kyrka {
            raw: NonNull::new(ctx).ok_or_else(|| Box::new(KyrkaError::from_code(1)))?,
            send_fn: None,
        })
    }

    /// Obtain the last error that in the KYRKA context.
    pub fn last_error(&mut self) -> KyrkaError {
        KyrkaError::from_code(unsafe { ffi::kyrka_last_error(self.raw.as_ptr()) })
    }

    /// Enable shroud on a context, cannot be turned off once enabled.
    pub fn enable_shroud(&mut self) -> Result<(), KyrkaError> {
        let ret = unsafe { ffi::kyrka_shroud_enable(self.raw.as_ptr()) as i32 };
        if ret != 0 {
            return Err(self.last_error()?);
        }
        Ok(())
    }

    /// Set the shared secret directly by copying in the given secret.
    /// 
    /// This is not required for cathedral setups as the shared secret comes
    /// from the cathedral in ambry form.
    pub fn secret_load(&mut self, secret: [u8; 32]) -> Result<(), KyrkaError> {
        let ret = unsafe { ffi::kyrka_secret_load(self.raw.as_ptr(), secret.as_ptr() as *const c_void, secret.len()) as i32 };
        if ret != 0 {
            self.last_error()?;
        }
        Ok(())
    }

    /// Sets the cathedral secret directly by copying in the given secret.
    /// 
    /// Only call this if you did not specify the secret in the `KyrkaCathedralCfg`
    /// data structure when calling `kyrka_cathedral_config()` or did not call
    /// `kyrka_vicar_load()`.
    pub fn cathedral_secret_load(&mut self, secret: [u8; 32]) -> Result<(), KyrkaError> {
        let ret = unsafe { ffi::kyrka_cathedral_secret_load(self.raw.as_ptr(), secret.as_ptr() as *const c_void, secret.len()) as i32 };
        if ret != 0 {
            self.last_error()?;
        }
        Ok(())
    }

    /// Initialize a new `Kyrka` context that will represent a single sanctum tunnel.
    /// 
    /// `shroud` boolean indicating whether to enable shroud on the context.
    pub fn init(shroud: bool) -> Result<Kyrka, Box<dyn Error>> {
        let mut kyrka = Self::new()?;
        if shroud {
            kyrka.enable_shroud()?;
        }
        Ok(kyrka)
    }
}

impl Drop for Kyrka {
    /// Zeroizes and frees the `Kyrka` context.
    fn drop(&mut self) {
        unsafe { ffi::kyrka_ctx_free(self.raw.as_ptr()) }
    }
}