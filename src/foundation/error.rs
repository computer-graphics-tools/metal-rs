//! NSError - An object that encapsulates information about an error.

use crate::foundation::string::{NSString, NSStringRef};
use crate::foundation::dictionary::NSDictionaryRef;
use crate::foundation::types::NSInteger;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL};
use std::fmt;

/// A reference to an Objective-C `NSError`.
pub enum NSErrorRef {}

/// An owned Objective-C `NSError`.
pub struct NSError(foreign_types::ForeignObjectRef<NSErrorRef>);

foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSError;
    pub struct NSErrorRef;
    type ParentType = super::object::NSObjectRef;
}

unsafe impl objc::Message for NSErrorRef {}

impl NSError {
    /// Creates a new error with a domain, code, and user info.
    pub fn new(domain: &str, code: NSInteger, user_info: Option<&NSDictionaryRef>) -> Self {
        let ns_domain = NSString::from_str(domain);
        let user_info_ptr = match user_info {
            Some(dict) => dict as *const NSDictionaryRef,
            None => std::ptr::null(),
        };
        
        unsafe {
            let class = class!(NSError);
            let obj: *mut Object = msg_send![class, errorWithDomain:ns_domain.as_ref() code:code userInfo:user_info_ptr];
            NSError::from_ptr(obj)
        }
    }

    /// Returns the error domain.
    pub fn domain(&self) -> NSString {
        unsafe {
            let domain: *mut Object = msg_send![self.as_ref(), domain];
            NSString::from_ptr(domain)
        }
    }

    /// Returns the error code.
    pub fn code(&self) -> NSInteger {
        unsafe {
            msg_send![self.as_ref(), code]
        }
    }

    /// Returns the user info dictionary.
    pub fn user_info(&self) -> NSDictionary {
        unsafe {
            let dict: *mut Object = msg_send![self.as_ref(), userInfo];
            NSDictionary::from_ptr(dict)
        }
    }

    /// Returns a string containing the localized description of the error.
    pub fn localized_description(&self) -> NSString {
        unsafe {
            let desc: *mut Object = msg_send![self.as_ref(), localizedDescription];
            NSString::from_ptr(desc)
        }
    }

    /// Returns a string containing the localized failure reason of the error.
    pub fn localized_failure_reason(&self) -> Option<NSString> {
        unsafe {
            let reason: *mut Object = msg_send![self.as_ref(), localizedFailureReason];
            if reason.is_null() {
                None
            } else {
                Some(NSString::from_ptr(reason))
            }
        }
    }

    /// Returns a string containing the localized recovery suggestion for the error.
    pub fn localized_recovery_suggestion(&self) -> Option<NSString> {
        unsafe {
            let suggestion: *mut Object = msg_send![self.as_ref(), localizedRecoverySuggestion];
            if suggestion.is_null() {
                None
            } else {
                Some(NSString::from_ptr(suggestion))
            }
        }
    }
}

impl fmt::Debug for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NSError {{ domain: {}, code: {}, description: {} }}", 
            self.domain().as_str(), 
            self.code(), 
            self.localized_description().as_str())
    }
}

impl fmt::Display for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.localized_description().as_str())
    }
}

impl std::error::Error for NSError {}