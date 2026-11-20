#[repr(u64)]
pub(crate) enum Magic {
    Cathedral = 0x4B4154454452414Cu64,
    Nat = 0x4349424F5249554Du64,
    Liturgy = 0x004C495455524759u64,
}

impl TryFrom<u64> for Magic {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0x4B4154454452414Cu64 => Ok(Magic::Cathedral),
            0x4349424F5249554Du64 => Ok(Magic::Nat),
            0x004C495455524759u64 => Ok(Magic::Liturgy),
            _ => Err(()),
        }
    }
}

pub(crate) type SendFn = dyn FnMut(&[u8], Magic);

pub(crate) struct CallbackState {
    send_fn: Box<SendFn>,
}

impl CallbackState {
    pub(crate) fn new(send_fn: Box<SendFn>) -> Self {
        Self { send_fn }
    }
}

pub(crate) unsafe extern "C" fn send_trampoline(
    packet: *mut libkyrka_sys::kyrka_packet,
    magic: u64,
    udata: *mut std::ffi::c_void,
) {
    if packet.is_null() {
        return;
    }

    let packet = unsafe { &*packet };
    let state = unsafe { &mut *(udata as *mut CallbackState) };

    if packet.length > packet.data.len() {
        return;
    }

    let magic = Magic::try_from(magic).ok();

    if let Some(magic) = magic {
        (state.send_fn)(&packet.data[..packet.length], magic);
    }
}
