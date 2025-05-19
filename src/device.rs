//! Wrapper for `id<MTLDevice>` – the root object of the Metal API.
//!
//! The underlying Objective-C entity is *not* a class but a **protocol**.
//! Apple exposes a C helper `MTLCreateSystemDefaultDevice()` to obtain a
//! concrete implementation.  We mirror that approach here.

use crate::prelude::*;
use objc2::encode::{RefEncode, Encoding};
use objc2::Message;

unsafe extern "C" {
    /// Equivalent to Apple's `MTLCreateSystemDefaultDevice()` C function.
    fn MTLCreateSystemDefaultDevice() -> *mut AnyObject;
}

// Protocol definition -------------------------------------------------------
extern_protocol!(pub unsafe trait DeviceProtocol: NSObjectProtocol {
    // no methods added yet – we use message-send directly
});

/// Thin wrapper over `id<MTLDevice>` with lifetime & ownership tracking.
/// Implemented as a `#[repr(transparent)]` new-type so we can add inherent
/// methods.
#[repr(transparent)]
pub struct Device(ProtocolObject<dyn DeviceProtocol>);

// SAFETY: Transparent wrapper around a valid Objective-C object; forwards encoding.
unsafe impl Message for Device {}
unsafe impl RefEncode for Device {
    const ENCODING_REF: Encoding = <ProtocolObject<dyn DeviceProtocol>>::ENCODING_REF;
}

// Allow transparent deref to underlying `ProtocolObject` so that it can be
// used in `msg_send!` safely.
impl core::ops::Deref for Device {
    type Target = ProtocolObject<dyn DeviceProtocol>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Device {
    /// Returns the system-default GPU (if any).
    pub fn system_default() -> Option<Retained<Self>> {
        unsafe {
            let raw_ptr = MTLCreateSystemDefaultDevice();
            if raw_ptr.is_null() {
                return None;
            }
            let po_ptr = raw_ptr as *mut ProtocolObject<dyn DeviceProtocol>;
            let option_po: Option<Retained<ProtocolObject<dyn DeviceProtocol>>> = Retained::retain_autoreleased(po_ptr);
            option_po.map(|p| Retained::cast_unchecked::<Device>(p))
        }
    }

    /// Human-readable name (e.g. "Apple M3", "iPhone GPU").
    pub fn name(&self) -> String {
        unsafe {
            let ns_string: *mut NSString = msg_send![self, name];
            Retained::retain_autoreleased(ns_string)
                .map(|s| s.to_string())
                .unwrap_or_default()
        }
    }

    /// Creates a new [`CommandQueue`] from this device.
    pub fn new_command_queue(&self) -> Retained<crate::command_queue::CommandQueue> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::command_queue::CommandQueueProtocol> = msg_send![self, newCommandQueue];
            let po_ret: Retained<ProtocolObject<dyn crate::command_queue::CommandQueueProtocol>> = Retained::retain_autoreleased(ptr).expect("newCommandQueue returned nil");
            Retained::cast_unchecked::<crate::command_queue::CommandQueue>(po_ret)
        }
    }

    /// Allocates a Metal buffer of `length` bytes with the given options.
    pub fn new_buffer_with_length(
        &self,
        length: usize,
        options: crate::buffer::ResourceOptions,
    ) -> Retained<crate::buffer::Buffer> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::buffer::BufferProtocol> = msg_send![
                self,
                newBufferWithLength: length as u64,
                options: options.bits()
            ];
            let po_ret: Retained<ProtocolObject<dyn crate::buffer::BufferProtocol>> = Retained::retain_autoreleased(ptr).expect("newBufferWithLength returned nil");
            Retained::cast_unchecked::<crate::buffer::Buffer>(po_ret)
        }
    }
}

// ---------------------------------------------------------------------------
// (No enums/flags yet – they will be added alongside their first call-sites)
// --------------------------------------------------------------------------- 