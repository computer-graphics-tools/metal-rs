//! MTLFunctionLog - A Rust wrapper around Metal's function logging API.
//!
//! This module provides safe Rust bindings to the MTLFunctionLog class from Apple's Metal framework.
//! MTLFunctionLog provides information about function execution during GPU operations, particularly
//! focusing on validation-related information for Metal shaders.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCommandQueue, MTLCompileOptions};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue
//! let command_queue = device.new_command_queue();
//! 
//! // Enable logging in the compile options
//! let mut compile_options = MTLCompileOptions::new();
//! compile_options.set_enable_logging(true);
//! 
//! // Create a library with logging enabled
//! let source = r#"
//!     #include <metal_stdlib>
//!     using namespace metal;
//!     
//!     kernel void test_kernel(device float* buffer [[buffer(0)]],
//!                            uint id [[thread_position_in_grid]]) {
//!         buffer[id] = buffer[id] * 2.0f;
//!     }
//! "#;
//! 
//! let library = device.new_library_with_source(source, &compile_options).unwrap();
//! let function = library.get_function("test_kernel").unwrap();
//! 
//! // Create a command buffer
//! let command_buffer = command_queue.new_command_buffer();
//! 
//! // After command buffer execution
//! // (execution code omitted for brevity)
//! 
//! // Get logs from command buffer
//! if let Some(logs) = command_buffer.logs() {
//!     for log in logs {
//!         if let Some(function_log) = log.as_function_log() {
//!             println!("Function log type: {:?}", function_log.log_type());
//!             if let Some(function) = function_log.function() {
//!                 println!("Function: {}", function.name());
//!             }
//!             if let Some(encoder_label) = function_log.encoder_label() {
//!                 println!("Encoder label: {}", encoder_label);
//!             }
//!         }
//!     }
//! }
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::MTLFunction;

/// A reference to an Objective-C `MTLFunctionLog`.
pub struct MTLFunctionLogRef(Object);

/// An owned Objective-C `MTLFunctionLog`.
pub struct MTLFunctionLog(*mut Object);

/// A reference to an Objective-C `MTLFunctionLogDebugLocation`.
pub struct MTLFunctionLogDebugLocationRef(Object);

/// An owned Objective-C `MTLFunctionLogDebugLocation`.
pub struct MTLFunctionLogDebugLocation(*mut Object);

/// Enum defining the type of a function log.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLFunctionLogType {
    /// Validation log type
    Validation = 0,
}

// Implementation for MTLFunctionLog
unsafe impl ForeignTypeRef for MTLFunctionLogRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionLogRef {}
unsafe impl Sync for MTLFunctionLogRef {}

unsafe impl ForeignType for MTLFunctionLog {
    type CType = Object;
    type Ref = MTLFunctionLogRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionLog {
        MTLFunctionLog(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionLogRef> for MTLFunctionLog {
    fn as_ref(&self) -> &MTLFunctionLogRef {
        unsafe { &*(self.0.cast::<MTLFunctionLogRef>()) }
    }
}

unsafe impl Send for MTLFunctionLog {}
unsafe impl Sync for MTLFunctionLog {}

unsafe impl objc::Message for MTLFunctionLogRef {}

// Implementation for MTLFunctionLogDebugLocation
unsafe impl ForeignTypeRef for MTLFunctionLogDebugLocationRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionLogDebugLocationRef {}
unsafe impl Sync for MTLFunctionLogDebugLocationRef {}

unsafe impl ForeignType for MTLFunctionLogDebugLocation {
    type CType = Object;
    type Ref = MTLFunctionLogDebugLocationRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionLogDebugLocation {
        MTLFunctionLogDebugLocation(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionLogDebugLocationRef> for MTLFunctionLogDebugLocation {
    fn as_ref(&self) -> &MTLFunctionLogDebugLocationRef {
        unsafe { &*(self.0.cast::<MTLFunctionLogDebugLocationRef>()) }
    }
}

unsafe impl Send for MTLFunctionLogDebugLocation {}
unsafe impl Sync for MTLFunctionLogDebugLocation {}

unsafe impl objc::Message for MTLFunctionLogDebugLocationRef {}

impl MTLFunctionLog {
    /// Returns the type of the function log.
    #[must_use]
    pub fn log_type(&self) -> MTLFunctionLogType {
        unsafe {
            let log_type: u64 = msg_send![self.as_ref(), type];
            std::mem::transmute(log_type)
        }
    }
    
    /// Returns the label of the encoder that generated this log.
    #[must_use]
    pub fn encoder_label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), encoderLabel];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Returns the function that generated this log.
    #[must_use]
    pub fn function(&self) -> Option<MTLFunction> {
        unsafe {
            let function: *mut Object = msg_send![self.as_ref(), function];
            if function.is_null() {
                None
            } else {
                Some(MTLFunction::from_ptr(function))
            }
        }
    }
    
    /// Returns the debug location for this log.
    #[must_use]
    pub fn debug_location(&self) -> Option<MTLFunctionLogDebugLocation> {
        unsafe {
            let location: *mut Object = msg_send![self.as_ref(), debugLocation];
            if location.is_null() {
                None
            } else {
                Some(MTLFunctionLogDebugLocation::from_ptr(location))
            }
        }
    }
}

impl MTLFunctionLogDebugLocation {
    /// Returns the URL (file path) of the source file.
    #[must_use]
    pub fn url(&self) -> Option<String> {
        unsafe {
            let url: *mut Object = msg_send![self.as_ref(), URL];
            if url.is_null() {
                None
            } else {
                // URL is NSURL, we need to extract absoluteString
                let absolute_string: *mut Object = msg_send![url, absoluteString];
                if absolute_string.is_null() {
                    None
                } else {
                    let ns_string = NSString::from_ptr(absolute_string);
                    Some(ns_string.to_rust_string())
                }
            }
        }
    }
    
    /// Returns the function name.
    #[must_use]
    pub fn function_name(&self) -> Option<String> {
        unsafe {
            let name: *mut Object = msg_send![self.as_ref(), functionName];
            if name.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(name);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Returns the line number in the source file.
    #[must_use]
    pub fn line(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), line]
        }
    }
    
    /// Returns the column number in the source file.
    #[must_use]
    pub fn column(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), column]
        }
    }
}

impl fmt::Debug for MTLFunctionLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionLog")
            .field("type", &self.log_type())
            .field("encoder_label", &self.encoder_label())
            .field("function", &self.function().map(|func| func.name()))
            .field("debug_location", &self.debug_location())
            .finish()
    }
}

impl fmt::Debug for MTLFunctionLogDebugLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionLogDebugLocation")
            .field("url", &self.url())
            .field("function_name", &self.function_name())
            .field("line", &self.line())
            .field("column", &self.column())
            .finish()
    }
}

impl Drop for MTLFunctionLog {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionLog {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionLog::from_ptr(obj)
        }
    }
}

impl Drop for MTLFunctionLogDebugLocation {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionLogDebugLocation {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionLogDebugLocation::from_ptr(obj)
        }
    }
}