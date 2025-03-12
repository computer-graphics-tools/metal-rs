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
use crate::foundation::{NSString, NSArray, NSArrayRef};
use crate::metal::library::MTLFunctionConstantValues;
use std::fmt;

/// Options for creating Metal functions.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLFunctionOptions {
    /// No options.
    None = 0,
    /// Compile the function to binary.
    CompileToBinary = 1,
    /// Store the function in a Metal script.
    StoreFunctionInMetalScript = 2,
    // Note: StoreFunctionInMetalPipelinesScript = 2 is not included separately 
    // as it has the same value as StoreFunctionInMetalScript
    /// Fail if the binary archive is missing.
    FailOnBinaryArchiveMiss = 4,
}

/// A reference to an Objective-C `MTLFunctionDescriptor`.
pub struct MTLFunctionDescriptorRef(Object);

/// An owned Objective-C `MTLFunctionDescriptor`.
pub struct MTLFunctionDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLFunctionDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionDescriptorRef {}
unsafe impl Sync for MTLFunctionDescriptorRef {}

unsafe impl ForeignType for MTLFunctionDescriptor {
    type CType = Object;
    type Ref = MTLFunctionDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionDescriptor {
        MTLFunctionDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionDescriptorRef> for MTLFunctionDescriptor {
    fn as_ref(&self) -> &MTLFunctionDescriptorRef {
        unsafe { &*(self.0.cast::<MTLFunctionDescriptorRef>()) }
    }
}

unsafe impl Send for MTLFunctionDescriptor {}
unsafe impl Sync for MTLFunctionDescriptor {}

unsafe impl objc::Message for MTLFunctionDescriptorRef {}

impl MTLFunctionDescriptor {
    /// Creates a new function descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLFunctionDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLFunctionDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new function descriptor with default values.
    pub fn function_descriptor() -> Self {
        unsafe {
            let class = class!(MTLFunctionDescriptor);
            let obj: *mut Object = msg_send![class, functionDescriptor];
            MTLFunctionDescriptor::from_ptr(obj)
        }
    }
}

impl MTLFunctionDescriptorRef {
    /// Gets the name of the function.
    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe {
            let obj: *mut Object = msg_send![self, name];
            if obj.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(obj);
                Some(ns_string.to_rust_string())
            }
        }
    }

    /// Sets the name of the function.
    pub fn set_name(&self, name: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            let _: () = msg_send![self, setName: ns_string.as_ptr()];
        }
    }

    /// Gets the specialized name of the function.
    #[must_use]
    pub fn specialized_name(&self) -> Option<String> {
        unsafe {
            let obj: *mut Object = msg_send![self, specializedName];
            if obj.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(obj);
                Some(ns_string.to_rust_string())
            }
        }
    }

    /// Sets the specialized name of the function.
    pub fn set_specialized_name(&self, name: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            let _: () = msg_send![self, setSpecializedName: ns_string.as_ptr()];
        }
    }

    /// Gets the constant values used for the function.
    #[must_use]
    pub fn constant_values(&self) -> Option<MTLFunctionConstantValues> {
        unsafe {
            let obj: *mut Object = msg_send![self, constantValues];
            if obj.is_null() {
                None
            } else {
                Some(MTLFunctionConstantValues::from_ptr(obj))
            }
        }
    }

    /// Sets the constant values used for the function.
    pub fn set_constant_values(&self, values: Option<&MTLFunctionConstantValues>) {
        unsafe {
            let ptr = match values {
                Some(values) => values.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self, setConstantValues: ptr];
        }
    }

    /// Gets the options for the function.
    #[must_use]
    pub fn options(&self) -> MTLFunctionOptions {
        unsafe {
            msg_send![self, options]
        }
    }

    /// Sets the options for the function.
    pub fn set_options(&self, options: MTLFunctionOptions) {
        unsafe {
            let _: () = msg_send![self, setOptions: options];
        }
    }

    /// Gets the binary archives for the function.
    #[must_use]
    pub fn binary_archives(&self) -> NSArray {
        unsafe {
            let obj: *mut Object = msg_send![self, binaryArchives];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the binary archives for the function.
    pub fn set_binary_archives(&self, archives: &impl AsRef<NSArrayRef>) {
        unsafe {
            let _: () = msg_send![self, setBinaryArchives: archives.as_ref().as_ptr()];
        }
    }
}

impl Clone for MTLFunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLFunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLFunctionDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLFunctionDescriptor")
            .field("name", &self.as_ref().name())
            .field("specialized_name", &self.as_ref().specialized_name())
            .field("options", &self.as_ref().options())
            .finish()
    }
}

/// A reference to an Objective-C `MTLIntersectionFunctionDescriptor`.
pub struct MTLIntersectionFunctionDescriptorRef(Object);

/// An owned Objective-C `MTLIntersectionFunctionDescriptor`.
pub struct MTLIntersectionFunctionDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLIntersectionFunctionDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLIntersectionFunctionDescriptorRef {}
unsafe impl Sync for MTLIntersectionFunctionDescriptorRef {}

unsafe impl ForeignType for MTLIntersectionFunctionDescriptor {
    type CType = Object;
    type Ref = MTLIntersectionFunctionDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIntersectionFunctionDescriptor {
        MTLIntersectionFunctionDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIntersectionFunctionDescriptorRef> for MTLIntersectionFunctionDescriptor {
    fn as_ref(&self) -> &MTLIntersectionFunctionDescriptorRef {
        unsafe { &*(self.0.cast::<MTLIntersectionFunctionDescriptorRef>()) }
    }
}

unsafe impl Send for MTLIntersectionFunctionDescriptor {}
unsafe impl Sync for MTLIntersectionFunctionDescriptor {}

unsafe impl objc::Message for MTLIntersectionFunctionDescriptorRef {}

impl MTLIntersectionFunctionDescriptor {
    /// Creates a new intersection function descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLIntersectionFunctionDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLIntersectionFunctionDescriptor::from_ptr(obj)
        }
    }
}

impl Clone for MTLIntersectionFunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIntersectionFunctionDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLIntersectionFunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLIntersectionFunctionDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLIntersectionFunctionDescriptor")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // This test only runs on macOS with Metal support
    #[test]
    #[cfg(target_os = "macos")]
    #[ignore] // Only run manually since it requires a Metal environment
    fn test_function_descriptor_creation() {
        // Create a new function descriptor
        let descriptor = MTLFunctionDescriptor::new();
        
        // Set the function name
        descriptor.as_ref().set_name("test_function");
        
        // Set the specialized name
        descriptor.as_ref().set_specialized_name("test_function_specialized");
        
        // Set options
        descriptor.as_ref().set_options(MTLFunctionOptions::CompileToBinary);
        
        // Verify the configuration
        assert_eq!(descriptor.as_ref().name().unwrap(), "test_function");
        assert_eq!(descriptor.as_ref().specialized_name().unwrap(), "test_function_specialized");
        assert_eq!(descriptor.as_ref().options(), MTLFunctionOptions::CompileToBinary);
    }
}