//! MTLPipeline - Common types for Metal pipelines.
//!
//! This module provides safe Rust bindings to the common pipeline types from Apple's Metal framework.
//! These types are used by both render and compute pipelines.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLPipelineBufferDescriptor, MTLMutability};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a pipeline buffer descriptor
//! let buffer_desc = MTLPipelineBufferDescriptor::new();
//! buffer_desc.set_mutability(MTLMutability::Immutable);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSUInteger;

/// Describes the mutability of a pipeline buffer.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLMutability {
    /// The buffer is immutable.
    Immutable = 0,
    /// The buffer is mutable.
    Mutable = 1,
}

/// Describes the level of shader validation.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPipelineOption {
    /// No options specified.
    None = 0,
    /// Performs shader validation.
    ArgumentInfo = 1,
    /// Performs additional validation of shader shader resources.
    BufferTypeInfo = 2,
}

/// A reference to an Objective-C `MTLPipelineBufferDescriptor`.
pub struct MTLPipelineBufferDescriptorRef(Object);

/// An owned Objective-C `MTLPipelineBufferDescriptor`.
pub struct MTLPipelineBufferDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLPipelineBufferDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLPipelineBufferDescriptorRef {}
unsafe impl Sync for MTLPipelineBufferDescriptorRef {}

unsafe impl ForeignType for MTLPipelineBufferDescriptor {
    type CType = Object;
    type Ref = MTLPipelineBufferDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLPipelineBufferDescriptor {
        MTLPipelineBufferDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLPipelineBufferDescriptorRef> for MTLPipelineBufferDescriptor {
    fn as_ref(&self) -> &MTLPipelineBufferDescriptorRef {
        unsafe { &*(self.0.cast::<MTLPipelineBufferDescriptorRef>()) }
    }
}

unsafe impl Send for MTLPipelineBufferDescriptor {}
unsafe impl Sync for MTLPipelineBufferDescriptor {}

unsafe impl objc::Message for MTLPipelineBufferDescriptorRef {}

impl MTLPipelineBufferDescriptor {
    /// Creates a new pipeline buffer descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLPipelineBufferDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            Self::from_ptr(obj)
        }
    }
    
    /// Returns the index of the buffer in the argument table.
    #[must_use]
    pub fn index(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), index]
        }
    }
    
    /// Sets the index of the buffer in the argument table.
    pub fn set_index(&self, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setIndex:index];
        }
    }
    
    /// Returns the mutability of the buffer.
    #[must_use]
    pub fn mutability(&self) -> MTLMutability {
        unsafe {
            msg_send![self.as_ref(), mutability]
        }
    }
    
    /// Sets the mutability of the buffer.
    pub fn set_mutability(&self, mutability: MTLMutability) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMutability:mutability];
        }
    }
}

impl fmt::Debug for MTLPipelineBufferDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLPipelineBufferDescriptor")
            .field("index", &self.index())
            .field("mutability", &self.mutability())
            .finish()
    }
}

impl Drop for MTLPipelineBufferDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLPipelineBufferDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut Object = msg_send![self.0, copy];
            Self::from_ptr(ptr)
        }
    }
}

/// A reference to an Objective-C `MTLPipelineBufferDescriptorArray`.
pub struct MTLPipelineBufferDescriptorArrayRef(Object);

/// An owned Objective-C `MTLPipelineBufferDescriptorArray`.
pub struct MTLPipelineBufferDescriptorArray(*mut Object);

unsafe impl ForeignTypeRef for MTLPipelineBufferDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLPipelineBufferDescriptorArrayRef {}
unsafe impl Sync for MTLPipelineBufferDescriptorArrayRef {}

unsafe impl ForeignType for MTLPipelineBufferDescriptorArray {
    type CType = Object;
    type Ref = MTLPipelineBufferDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLPipelineBufferDescriptorArray {
        MTLPipelineBufferDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLPipelineBufferDescriptorArrayRef> for MTLPipelineBufferDescriptorArray {
    fn as_ref(&self) -> &MTLPipelineBufferDescriptorArrayRef {
        unsafe { &*(self.0.cast::<MTLPipelineBufferDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLPipelineBufferDescriptorArray {}
unsafe impl Sync for MTLPipelineBufferDescriptorArray {}

unsafe impl objc::Message for MTLPipelineBufferDescriptorArrayRef {}

impl MTLPipelineBufferDescriptorArray {
    /// Returns the buffer descriptor at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> MTLPipelineBufferDescriptor {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), objectAtIndexedSubscript:index];
            MTLPipelineBufferDescriptor::from_ptr(ptr)
        }
    }
    
    /// Sets the buffer descriptor at the specified index.
    pub fn set_object(&self, buffer_desc: &MTLPipelineBufferDescriptor, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setObject:buffer_desc.as_ptr() atIndexedSubscript:index];
        }
    }
}

impl fmt::Debug for MTLPipelineBufferDescriptorArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLPipelineBufferDescriptorArray")
            .finish()
    }
}

impl Drop for MTLPipelineBufferDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLPipelineBufferDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            Self::from_ptr(obj)
        }
    }
}