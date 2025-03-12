//! MTLFunctionHandle - A Rust wrapper around Metal's function handle API.
//!
//! This module provides Rust bindings to the MTLFunctionHandle class from Apple's Metal framework.
//! MTLFunctionHandle is a lightweight reference to a shader function that can be used in places where you need to refer to functions
//! without the overhead of the full MTLFunction object.
//!
//! # Note
//!
//! This implementation is currently a placeholder for future Metal API support. The MTLFunctionHandle
//! functionality may not be directly available through the public Metal API in current versions.
//! This implementation will be updated when the API becomes available or when the correct creation
//! method is identified.
//!
//! To use function references in Metal, please use MTLFunction objects directly.

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::MTLFunctionType;

/// A reference to an Objective-C `MTLFunctionHandle`.
pub struct MTLFunctionHandleRef(Object);

/// An owned Objective-C `MTLFunctionHandle`.
pub struct MTLFunctionHandle(*mut Object);

// Implementation for MTLFunctionHandle
unsafe impl ForeignTypeRef for MTLFunctionHandleRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionHandleRef {}
unsafe impl Sync for MTLFunctionHandleRef {}

unsafe impl ForeignType for MTLFunctionHandle {
    type CType = Object;
    type Ref = MTLFunctionHandleRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionHandle {
        MTLFunctionHandle(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionHandleRef> for MTLFunctionHandle {
    fn as_ref(&self) -> &MTLFunctionHandleRef {
        unsafe { &*(self.0.cast::<MTLFunctionHandleRef>()) }
    }
}

unsafe impl Send for MTLFunctionHandle {}
unsafe impl Sync for MTLFunctionHandle {}

unsafe impl objc::Message for MTLFunctionHandleRef {}

impl MTLFunctionHandle {
    /// Returns the type of the function.
    #[must_use]
    pub fn function_type(&self) -> MTLFunctionType {
        unsafe {
            let function_type: u64 = msg_send![self.as_ref(), functionType];
            std::mem::transmute(function_type)
        }
    }
    
    /// Returns the name of the function.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), name];
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }
    
    /// Returns the device that created this function handle.
    #[must_use]
    pub fn device(&self) -> crate::metal::MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            crate::metal::MTLDevice::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLFunctionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionHandle")
            .field("name", &self.name())
            .field("function_type", &self.function_type())
            .finish()
    }
}

impl Drop for MTLFunctionHandle {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionHandle {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionHandle::from_ptr(obj)
        }
    }
}