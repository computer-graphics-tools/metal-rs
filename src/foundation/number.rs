// Copyright 2024 The metal-rs contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use std::fmt;

/// A reference to an Objective-C `NSNumber`.
pub struct NSNumberRef(Object);

/// An owned Objective-C `NSNumber`.
pub struct NSNumber(*mut Object);

unsafe impl ForeignTypeRef for NSNumberRef {
    type CType = Object;
}

unsafe impl Send for NSNumberRef {}
unsafe impl Sync for NSNumberRef {}

unsafe impl ForeignType for NSNumber {
    type CType = Object;
    type Ref = NSNumberRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> NSNumber {
        NSNumber(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<NSNumberRef> for NSNumber {
    fn as_ref(&self) -> &NSNumberRef {
        unsafe { &*(self.0.cast::<NSNumberRef>()) }
    }
}

unsafe impl Send for NSNumber {}
unsafe impl Sync for NSNumber {}

unsafe impl objc::Message for NSNumberRef {}

impl NSNumber {
    /// Creates a new NSNumber with a boolean value.
    pub fn from_bool(value: bool) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![class!(NSNumber), numberWithBool:value];
            NSNumber::from_ptr(obj)
        }
    }

    /// Creates a new NSNumber with an integer value.
    pub fn from_integer(value: i32) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![class!(NSNumber), numberWithInt:value];
            NSNumber::from_ptr(obj)
        }
    }

    /// Creates a new NSNumber with a unsigned integer value.
    pub fn from_unsigned_integer(value: u32) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![class!(NSNumber), numberWithUnsignedInt:value];
            NSNumber::from_ptr(obj)
        }
    }

    /// Creates a new NSNumber with a float value.
    pub fn from_float(value: f32) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![class!(NSNumber), numberWithFloat:value];
            NSNumber::from_ptr(obj)
        }
    }

    /// Creates a new NSNumber with a double value.
    pub fn from_double(value: f64) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![class!(NSNumber), numberWithDouble:value];
            NSNumber::from_ptr(obj)
        }
    }
}

impl NSNumberRef {
    /// Gets the boolean value of this number.
    #[must_use]
    pub fn bool_value(&self) -> bool {
        unsafe { msg_send![self, boolValue] }
    }

    /// Gets the integer value of this number.
    #[must_use]
    pub fn int_value(&self) -> i32 {
        unsafe { msg_send![self, intValue] }
    }

    /// Gets the unsigned integer value of this number.
    #[must_use]
    pub fn unsigned_int_value(&self) -> u32 {
        unsafe { msg_send![self, unsignedIntValue] }
    }

    /// Gets the float value of this number.
    #[must_use]
    pub fn float_value(&self) -> f32 {
        unsafe { msg_send![self, floatValue] }
    }

    /// Gets the double value of this number.
    #[must_use]
    pub fn double_value(&self) -> f64 {
        unsafe { msg_send![self, doubleValue] }
    }
}

impl Clone for NSNumber {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            NSNumber::from_ptr(obj)
        }
    }
}

impl Drop for NSNumber {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NSNumber({:?})", self.as_ref().float_value())
    }
}