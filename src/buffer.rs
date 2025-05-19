//! Wrapper for `id<MTLBuffer>` – linear memory allocation on the GPU.

use crate::prelude::*;
use objc2::encode::{Encoding, RefEncode};
use objc2::Message;

extern_protocol!(pub unsafe trait BufferProtocol: NSObjectProtocol {
});

/// Thin wrapper over `id<MTLBuffer>`.
#[repr(transparent)]
pub struct Buffer(ProtocolObject<dyn BufferProtocol>);

impl core::ops::Deref for Buffer {
    type Target = ProtocolObject<dyn BufferProtocol>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

unsafe impl Message for Buffer {}
unsafe impl RefEncode for Buffer {
    const ENCODING_REF: Encoding = <ProtocolObject<dyn BufferProtocol>>::ENCODING_REF;
}

bitflags! {
    /// Subset of Metal's `MTLResourceOptions` used by buffer allocations.
    #[repr(transparent)]
    #[doc(alias = "MTLResourceOptions")]
    pub struct ResourceOptions: u64 {
        const CPU_CACHE_MODE_DEFAULT_CACHE  = 0;      // Metal default
        const STORAGE_MODE_SHARED          = 0x0100; // (1 << 8)
        const STORAGE_MODE_PRIVATE         = 0x0200; // (2 << 8)
    }
}

impl Buffer {
    /// Total size (in bytes) of the buffer.
    pub fn length(&self) -> usize {
        unsafe { msg_send![self, length] }
    }

    /// Returns a raw CPU pointer to the contents.  The pointer is only valid
    /// while the buffer is CPU-accessible (non‐private storage mode).
    pub fn contents(&self) -> *mut std::ffi::c_void {
        unsafe { msg_send![self, contents] }
    }

    /// Informs the driver that `range` bytes have been modified by the CPU.
    pub fn did_modify_range(&self, start: usize, length: usize) {
        unsafe {
            let nsrange = objc2_foundation::NSRange::new(start, length);
            let _: () = msg_send![self, didModifyRange: nsrange];
        }
    }
} 