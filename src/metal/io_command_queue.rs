//! MTLIOCommandQueue - A Rust wrapper around Metal's I/O command queue API.
//!
//! This module provides Rust bindings to the MTLIOCommandQueue class from Apple's Metal framework.
//! MTLIOCommandQueue is used for creating and managing I/O command buffers for asynchronous file I/O operations.

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::{Object, Class};
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSInteger, NSUInteger, NSString};
use crate::metal::io_command_buffer::MTLIOCommandBuffer;

/// The priority of an I/O command queue.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLIOPriority {
    /// High priority.
    High = 0,
    /// Normal priority.
    Normal = 1,
    /// Low priority.
    Low = 2,
}

/// The type of an I/O command queue.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLIOCommandQueueType {
    /// A concurrent queue that can execute multiple command buffers in parallel.
    Concurrent = 0,
    /// A serial queue that executes command buffers one at a time.
    Serial = 1,
}

/// A reference to an Objective-C `MTLIOCommandQueueDescriptor`.
pub struct MTLIOCommandQueueDescriptorRef(Object);

/// An owned Objective-C `MTLIOCommandQueueDescriptor`.
pub struct MTLIOCommandQueueDescriptor(*mut Object);

// Implementation for MTLIOCommandQueueDescriptor
unsafe impl ForeignTypeRef for MTLIOCommandQueueDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLIOCommandQueueDescriptorRef {}
unsafe impl Sync for MTLIOCommandQueueDescriptorRef {}

unsafe impl ForeignType for MTLIOCommandQueueDescriptor {
    type CType = Object;
    type Ref = MTLIOCommandQueueDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIOCommandQueueDescriptor {
        MTLIOCommandQueueDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIOCommandQueueDescriptorRef> for MTLIOCommandQueueDescriptor {
    fn as_ref(&self) -> &MTLIOCommandQueueDescriptorRef {
        unsafe { &*(self.0.cast::<MTLIOCommandQueueDescriptorRef>()) }
    }
}

unsafe impl Send for MTLIOCommandQueueDescriptor {}
unsafe impl Sync for MTLIOCommandQueueDescriptor {}

unsafe impl objc::Message for MTLIOCommandQueueDescriptorRef {}

impl MTLIOCommandQueueDescriptor {
    /// Creates a new `MTLIOCommandQueueDescriptor`.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLIOCommandQueueDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLIOCommandQueueDescriptor::from_ptr(obj)
        }
    }
    
    /// Returns the maximum number of command buffers.
    #[must_use]
    pub fn max_command_buffer_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxCommandBufferCount]
        }
    }
    
    /// Sets the maximum number of command buffers.
    pub fn set_max_command_buffer_count(&self, count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxCommandBufferCount:count];
        }
    }
    
    /// Returns the priority of the command queue.
    #[must_use]
    pub fn priority(&self) -> MTLIOPriority {
        unsafe {
            let priority: u64 = msg_send![self.as_ref(), priority];
            std::mem::transmute(priority)
        }
    }
    
    /// Sets the priority of the command queue.
    pub fn set_priority(&self, priority: MTLIOPriority) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setPriority:priority];
        }
    }
    
    /// Returns the type of the command queue.
    #[must_use]
    pub fn queue_type(&self) -> MTLIOCommandQueueType {
        unsafe {
            let queue_type: u64 = msg_send![self.as_ref(), type];
            std::mem::transmute(queue_type)
        }
    }
    
    /// Sets the type of the command queue.
    pub fn set_queue_type(&self, queue_type: MTLIOCommandQueueType) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setType:queue_type];
        }
    }
    
    /// Returns the maximum number of commands in flight.
    #[must_use]
    pub fn max_commands_in_flight(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxCommandsInFlight]
        }
    }
    
    /// Sets the maximum number of commands in flight.
    pub fn set_max_commands_in_flight(&self, count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxCommandsInFlight:count];
        }
    }
}

impl fmt::Debug for MTLIOCommandQueueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIOCommandQueueDescriptor")
            .field("max_command_buffer_count", &self.max_command_buffer_count())
            .field("priority", &self.priority())
            .field("queue_type", &self.queue_type())
            .field("max_commands_in_flight", &self.max_commands_in_flight())
            .finish()
    }
}

impl Drop for MTLIOCommandQueueDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIOCommandQueueDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIOCommandQueueDescriptor::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLIOCommandQueue`.
pub struct MTLIOCommandQueueRef(Object);

/// An owned Objective-C `MTLIOCommandQueue`.
pub struct MTLIOCommandQueue(*mut Object);

// Implementation for MTLIOCommandQueue
unsafe impl ForeignTypeRef for MTLIOCommandQueueRef {
    type CType = Object;
}

unsafe impl Send for MTLIOCommandQueueRef {}
unsafe impl Sync for MTLIOCommandQueueRef {}

unsafe impl ForeignType for MTLIOCommandQueue {
    type CType = Object;
    type Ref = MTLIOCommandQueueRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIOCommandQueue {
        MTLIOCommandQueue(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIOCommandQueueRef> for MTLIOCommandQueue {
    fn as_ref(&self) -> &MTLIOCommandQueueRef {
        unsafe { &*(self.0.cast::<MTLIOCommandQueueRef>()) }
    }
}

unsafe impl Send for MTLIOCommandQueue {}
unsafe impl Sync for MTLIOCommandQueue {}

unsafe impl objc::Message for MTLIOCommandQueueRef {}

impl MTLIOCommandQueue {
    /// Returns the label of the command queue.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                String::new()
            } else {
                let nsstring = NSString::from_ptr(label);
                nsstring.to_rust_string()
            }
        }
    }
    
    /// Sets the label of the command queue.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let nsstring = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:nsstring.as_ptr()];
        }
    }
    
    /// Adds a barrier to the queue.
    pub fn enqueue_barrier(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), enqueueBarrier];
        }
    }
    
    /// Creates a new command buffer.
    #[must_use]
    pub fn command_buffer(&self) -> MTLIOCommandBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), commandBuffer];
            MTLIOCommandBuffer::from_ptr(ptr)
        }
    }
    
    /// Creates a new command buffer with unretained references.
    #[must_use]
    pub fn command_buffer_with_unretained_references(&self) -> MTLIOCommandBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), commandBufferWithUnretainedReferences];
            MTLIOCommandBuffer::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLIOCommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIOCommandQueue")
            .field("label", &self.label())
            .finish()
    }
}

impl Drop for MTLIOCommandQueue {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIOCommandQueue {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIOCommandQueue::from_ptr(obj)
        }
    }
}