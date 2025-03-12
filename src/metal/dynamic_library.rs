// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(dead_code)]
use std::fmt::{Debug, Formatter};

use foreign_types::{ForeignType, ForeignTypeRef};
use objc::{class, msg_send, runtime::{BOOL, Object, YES}, sel, sel_impl};

use crate::foundation::{NSString};
use crate::metal::device::MTLDeviceRef;

/// Dynamic library error codes.
#[repr(u64)]
#[derive(Copy, Clone, Debug)]
pub enum MTLDynamicLibraryError {
    /// No error.
    None = 0,
    /// The given dynamic library file is invalid or corrupted.
    InvalidFile = 1,
    /// The compilation failed when creating the dynamic library.
    CompilationFailure = 2,
    /// The install name for a dynamic library could not be resolved.
    UnresolvedInstallName = 3,
    /// A dependency of the dynamic library failed to load.
    DependencyLoadFailure = 4,
    /// The operation is not supported on this device.
    Unsupported = 5,
}

/// A container for Metal functions that can be dynamically loaded.
///
/// Dynamic libraries contain Metal functions that can be loaded at runtime. Use this type
/// to load and access dynamic Metal libraries.
///
/// # Examples
///
/// ```no_run
/// use metal::{Device, NSURL};
///
/// let device = Device::system_default().expect("No device found");
/// let url = NSURL::file_url_with_path("/path/to/library.metallib", false);
///
/// // Load a dynamic library from disk
/// let dynamic_library = device.new_dynamic_library(&url)
///     .expect("Failed to load dynamic library");
///
/// // Get the install name
/// let install_name = dynamic_library.install_name().unwrap_or_else(|| "No install name".to_string());
/// println!("Dynamic library install name: {}", install_name);
///
/// // Save the dynamic library to a different location
/// let save_url = NSURL::file_url_with_path("/path/to/save.metallib", false);
/// let _ = dynamic_library.serialize_to_url(&save_url);
/// ```
pub struct MTLDynamicLibraryRef(Object);

pub struct MTLDynamicLibrary(*mut Object);

unsafe impl ForeignTypeRef for MTLDynamicLibraryRef {
    type CType = Object;
}

unsafe impl Send for MTLDynamicLibraryRef {}
unsafe impl Sync for MTLDynamicLibraryRef {}

unsafe impl ForeignType for MTLDynamicLibrary {
    type CType = Object;
    type Ref = MTLDynamicLibraryRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLDynamicLibrary {
        MTLDynamicLibrary(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLDynamicLibraryRef> for MTLDynamicLibrary {
    fn as_ref(&self) -> &MTLDynamicLibraryRef {
        unsafe { &*(self.0.cast::<MTLDynamicLibraryRef>()) }
    }
}

unsafe impl Send for MTLDynamicLibrary {}
unsafe impl Sync for MTLDynamicLibrary {}

unsafe impl objc::Message for MTLDynamicLibraryRef {}

impl Drop for MTLDynamicLibrary {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLDynamicLibrary {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = msg_send![self.0, retain];
            MTLDynamicLibrary(ptr)
        }
    }
}

impl MTLDynamicLibraryRef {
    /// Gets the label of the dynamic library.
    ///
    /// # Returns
    ///
    /// The label of the dynamic library.
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.as_str().to_string())
            }
        }
    }

    /// Sets the label of the dynamic library.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the dynamic library.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            msg_send![self, setLabel: ns_string.as_ptr()]
        }
    }

    /// Gets the device that created this dynamic library.
    ///
    /// # Returns
    ///
    /// The device that created this dynamic library.
    pub fn device(&self) -> &MTLDeviceRef {
        unsafe {
            let ptr: *mut MTLDeviceRef = msg_send![self, device];
            &*ptr
        }
    }

    /// Gets the install name of the dynamic library.
    ///
    /// The install name is a unique identifier for the dynamic library. It can be used
    /// to reference the library from other dynamic libraries.
    ///
    /// # Returns
    ///
    /// The install name of the dynamic library.
    pub fn install_name(&self) -> Option<String> {
        unsafe {
            let name: *mut Object = msg_send![self, installName];
            if name.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(name);
                Some(ns_string.as_str().to_string())
            }
        }
    }

    /// Serializes the dynamic library to a URL.
    ///
    /// # Arguments
    ///
    /// * `url_string` - The URL to serialize the dynamic library to.
    ///
    /// # Returns
    ///
    /// `true` if the dynamic library was serialized successfully, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use metal::{DynamicLibrary};
    /// # let dynamic_library: DynamicLibrary = unimplemented!();
    /// let success = dynamic_library.serialize_to_url("/path/to/save.metallib");
    /// ```
    pub fn serialize_to_url(&self, url_string: &str) -> Result<(), String> {
        unsafe {
            let ns_string = NSString::from_rust_str(url_string);
            let url_class = class!(NSURL);
            let url: *mut Object = msg_send![url_class, URLWithString: ns_string.as_ptr()];
            
            if url.is_null() {
                return Err("Invalid URL".to_string());
            }
            
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, serializeToURL:url error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to serialize dynamic library".to_string())
            }
        }
    }
}

impl Debug for MTLDynamicLibraryRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MTLDynamicLibrary")
            .field("label", &self.label())
            .field("install_name", &self.install_name())
            .finish()
    }
}