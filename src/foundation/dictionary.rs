//! NSDictionary - A collection of key-value pairs.

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::{Object, Class};
use crate::foundation::object::NSObjectRef;

/// A reference to an Objective-C `NSDictionary`.
pub enum NSDictionaryRef {}

/// An owned Objective-C `NSDictionary`.
pub struct NSDictionary(*mut Object);

crate::foreign_obj_type! {
    type CType = Object;
    pub struct NSDictionary;
    pub struct NSDictionaryRef;
    type ParentType = NSObjectRef;
}

unsafe impl objc::Message for NSDictionaryRef {}

impl NSDictionary {
    /// Creates a new empty dictionary.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let cls = class!(NSDictionary);
            let obj: *mut Object = msg_send![cls, dictionary];
            NSDictionary::from_ptr(obj)
        }
    }

    /// Returns the number of entries in the dictionary.
    #[must_use]
    pub fn count(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), count]
        }
    }

    /// Returns whether the dictionary is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

impl fmt::Debug for NSDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSDictionary")
            .field("count", &self.count())
            .finish()
    }
}

impl AsRef<NSDictionaryRef> for NSDictionary {
    fn as_ref(&self) -> &NSDictionaryRef {
        unsafe { &*(self.0.cast::<NSDictionaryRef>()) }
    }
}

impl std::default::Default for NSDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for NSDictionary {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for NSDictionary {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            NSDictionary::from_ptr(obj)
        }
    }
}