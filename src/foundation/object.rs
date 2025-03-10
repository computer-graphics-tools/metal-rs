//! NSObject - the root class of most Objective-C class hierarchies.

use super::string::{NSString, NSStringRef};
use super::types::NSUInteger;
use crate::foreign_obj_type;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL};
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::msg_send_bool;

/// A reference to an Objective-C `NSObject`.
pub enum NSObjectRef {}

/// An owned Objective-C `NSObject`.
pub struct NSObject(foreign_types::ForeignObjectRef<NSObjectRef>);

foreign_types::foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSObject;
    pub struct NSObjectRef;
}

unsafe impl objc::Message for NSObjectRef {}

impl NSObject {
    /// Creates a new object by calling `alloc` and `init` on the class.
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSObject);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            NSObject::from_raw(obj)
        }
    }
    
    /// Creates an NSObject from a raw pointer.
    pub fn from_ptr(ptr: *mut Object) -> Self {
        unsafe { Self::from_raw(ptr) }
    }

    /// Returns the receiver's class.
    pub fn class(&self) -> *const Class {
        unsafe { msg_send![self.as_ref(), class] }
    }

    /// Returns the class name of the receiver.
    pub fn class_name(&self) -> String {
        unsafe {
            let name: *const objc::runtime::Object = msg_send![self.class(), name];
            let nsstring = NSString::from_ptr(name as *mut _);
            nsstring.to_string()
        }
    }

    /// Returns a Boolean value that indicates whether the receiver is an instance of a given class or an instance of any class that inherits from that class.
    pub fn is_kind_of_class(&self, class: &Class) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.as_ref(), isKindOfClass: class];
            result == YES
        }
    }

    /// Returns a Boolean value that indicates whether the receiver is an instance of the specified class.
    pub fn is_member_of_class(&self, class: &Class) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.as_ref(), isMemberOfClass: class];
            result == YES
        }
    }

    /// Returns a Boolean value that indicates whether the receiver responds to a selector.
    pub fn responds_to_selector(&self, selector: objc::runtime::Sel) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.as_ref(), respondsToSelector: selector];
            result == YES
        }
    }

    /// Returns a Boolean value that indicates whether the receiver conforms to a given protocol.
    pub fn conforms_to_protocol(&self, protocol: &objc::runtime::Protocol) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.as_ref(), conformsToProtocol: protocol];
            result == YES
        }
    }

    /// Returns the hash value of the receiver.
    pub fn hash_value(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), hash]
        }
    }

    /// Returns a Boolean value that indicates whether the receiver and a given object are equal.
    pub fn is_equal(&self, other: &NSObjectRef) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.as_ref(), isEqual: other];
            result == YES
        }
    }

    /// Returns a description of the receiver.
    pub fn description(&self) -> NSString {
        unsafe {
            let description: *mut Object = msg_send![self.as_ref(), description];
            NSString::from_ptr(description)
        }
    }

    /// Returns a detailed description of the receiver.
    pub fn debug_description(&self) -> NSString {
        unsafe {
            let description: *mut Object = msg_send![self.as_ref(), debugDescription];
            NSString::from_ptr(description)
        }
    }
}

impl Default for NSObject {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let description: *mut Object = msg_send![self.as_ref(), description];
            let nsstring = NSString::from_ptr(description);
            write!(f, "{}", nsstring.to_string())
        }
    }
}

impl Hash for NSObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_value().hash(state);
    }
}

impl PartialEq for NSObject {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.as_ref())
    }
}

impl Eq for NSObject {}