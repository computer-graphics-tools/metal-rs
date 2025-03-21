//! NSData - An object that wraps a byte buffer.

use std::fmt;
use std::slice;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object};

/// A reference to an Objective-C `NSData`.
pub enum NSDataRef {}

/// An owned Objective-C `NSData`.
pub struct NSData(*mut Object);

crate::foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSData;
    pub struct NSDataRef;
    type ParentType = super::object::NSObjectRef;
}

unsafe impl objc::Message for NSDataRef {}

impl NSData {
    /// Creates a new empty NSData.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSData);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            NSData::from_ptr(obj)
        }
    }

    /// Creates an NSData with the given bytes.
    #[must_use]
    pub fn with_bytes(bytes: &[u8]) -> Self {
        unsafe {
            let class = class!(NSData);
            let obj: *mut Object = msg_send![class, dataWithBytes:bytes.as_ptr() 
                                                        length:bytes.len()];
            NSData::from_ptr(obj)
        }
    }

    /// Returns the length of the data.
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), length]
        }
    }

    /// Returns a pointer to the data.
    #[must_use]
    pub fn bytes(&self) -> *const u8 {
        unsafe {
            msg_send![self.as_ref(), bytes]
        }
    }

    /// Returns the data as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.bytes(), self.length())
        }
    }
}

impl fmt::Debug for NSData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSData")
            .field("length", &self.length())
            .finish()
    }
}

impl AsRef<NSDataRef> for NSData {
    fn as_ref(&self) -> &NSDataRef {
        unsafe { &*(self.0.cast::<NSDataRef>()) }
    }
}

impl std::default::Default for NSData {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&[u8]> for NSData {
    fn from(bytes: &[u8]) -> Self {
        Self::with_bytes(bytes)
    }
}

impl Drop for NSData {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for NSData {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            NSData::from_ptr(obj)
        }
    }
}