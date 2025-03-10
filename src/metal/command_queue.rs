//! MTLCommandQueue - A Rust wrapper around Metal's MTLCommandQueue class.
//!
//! This module provides safe Rust bindings to the MTLCommandQueue class from Apple's Metal framework.
//! MTLCommandQueue manages command buffers, which are used to encapsulate and schedule Metal commands.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::MTLCreateSystemDefaultDevice;
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue
//! let queue = device.new_command_queue();
//! 
//! // Create a command buffer
//! let command_buffer = queue.new_command_buffer();
//! 
//! // After configuring the command buffer:
//! command_buffer.commit();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;

// Forward declarations that will be properly defined later
pub struct MTLCommandBufferRef(Object);
pub struct MTLCommandBuffer(*mut Object);

// Implement foreign type traits for MTLCommandBuffer (placeholder)
unsafe impl ForeignTypeRef for MTLCommandBufferRef {
    type CType = Object;
}

unsafe impl ForeignType for MTLCommandBuffer {
    type CType = Object;
    type Ref = MTLCommandBufferRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCommandBuffer {
        MTLCommandBuffer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

// Implementation for MTLCommandQueue
/// A reference to an Objective-C `MTLCommandQueue`.
pub struct MTLCommandQueueRef(Object);

/// An owned Objective-C `MTLCommandQueue`.
pub struct MTLCommandQueue(*mut Object);

unsafe impl ForeignTypeRef for MTLCommandQueueRef {
    type CType = Object;
}

unsafe impl Send for MTLCommandQueueRef {}
unsafe impl Sync for MTLCommandQueueRef {}

unsafe impl ForeignType for MTLCommandQueue {
    type CType = Object;
    type Ref = MTLCommandQueueRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCommandQueue {
        MTLCommandQueue(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCommandQueueRef> for MTLCommandQueue {
    fn as_ref(&self) -> &MTLCommandQueueRef {
        unsafe { &*(self.0.cast::<MTLCommandQueueRef>()) }
    }
}

unsafe impl Send for MTLCommandQueue {}
unsafe impl Sync for MTLCommandQueue {}

unsafe impl objc::Message for MTLCommandQueueRef {}

impl MTLCommandQueue {
    /// Returns the label of the command queue.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Sets the label of the command queue.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Creates a new command buffer that can contain Metal commands.
    #[must_use]
    pub fn new_command_buffer(&self) -> MTLCommandBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), commandBuffer];
            MTLCommandBuffer::from_ptr(ptr)
        }
    }
    
    /// Creates a new command buffer with a specific unretained reference.
    #[must_use]
    pub fn new_command_buffer_with_unretained_references(&self) -> MTLCommandBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), commandBufferWithUnretainedReferences];
            MTLCommandBuffer::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLCommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLCommandQueue {{ label: {} }}", label)
    }
}

impl Drop for MTLCommandQueue {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCommandQueue {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCommandQueue::from_ptr(obj)
        }
    }
}

// Minimal implementation for MTLCommandBuffer to make it work
impl MTLCommandBuffer {
    /// Commits this command buffer for execution.
    pub fn commit(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), commit];
        }
    }
    
    /// Returns the label of the command buffer.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Sets the label of the command buffer.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
}

impl AsRef<MTLCommandBufferRef> for MTLCommandBuffer {
    fn as_ref(&self) -> &MTLCommandBufferRef {
        unsafe { &*(self.0.cast::<MTLCommandBufferRef>()) }
    }
}

unsafe impl objc::Message for MTLCommandBufferRef {}

impl Drop for MTLCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCommandBuffer::from_ptr(obj)
        }
    }
}