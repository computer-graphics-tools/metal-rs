//! NSObject - The root class of most Objective-C class hierarchies.

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::{Object, Class};

/// A reference to an Objective-C `NSObject`.
pub enum NSObjectRef {}

/// An owned Objective-C `NSObject`.
pub struct NSObject(*mut Object);

crate::foreign_obj_type! {
    type CType = Object;
    pub struct NSObject;
    pub struct NSObjectRef;
    type ParentType = NSObjectRef;
}

unsafe impl objc::Message for NSObjectRef {}

impl NSObject {
    /// Creates a new `NSObject`.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let cls = Class::get("NSObject").unwrap();
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            NSObject::from_ptr(obj)
        }
    }

    /// Returns the class name of the object.
    #[must_use]
    pub fn class_name(&self) -> &str {
        unsafe {
            let cls: *const Class = msg_send![self.as_ref(), class];
            let name = (*cls).name();
            std::str::from_utf8_unchecked(name)
        }
    }
}

impl fmt::Debug for NSObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSObject")
            .field("class", &self.class_name())
            .finish()
    }
}

impl AsRef<NSObjectRef> for NSObject {
    fn as_ref(&self) -> &NSObjectRef {
        unsafe { &*(self.0.cast::<NSObjectRef>()) }
    }
}

impl std::default::Default for NSObject {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for NSObject {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for NSObject {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            NSObject::from_ptr(obj)
        }
    }
}