//! NSString - A Rust wrapper around Objective-C's NSString class.
//!
//! This module provides safe Rust bindings to the NSString class from Apple's Foundation framework.
//! It implements proper memory management and conversion between Rust and Objective-C strings.
//!
//! # Examples
//!
//! ```
//! use metal_rs::foundation::NSString;
//!
//! // Create a string from a Rust str
//! let hello = NSString::from_rust_str("Hello, Foundation!");
//!
//! // Access string properties
//! assert_eq!(hello.len(), 18);
//! assert_eq!(hello.as_str(), "Hello, Foundation!");
//! assert!(!hello.is_empty());
//!
//! // Convert back to a Rust String
//! let rust_string = hello.to_rust_string();
//! assert_eq!(rust_string, "Hello, Foundation!");
//!
//! // Can also use FromStr and Display traits
//! use std::str::FromStr;
//! let hello2 = NSString::from_str("Hello, FromStr!").unwrap();
//! println!("{}", hello2); // uses Display
//! ```

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    // This is empty, but links with the Foundation framework
}

use std::fmt;
use std::os::raw::c_char;
use std::str;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};

/// Objective-C string encoding options.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum NSStringEncoding {
    /// UTF-8 encoding.
    UTF8 = 4,
}

/// A reference to an Objective-C `NSString`.
pub struct NSStringRef(Object);

/// An owned Objective-C `NSString`.
pub struct NSString(*mut Object);

unsafe impl ForeignTypeRef for NSStringRef {
    type CType = Object;
}

unsafe impl Send for NSStringRef {}
unsafe impl Sync for NSStringRef {}

unsafe impl ForeignType for NSString {
    type CType = Object;
    type Ref = NSStringRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> NSString {
        NSString(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<NSStringRef> for NSString {
    fn as_ref(&self) -> &NSStringRef {
        unsafe { &*(self.0.cast::<NSStringRef>()) }
    }
}

unsafe impl Send for NSString {}
unsafe impl Sync for NSString {}

unsafe impl objc::Message for NSStringRef {}

impl NSString {
    /// Creates an empty string.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSString);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            NSString::from_ptr(obj)
        }
    }

    /// Creates a string from a Rust string.
    #[must_use]
    pub fn from_rust_str(s: &str) -> Self {
        nsstring_from_str(s)
    }

    /// Returns the UTF-8 representation of the string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe {
            let bytes: *const c_char = msg_send![self.as_ref(), UTF8String];
            let len: usize = msg_send![self.as_ref(), lengthOfBytesUsingEncoding: NSStringEncoding::UTF8 as usize];
            let bytes = bytes.cast::<u8>();
            let bytes = std::slice::from_raw_parts(bytes, len);
            std::str::from_utf8_unchecked(bytes)
        }
    }

    /// Returns the length of the string.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), length]
        }
    }

    /// Returns whether the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the string as a Rust String.
    #[must_use]
    pub fn to_rust_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for NSString {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            NSString::from_ptr(obj)
        }
    }
}

impl Default for NSString {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NSString(\"{}\")", self.as_str())
    }
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for NSString {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(nsstring_from_str(s))
    }
}

/// Creates an `NSString` from a Rust string.
#[must_use]
pub fn nsstring_from_str(string: &str) -> NSString {
    unsafe {
        let cls = class!(NSString);
        let bytes = string.as_ptr().cast::<c_char>();
        let len = string.len();
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, initWithBytes: bytes 
                                        length: len 
                                        encoding: NSStringEncoding::UTF8 as usize];
        NSString::from_ptr(obj)
    }
}

/// Returns a borrowed Rust string from an `NSString`.
#[must_use]
pub fn nsstring_as_str(nsstr: &NSString) -> &str {
    nsstr.as_str()
}