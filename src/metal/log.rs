//! MTLLog and related types - A Rust wrapper around Metal's logging API.
//!
//! This module provides safe Rust bindings to the MTLLog, MTLLogContainer, and related classes
//! from Apple's Metal framework. These types are used to capture and analyze logs generated
//! during Metal command execution.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCompileOptions};
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
//! // Execute commands
//! // ...
//! 
//! // Examine logs
//! if let Some(logs) = command_buffer.logs() {
//!     for log in logs {
//!         println!("Log type: {:?}", log.log_type());
//!     }
//! }
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSArray;
use crate::metal::function_log::MTLFunctionLog;

/// A reference to an Objective-C `MTLLog`.
pub struct MTLLogRef(Object);

/// An owned Objective-C `MTLLog`.
pub struct MTLLog(*mut Object);

/// A reference to an Objective-C `MTLLogContainer`.
pub struct MTLLogContainerRef(Object);

/// An owned Objective-C `MTLLogContainer`.
pub struct MTLLogContainer(*mut Object);

/// Enum defining the types of logs that Metal can generate.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLLogType {
    /// Validation related log
    Validation = 0,
    /// Error log
    Error = 1,
}

// Implementation for MTLLog
unsafe impl ForeignTypeRef for MTLLogRef {
    type CType = Object;
}

unsafe impl Send for MTLLogRef {}
unsafe impl Sync for MTLLogRef {}

unsafe impl ForeignType for MTLLog {
    type CType = Object;
    type Ref = MTLLogRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLLog {
        MTLLog(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLLogRef> for MTLLog {
    fn as_ref(&self) -> &MTLLogRef {
        unsafe { &*(self.0.cast::<MTLLogRef>()) }
    }
}

unsafe impl Send for MTLLog {}
unsafe impl Sync for MTLLog {}

unsafe impl objc::Message for MTLLogRef {}

// Implementation for MTLLogContainer
unsafe impl ForeignTypeRef for MTLLogContainerRef {
    type CType = Object;
}

unsafe impl Send for MTLLogContainerRef {}
unsafe impl Sync for MTLLogContainerRef {}

unsafe impl ForeignType for MTLLogContainer {
    type CType = Object;
    type Ref = MTLLogContainerRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLLogContainer {
        MTLLogContainer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLLogContainerRef> for MTLLogContainer {
    fn as_ref(&self) -> &MTLLogContainerRef {
        unsafe { &*(self.0.cast::<MTLLogContainerRef>()) }
    }
}

unsafe impl Send for MTLLogContainer {}
unsafe impl Sync for MTLLogContainer {}

unsafe impl objc::Message for MTLLogContainerRef {}

impl MTLLog {
    /// Returns the type of the log.
    #[must_use]
    pub fn log_type(&self) -> MTLLogType {
        unsafe {
            let log_type: u64 = msg_send![self.as_ref(), type];
            std::mem::transmute(log_type)
        }
    }
    
    /// Attempts to convert this log to a function log.
    ///
    /// # Returns
    ///
    /// `Some(MTLFunctionLog)` if this log is a function log, `None` otherwise.
    #[must_use]
    pub fn as_function_log(&self) -> Option<MTLFunctionLog> {
        unsafe {
            if self.log_type() != MTLLogType::Validation {
                return None;
            }
            
            // Try to cast the log to an MTLFunctionLog
            // In Objective-C this would be done with isKindOfClass: but here we just
            // create a new MTLFunctionLog from the same pointer if the type is correct
            Some(MTLFunctionLog::from_ptr(self.as_ptr()))
        }
    }
}

impl MTLLogContainer {
    /// Returns an array of logs in this container.
    #[must_use]
    pub fn logs(&self) -> NSArray {
        unsafe {
            let logs: *mut Object = msg_send![self.as_ref(), logs];
            NSArray::from_ptr(logs)
        }
    }
}

impl fmt::Debug for MTLLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLLog")
            .field("type", &self.log_type())
            .finish()
    }
}

impl fmt::Debug for MTLLogContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLLogContainer")
            .field("logs_count", &self.logs().count())
            .finish()
    }
}

impl Drop for MTLLog {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLLog {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLLog::from_ptr(obj)
        }
    }
}

impl Drop for MTLLogContainer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLLogContainer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLLogContainer::from_ptr(obj)
        }
    }
}