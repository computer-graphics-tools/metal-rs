//! MTLComputePass - A Rust wrapper around Metal's MTLComputePass classes.
//!
//! This module provides safe Rust bindings to the MTLComputePass related classes from Apple's Metal framework.
//! MTLComputePass is used to describe compute pass configuration for encoding compute commands.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLDispatchType};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue and command buffer
//! let command_queue = device.new_command_queue();
//! let command_buffer = command_queue.new_command_buffer();
//! 
//! // Create a compute pass descriptor
//! let compute_pass_descriptor = metal_rs::metal::MTLComputePassDescriptor::new();
//! compute_pass_descriptor.set_dispatch_type(MTLDispatchType::Concurrent);
//! 
//! // Create a compute command encoder with the descriptor
//! let compute_encoder = command_buffer.new_compute_command_encoder_with_descriptor(&compute_pass_descriptor);
//! 
//! // Use the compute encoder
//! compute_encoder.set_label("Custom Compute Encoder");
//! 
//! // End encoding and commit
//! compute_encoder.end_encoding();
//! command_buffer.commit();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSUInteger;
use crate::metal::compute_command_encoder::MTLDispatchType;

/// A reference to an Objective-C `MTLComputePassSampleBufferAttachmentDescriptor`.
pub struct MTLComputePassSampleBufferAttachmentDescriptorRef(Object);

/// An owned Objective-C `MTLComputePassSampleBufferAttachmentDescriptor`.
pub struct MTLComputePassSampleBufferAttachmentDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLComputePassSampleBufferAttachmentDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLComputePassSampleBufferAttachmentDescriptorRef {}
unsafe impl Sync for MTLComputePassSampleBufferAttachmentDescriptorRef {}

unsafe impl ForeignType for MTLComputePassSampleBufferAttachmentDescriptor {
    type CType = Object;
    type Ref = MTLComputePassSampleBufferAttachmentDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLComputePassSampleBufferAttachmentDescriptor {
        MTLComputePassSampleBufferAttachmentDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLComputePassSampleBufferAttachmentDescriptorRef> for MTLComputePassSampleBufferAttachmentDescriptor {
    fn as_ref(&self) -> &MTLComputePassSampleBufferAttachmentDescriptorRef {
        unsafe { &*(self.as_ptr().cast::<MTLComputePassSampleBufferAttachmentDescriptorRef>()) }
    }
}

unsafe impl Send for MTLComputePassSampleBufferAttachmentDescriptor {}
unsafe impl Sync for MTLComputePassSampleBufferAttachmentDescriptor {}

unsafe impl objc::Message for MTLComputePassSampleBufferAttachmentDescriptorRef {}

impl MTLComputePassSampleBufferAttachmentDescriptor {
    /// Creates a new compute pass sample buffer attachment descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLComputePassSampleBufferAttachmentDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            Self::from_ptr(obj)
        }
    }
    
    /// Returns the start of encoder sample index.
    #[must_use]
    pub fn start_of_encoder_sample_index(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), startOfEncoderSampleIndex]
        }
    }
    
    /// Sets the start of encoder sample index.
    pub fn set_start_of_encoder_sample_index(&self, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStartOfEncoderSampleIndex:index];
        }
    }
    
    /// Returns the end of encoder sample index.
    #[must_use]
    pub fn end_of_encoder_sample_index(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), endOfEncoderSampleIndex]
        }
    }
    
    /// Sets the end of encoder sample index.
    pub fn set_end_of_encoder_sample_index(&self, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setEndOfEncoderSampleIndex:index];
        }
    }
}

impl fmt::Debug for MTLComputePassSampleBufferAttachmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLComputePassSampleBufferAttachmentDescriptor")
            .field("start_of_encoder_sample_index", &self.start_of_encoder_sample_index())
            .field("end_of_encoder_sample_index", &self.end_of_encoder_sample_index())
            .finish()
    }
}

impl Drop for MTLComputePassSampleBufferAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.as_ptr(), release];
        }
    }
}

impl Clone for MTLComputePassSampleBufferAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ptr(), copy];
            Self::from_ptr(ptr)
        }
    }
}

/// A reference to an Objective-C `MTLComputePassSampleBufferAttachmentDescriptorArray`.
pub struct MTLComputePassSampleBufferAttachmentDescriptorArrayRef(Object);

/// An owned Objective-C `MTLComputePassSampleBufferAttachmentDescriptorArray`.
pub struct MTLComputePassSampleBufferAttachmentDescriptorArray(*mut Object);

unsafe impl ForeignTypeRef for MTLComputePassSampleBufferAttachmentDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLComputePassSampleBufferAttachmentDescriptorArrayRef {}
unsafe impl Sync for MTLComputePassSampleBufferAttachmentDescriptorArrayRef {}

unsafe impl ForeignType for MTLComputePassSampleBufferAttachmentDescriptorArray {
    type CType = Object;
    type Ref = MTLComputePassSampleBufferAttachmentDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLComputePassSampleBufferAttachmentDescriptorArray {
        MTLComputePassSampleBufferAttachmentDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLComputePassSampleBufferAttachmentDescriptorArrayRef> for MTLComputePassSampleBufferAttachmentDescriptorArray {
    fn as_ref(&self) -> &MTLComputePassSampleBufferAttachmentDescriptorArrayRef {
        unsafe { &*(self.as_ptr().cast::<MTLComputePassSampleBufferAttachmentDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLComputePassSampleBufferAttachmentDescriptorArray {}
unsafe impl Sync for MTLComputePassSampleBufferAttachmentDescriptorArray {}

unsafe impl objc::Message for MTLComputePassSampleBufferAttachmentDescriptorArrayRef {}

impl MTLComputePassSampleBufferAttachmentDescriptorArray {
    /// Returns the object at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> MTLComputePassSampleBufferAttachmentDescriptor {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), objectAtIndexedSubscript:index];
            MTLComputePassSampleBufferAttachmentDescriptor::from_ptr(ptr)
        }
    }
    
    /// Sets the object at the specified index.
    pub fn set_object(&self, attachment: &MTLComputePassSampleBufferAttachmentDescriptor, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setObject:attachment.as_ptr() atIndexedSubscript:index];
        }
    }
}

impl fmt::Debug for MTLComputePassSampleBufferAttachmentDescriptorArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLComputePassSampleBufferAttachmentDescriptorArray")
            .finish()
    }
}

impl Drop for MTLComputePassSampleBufferAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.as_ptr(), release];
        }
    }
}

impl Clone for MTLComputePassSampleBufferAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.as_ptr(), retain];
            Self::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLComputePassDescriptor`.
pub struct MTLComputePassDescriptorRef(Object);

/// An owned Objective-C `MTLComputePassDescriptor`.
pub struct MTLComputePassDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLComputePassDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLComputePassDescriptorRef {}
unsafe impl Sync for MTLComputePassDescriptorRef {}

unsafe impl ForeignType for MTLComputePassDescriptor {
    type CType = Object;
    type Ref = MTLComputePassDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLComputePassDescriptor {
        MTLComputePassDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLComputePassDescriptorRef> for MTLComputePassDescriptor {
    fn as_ref(&self) -> &MTLComputePassDescriptorRef {
        unsafe { &*(self.as_ptr().cast::<MTLComputePassDescriptorRef>()) }
    }
}

unsafe impl Send for MTLComputePassDescriptor {}
unsafe impl Sync for MTLComputePassDescriptor {}

unsafe impl objc::Message for MTLComputePassDescriptorRef {}

impl MTLComputePassDescriptor {
    /// Creates a new compute pass descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLComputePassDescriptor);
            let ptr: *mut Object = msg_send![class, computePassDescriptor];
            Self::from_ptr(ptr)
        }
    }
    
    /// Returns the dispatch type.
    #[must_use]
    pub fn dispatch_type(&self) -> MTLDispatchType {
        unsafe {
            msg_send![self.as_ref(), dispatchType]
        }
    }
    
    /// Sets the dispatch type.
    pub fn set_dispatch_type(&self, dispatch_type: MTLDispatchType) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDispatchType:dispatch_type];
        }
    }
    
    /// Returns the sample buffer attachments array.
    #[must_use]
    pub fn sample_buffer_attachments(&self) -> MTLComputePassSampleBufferAttachmentDescriptorArray {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), sampleBufferAttachments];
            MTLComputePassSampleBufferAttachmentDescriptorArray::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLComputePassDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLComputePassDescriptor")
            .field("dispatch_type", &self.dispatch_type())
            .finish()
    }
}

impl Drop for MTLComputePassDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.as_ptr(), release];
        }
    }
}

impl Clone for MTLComputePassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ptr(), copy];
            Self::from_ptr(ptr)
        }
    }
}