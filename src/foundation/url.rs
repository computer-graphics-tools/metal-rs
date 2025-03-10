//! NSURL - An object that represents a URL.

use crate::foreign_obj_type;
use crate::foundation::string::{NSString, NSStringRef};
use crate::foundation::data::NSData;
use crate::msg_send_bool;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL};
use std::fmt;
use std::path::Path;

/// A reference to an Objective-C `NSURL`.
pub enum NSURLRef {}

/// An owned Objective-C `NSURL`.
pub struct NSURL(foreign_types::ForeignObjectRef<NSURLRef>);

foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSURL;
    pub struct NSURLRef;
    type ParentType = super::object::NSObjectRef;
}

unsafe impl objc::Message for NSURLRef {}

impl NSURL {
    /// Creates a URL with a string.
    pub fn from_string(string: &str) -> Option<Self> {
        let ns_string = NSString::from_str(string);
        
        unsafe {
            let class = class!(NSURL);
            let url: *mut Object = msg_send![class, URLWithString:, ns_string.as_ref()];
            
            if url.is_null() {
                None
            } else {
                Some(NSURL::from_ptr(url))
            }
        }
    }

    /// Creates a URL with a file path.
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path_str = path.as_ref().to_string_lossy();
        let ns_string = NSString::from_str(&path_str);
        
        unsafe {
            let class = class!(NSURL);
            let url: *mut Object = msg_send![class, fileURLWithPath:, ns_string.as_ref()];
            
            if url.is_null() {
                None
            } else {
                Some(NSURL::from_ptr(url))
            }
        }
    }

    /// Returns the URL's string representation.
    pub fn absolute_string(&self) -> NSString {
        unsafe {
            let string: *mut Object = msg_send![self.as_ref(), absoluteString];
            NSString::from_ptr(string)
        }
    }

    /// Returns whether the URL is a file URL.
    pub fn is_file_url(&self) -> bool {
        unsafe {
            msg_send_bool!(self.as_ref(), isFileURL)
        }
    }

    /// Returns the path component of the URL.
    pub fn path(&self) -> NSString {
        unsafe {
            let path: *mut Object = msg_send![self.as_ref(), path];
            NSString::from_ptr(path)
        }
    }

    /// Returns the scheme component of the URL.
    pub fn scheme(&self) -> Option<NSString> {
        unsafe {
            let scheme: *mut Object = msg_send![self.as_ref(), scheme];
            if scheme.is_null() {
                None
            } else {
                Some(NSString::from_ptr(scheme))
            }
        }
    }

    /// Returns the host component of the URL.
    pub fn host(&self) -> Option<NSString> {
        unsafe {
            let host: *mut Object = msg_send![self.as_ref(), host];
            if host.is_null() {
                None
            } else {
                Some(NSString::from_ptr(host))
            }
        }
    }

    /// Returns the path extension of the URL.
    pub fn path_extension(&self) -> NSString {
        unsafe {
            let extension: *mut Object = msg_send![self.as_ref(), pathExtension];
            NSString::from_ptr(extension)
        }
    }

    /// Returns the last path component of the URL.
    pub fn last_path_component(&self) -> NSString {
        unsafe {
            let component: *mut Object = msg_send![self.as_ref(), lastPathComponent];
            NSString::from_ptr(component)
        }
    }

    /// Returns a new URL with the path extension appended.
    pub fn append_path_extension(&self, extension: &str) -> Option<NSURL> {
        let ns_extension = NSString::from_str(extension);
        
        unsafe {
            let url: *mut Object = msg_send![self.as_ref(), URLByAppendingPathExtension:, ns_extension.as_ref()];
            if url.is_null() {
                None
            } else {
                Some(NSURL::from_ptr(url))
            }
        }
    }

    /// Returns a new URL with the path component appended.
    pub fn append_path_component(&self, component: &str, is_directory: bool) -> NSURL {
        let ns_component = NSString::from_str(component);
        
        unsafe {
            let url: *mut Object = msg_send![self.as_ref(), URLByAppendingPathComponent:isDirectory:, 
                                         ns_component.as_ref(), 
                                         if is_directory { YES } else { NO }];
            NSURL::from_ptr(url)
        }
    }

    /// Returns a new URL with the last path component deleted.
    pub fn delete_last_path_component(&self) -> NSURL {
        unsafe {
            let url: *mut Object = msg_send![self.as_ref(), URLByDeletingLastPathComponent];
            NSURL::from_ptr(url)
        }
    }

    /// Returns a new URL with the path extension deleted.
    pub fn delete_path_extension(&self) -> NSURL {
        unsafe {
            let url: *mut Object = msg_send![self.as_ref(), URLByDeletingPathExtension];
            NSURL::from_ptr(url)
        }
    }

    /// Returns a new URL with a standardized path.
    pub fn standardized_url(&self) -> NSURL {
        unsafe {
            let url: *mut Object = msg_send![self.as_ref(), standardizedURL];
            NSURL::from_ptr(url)
        }
    }

    /// Returns a Boolean value that indicates whether a URL's resource exists.
    pub fn check_resource_is_reachable(&self) -> Result<bool, super::NSError> {
        let mut error: *mut objc::runtime::Object = std::ptr::null_mut();
        let result: BOOL = unsafe {
            msg_send![self.as_ref(), checkResourceIsReachableAndReturnError:, &mut error]
        };
        
        if !error.is_null() {
            Err(unsafe { super::NSError::from_ptr(error) })
        } else {
            Ok(result == YES)
        }
    }

    /// Loads the contents of a URL into memory.
    pub fn load_data_to_memory(&self) -> Result<NSData, super::NSError> {
        let mut error: *mut objc::runtime::Object = std::ptr::null_mut();
        let data: *mut objc::runtime::Object = unsafe {
            msg_send![class!(NSData), dataWithContentsOfURL:options:error:, 
                     self.as_ref(), 
                     0, 
                     &mut error]
        };
        
        if !error.is_null() {
            Err(unsafe { super::NSError::from_ptr(error) })
        } else {
            Ok(unsafe { NSData::from_ptr(data) })
        }
    }
}

impl fmt::Debug for NSURL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.absolute_string().as_str())
    }
}

impl fmt::Display for NSURL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.absolute_string().as_str())
    }
}

impl Clone for NSURL {
    fn clone(&self) -> Self {
        unsafe {
            let copy: *mut Object = msg_send![self.as_ref(), copy];
            NSURL::from_ptr(copy)
        }
    }
}

impl PartialEq for NSURL {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            msg_send_bool!(self.as_ref(), isEqual:, other.as_ref())
        }
    }
}

impl Eq for NSURL {}