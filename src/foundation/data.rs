//! NSData - An object representing a static byte buffer.

use crate::foreign_obj_type;
use crate::foundation::types::NSUInteger;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL};
use std::fmt;
use std::ops::Deref;
use std::os::raw::c_void;
use std::slice;

/// A reference to an Objective-C `NSData`.
pub enum NSDataRef {}

/// An owned Objective-C `NSData`.
pub struct NSData(foreign_types::ForeignObjectRef<NSDataRef>);

foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSData;
    pub struct NSDataRef;
    type ParentType = super::object::NSObjectRef;
}

unsafe impl objc::Message for NSDataRef {}

impl NSData {
    /// Creates an empty data object.
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSData);
            let obj: *mut Object = msg_send![class, data];
            NSData::from_ptr(obj)
        }
    }

    /// Creates a data object with the given bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        unsafe {
            let class = class!(NSData);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithBytes:length:, 
                                           bytes.as_ptr() as *const c_void, 
                                           bytes.len()];
            NSData::from_ptr(obj)
        }
    }

    /// Returns the length of the data in bytes.
    pub fn len(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), length]
        }
    }

    /// Returns whether the data is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a pointer to the data.
    pub fn bytes(&self) -> *const u8 {
        unsafe {
            let ptr: *const c_void = msg_send![self.as_ref(), bytes];
            ptr as *const u8
        }
    }

    /// Returns the data as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = self.bytes();
            let len = self.len();
            slice::from_raw_parts(ptr, len)
        }
    }

    /// Creates a new data object by appending the given data.
    pub fn append(&self, other: &NSDataRef) -> NSData {
        unsafe {
            let mutable_data: *mut Object = msg_send![class!(NSMutableData), dataWithData:, self.as_ref()];
            let _: () = msg_send![mutable_data, appendData:, other];
            let result: *mut Object = msg_send![mutable_data, copy];
            NSData::from_ptr(result)
        }
    }

    /// Creates a new data object by appending the given bytes.
    pub fn append_bytes(&self, bytes: &[u8]) -> NSData {
        unsafe {
            let mutable_data: *mut Object = msg_send![class!(NSMutableData), dataWithData:, self.as_ref()];
            let _: () = msg_send![mutable_data, appendBytes:length:, 
                                bytes.as_ptr() as *const c_void, 
                                bytes.len()];
            let result: *mut Object = msg_send![mutable_data, copy];
            NSData::from_ptr(result)
        }
    }

    /// Creates a new data object with a subdataset.
    pub fn subdata(&self, range: std::ops::Range<usize>) -> NSData {
        let len = self.len();
        let start = range.start.min(len);
        let end = range.end.min(len);
        
        if start >= end {
            return NSData::new();
        }
        
        let length = end - start;
        let ns_range = super::types::NSRange {
            location: start,
            length,
        };
        
        unsafe {
            let result: *mut Object = msg_send![self.as_ref(), subdataWithRange:, ns_range];
            NSData::from_ptr(result)
        }
    }

    /// Returns a base64 encoded string representation of the data.
    pub fn base64_encoded_string(&self) -> super::NSString {
        unsafe {
            let result: *mut Object = msg_send![self.as_ref(), base64EncodedStringWithOptions:, 0];
            super::NSString::from_ptr(result)
        }
    }
}

impl Default for NSData {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.as_bytes();
        write!(f, "NSData {{ len: {}, bytes: {:?} }}", bytes.len(), bytes)
    }
}

impl Clone for NSData {
    fn clone(&self) -> Self {
        unsafe {
            let copy: *mut Object = msg_send![self.as_ref(), copy];
            NSData::from_ptr(copy)
        }
    }
}

impl AsRef<[u8]> for NSData {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> From<&'a [u8]> for NSData {
    fn from(bytes: &'a [u8]) -> Self {
        NSData::from_bytes(bytes)
    }
}

impl From<Vec<u8>> for NSData {
    fn from(bytes: Vec<u8>) -> Self {
        NSData::from_bytes(&bytes)
    }
}