//! MTLLogState - A Rust wrapper around Metal's log state API.
//!
//! This module provides Rust bindings to the MTLLogState class from Apple's Metal framework.
//! MTLLogState is used for managing GPU logging functionality and handler registration.

use std::fmt;
use std::sync::Arc;
use std::os::raw::c_void;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::{Object, Class};
use objc::rc::StrongPtr;
use foreign_types::{ForeignType, ForeignTypeRef};
use block::Block;
use block::ConcreteBlock;
use crate::foundation::{NSInteger, NSString};

/// The log level for Metal logging.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLLogLevel {
    /// Undefined log level.
    Undefined = 0,
    /// Debug log level.
    Debug = 1,
    /// Info log level.
    Info = 2,
    /// Notice log level.
    Notice = 3,
    /// Error log level.
    Error = 4,
    /// Fault log level.
    Fault = 5,
}

/// Errors that can occur with MTLLogState operations.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLLogStateError {
    /// Invalid size for the log buffer.
    InvalidSize = 1,
    /// Invalid log state.
    Invalid = 2,
}

/// Type for log handler callbacks.
pub type MTLLogHandler = Box<dyn Fn(&str, &str, MTLLogLevel, &str) + Send + Sync>;

/// A reference to an Objective-C `MTLLogStateDescriptor`.
pub struct MTLLogStateDescriptorRef(Object);

/// An owned Objective-C `MTLLogStateDescriptor`.
pub struct MTLLogStateDescriptor(*mut Object);

// Implementation for MTLLogStateDescriptor
unsafe impl ForeignTypeRef for MTLLogStateDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLLogStateDescriptorRef {}
unsafe impl Sync for MTLLogStateDescriptorRef {}

unsafe impl ForeignType for MTLLogStateDescriptor {
    type CType = Object;
    type Ref = MTLLogStateDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLLogStateDescriptor {
        MTLLogStateDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLLogStateDescriptorRef> for MTLLogStateDescriptor {
    fn as_ref(&self) -> &MTLLogStateDescriptorRef {
        unsafe { &*(self.0.cast::<MTLLogStateDescriptorRef>()) }
    }
}

unsafe impl Send for MTLLogStateDescriptor {}
unsafe impl Sync for MTLLogStateDescriptor {}

unsafe impl objc::Message for MTLLogStateDescriptorRef {}

impl MTLLogStateDescriptor {
    /// Creates a new `MTLLogStateDescriptor`.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLLogStateDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLLogStateDescriptor::from_ptr(obj)
        }
    }
    
    /// Sets the log level for the descriptor.
    pub fn set_level(&self, level: MTLLogLevel) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setLevel:level];
        }
    }
    
    /// Gets the log level for the descriptor.
    #[must_use]
    pub fn level(&self) -> MTLLogLevel {
        unsafe {
            let result: u64 = msg_send![self.as_ref(), level];
            std::mem::transmute(result)
        }
    }
    
    /// Sets the buffer size for the descriptor.
    pub fn set_buffer_size(&self, size: NSInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setBufferSize:size];
        }
    }
    
    /// Gets the buffer size for the descriptor.
    #[must_use]
    pub fn buffer_size(&self) -> NSInteger {
        unsafe {
            msg_send![self.as_ref(), bufferSize]
        }
    }
}

impl fmt::Debug for MTLLogStateDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLLogStateDescriptor")
            .field("level", &self.level())
            .field("buffer_size", &self.buffer_size())
            .finish()
    }
}

impl Drop for MTLLogStateDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLLogStateDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLLogStateDescriptor::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLLogState`.
pub struct MTLLogStateRef(Object);

/// An owned Objective-C `MTLLogState`.
pub struct MTLLogState {
    ptr: *mut Object,
    // Store log handlers to keep them alive
    _handlers: Arc<Vec<Arc<MTLLogHandler>>>,
}

// Implementation for MTLLogState
unsafe impl ForeignTypeRef for MTLLogStateRef {
    type CType = Object;
}

unsafe impl Send for MTLLogStateRef {}
unsafe impl Sync for MTLLogStateRef {}

unsafe impl ForeignType for MTLLogState {
    type CType = Object;
    type Ref = MTLLogStateRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLLogState {
        MTLLogState {
            ptr,
            _handlers: Arc::new(Vec::new()),
        }
    }

    fn as_ptr(&self) -> *mut Object {
        self.ptr
    }
}

impl AsRef<MTLLogStateRef> for MTLLogState {
    fn as_ref(&self) -> &MTLLogStateRef {
        unsafe { &*(self.ptr.cast::<MTLLogStateRef>()) }
    }
}

unsafe impl Send for MTLLogState {}
unsafe impl Sync for MTLLogState {}

unsafe impl objc::Message for MTLLogStateRef {}

impl MTLLogState {
    /// Adds a log handler to the log state.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLLogLevel};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let descriptor = MTLLogStateDescriptor::new();
    /// descriptor.set_level(MTLLogLevel::Info);
    /// let log_state = device.new_log_state(&descriptor).unwrap();
    ///
    /// log_state.add_log_handler(|subsystem, category, level, message| {
    ///     println!("[{:?}] {}/{}: {}", level, subsystem, category, message);
    /// });
    /// ```
    pub fn add_log_handler<F>(&mut self, handler: F)
    where
        F: Fn(&str, &str, MTLLogLevel, &str) + Send + Sync + 'static,
    {
        let handler = Arc::new(Box::new(handler) as MTLLogHandler);
        let handler_clone = handler.clone();

        unsafe {
            let block = ConcreteBlock::new(move |subsystem: *mut Object,
                                                  category: *mut Object,
                                                  level: u64,
                                                  message: *mut Object| {
                let subsystem_str = if subsystem.is_null() {
                    String::new()
                } else {
                    let nsstring = NSString::from_ptr(subsystem);
                    nsstring.to_rust_string()
                };

                let category_str = if category.is_null() {
                    String::new()
                } else {
                    let nsstring = NSString::from_ptr(category);
                    nsstring.to_rust_string()
                };

                let message_str = if message.is_null() {
                    String::new()
                } else {
                    let nsstring = NSString::from_ptr(message);
                    nsstring.to_rust_string()
                };

                let log_level: MTLLogLevel = std::mem::transmute(level);

                handler_clone(&subsystem_str, &category_str, log_level, &message_str);
            });
            
            let _: () = msg_send![self.as_ref(), addLogHandler:block];
            
            // Store the handler to keep it alive
            let handlers = Arc::get_mut(&mut self._handlers).unwrap();
            handlers.push(handler);
        }
    }
}

impl fmt::Debug for MTLLogState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLLogState")
            .field("handlers_count", &self._handlers.len())
            .finish()
    }
}

impl Drop for MTLLogState {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.ptr, release];
        }
    }
}

impl Clone for MTLLogState {
    fn clone(&self) -> Self {
        unsafe {
            MTLLogState {
                ptr: {
                    let obj: *mut Object = msg_send![self.ptr, retain];
                    obj
                },
                _handlers: Arc::clone(&self._handlers),
            }
        }
    }
}