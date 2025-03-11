//! MTLFence - A Rust wrapper around Metal's MTLFence class.
//!
//! This module provides safe Rust bindings to the MTLFence class from Apple's Metal framework.
//! MTLFence represents a synchronization point for cross-encoder dependencies to ensure
//! that resources used by multiple encoders are properly synchronized.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCommandQueue};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue
//! let command_queue = device.new_command_queue();
//! 
//! // Create a command buffer
//! let command_buffer = command_queue.new_command_buffer();
//! 
//! // Create a fence for synchronization
//! let fence = device.new_fence();
//! 
//! // Create encoders and use the fence
//! let blit_encoder = command_buffer.new_blit_command_encoder();
//! blit_encoder.update_fence(&fence);
//! blit_encoder.end_encoding();
//! 
//! let render_encoder = command_buffer.new_render_command_encoder_with_descriptor(&render_pass_descriptor);
//! render_encoder.wait_for_fence(&fence);
//! // Render operations...
//! render_encoder.end_encoding();
//! 
//! command_buffer.commit();
//! ```

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use std::fmt;
use crate::foundation::NSString;
use crate::metal::device::MTLDeviceRef;

/// A reference to an Objective-C `MTLFence`.
pub struct MTLFenceRef(Object);

/// An owned Objective-C `MTLFence`.
pub struct MTLFence(*mut Object);

unsafe impl ForeignTypeRef for MTLFenceRef {
    type CType = Object;
}

unsafe impl Send for MTLFenceRef {}
unsafe impl Sync for MTLFenceRef {}

unsafe impl ForeignType for MTLFence {
    type CType = Object;
    type Ref = MTLFenceRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFence {
        MTLFence(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFenceRef> for MTLFence {
    fn as_ref(&self) -> &MTLFenceRef {
        unsafe { &*(self.0.cast::<MTLFenceRef>()) }
    }
}

unsafe impl Send for MTLFence {}
unsafe impl Sync for MTLFence {}

unsafe impl objc::Message for MTLFenceRef {}

impl MTLFence {
    /// Returns the device that created this fence.
    #[must_use]
    pub fn device(&self) -> &MTLDeviceRef {
        unsafe {
            let device: *mut Object = msg_send![self.as_ref(), device];
            &*(device as *mut MTLDeviceRef)
        }
    }

    /// Returns the label for this fence.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), label];
            if ns_string.is_null() {
                return String::new();
            }
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }

    /// Sets the label for this fence.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            msg_send![self.as_ref(), setLabel:ns_string.as_ptr()]
        }
    }
}

impl fmt::Debug for MTLFence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLFence {{ label: \"{}\" }}", self.label())
    }
}

impl Drop for MTLFence {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFence {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFence::from_ptr(obj)
        }
    }
}