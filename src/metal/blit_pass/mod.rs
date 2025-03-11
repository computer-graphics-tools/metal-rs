//! MTLBlitPass - A Rust wrapper around Metal's MTLBlitPass classes.
//!
//! This module provides safe Rust bindings to the MTLBlitPass related classes from Apple's Metal framework.
//! MTLBlitPass is used to describe blit pass configuration for encoding blit commands.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue and command buffer
//! let command_queue = device.new_command_queue();
//! let command_buffer = command_queue.new_command_buffer();
//! 
//! // Create a blit pass descriptor
//! let blit_pass_descriptor = metal_rs::metal::MTLBlitPassDescriptor::new();
//! 
//! // Create a blit command encoder with the descriptor
//! let blit_encoder = command_buffer.new_blit_command_encoder_with_descriptor(&blit_pass_descriptor);
//! 
//! // Use the blit encoder
//! blit_encoder.set_label("Custom Blit Encoder");
//! 
//! // End encoding and commit
//! blit_encoder.end_encoding();
//! command_buffer.commit();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSUInteger;

/// A reference to an Objective-C `MTLBlitPassSampleBufferAttachmentDescriptor`.
pub struct MTLBlitPassSampleBufferAttachmentDescriptorRef(Object);

/// An owned Objective-C `MTLBlitPassSampleBufferAttachmentDescriptor`.
pub struct MTLBlitPassSampleBufferAttachmentDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLBlitPassSampleBufferAttachmentDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLBlitPassSampleBufferAttachmentDescriptorRef {}
unsafe impl Sync for MTLBlitPassSampleBufferAttachmentDescriptorRef {}

unsafe impl ForeignType for MTLBlitPassSampleBufferAttachmentDescriptor {
    type CType = Object;
    type Ref = MTLBlitPassSampleBufferAttachmentDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBlitPassSampleBufferAttachmentDescriptor {
        MTLBlitPassSampleBufferAttachmentDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBlitPassSampleBufferAttachmentDescriptorRef> for MTLBlitPassSampleBufferAttachmentDescriptor {
    fn as_ref(&self) -> &MTLBlitPassSampleBufferAttachmentDescriptorRef {
        unsafe { &*(self.as_ptr().cast::<MTLBlitPassSampleBufferAttachmentDescriptorRef>()) }
    }
}

unsafe impl Send for MTLBlitPassSampleBufferAttachmentDescriptor {}
unsafe impl Sync for MTLBlitPassSampleBufferAttachmentDescriptor {}

unsafe impl objc::Message for MTLBlitPassSampleBufferAttachmentDescriptorRef {}

impl MTLBlitPassSampleBufferAttachmentDescriptor {
    /// Creates a new blit pass sample buffer attachment descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLBlitPassSampleBufferAttachmentDescriptor);
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

impl fmt::Debug for MTLBlitPassSampleBufferAttachmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLBlitPassSampleBufferAttachmentDescriptor")
            .field("start_of_encoder_sample_index", &self.start_of_encoder_sample_index())
            .field("end_of_encoder_sample_index", &self.end_of_encoder_sample_index())
            .finish()
    }
}

impl Drop for MTLBlitPassSampleBufferAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.as_ptr(), release];
        }
    }
}

impl Clone for MTLBlitPassSampleBufferAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ptr(), copy];
            Self::from_ptr(ptr)
        }
    }
}

/// A reference to an Objective-C `MTLBlitPassSampleBufferAttachmentDescriptorArray`.
pub struct MTLBlitPassSampleBufferAttachmentDescriptorArrayRef(Object);

/// An owned Objective-C `MTLBlitPassSampleBufferAttachmentDescriptorArray`.
pub struct MTLBlitPassSampleBufferAttachmentDescriptorArray(*mut Object);

unsafe impl ForeignTypeRef for MTLBlitPassSampleBufferAttachmentDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLBlitPassSampleBufferAttachmentDescriptorArrayRef {}
unsafe impl Sync for MTLBlitPassSampleBufferAttachmentDescriptorArrayRef {}

unsafe impl ForeignType for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    type CType = Object;
    type Ref = MTLBlitPassSampleBufferAttachmentDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBlitPassSampleBufferAttachmentDescriptorArray {
        MTLBlitPassSampleBufferAttachmentDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBlitPassSampleBufferAttachmentDescriptorArrayRef> for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    fn as_ref(&self) -> &MTLBlitPassSampleBufferAttachmentDescriptorArrayRef {
        unsafe { &*(self.as_ptr().cast::<MTLBlitPassSampleBufferAttachmentDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLBlitPassSampleBufferAttachmentDescriptorArray {}
unsafe impl Sync for MTLBlitPassSampleBufferAttachmentDescriptorArray {}

unsafe impl objc::Message for MTLBlitPassSampleBufferAttachmentDescriptorArrayRef {}

impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
    /// Returns the object at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> MTLBlitPassSampleBufferAttachmentDescriptor {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), objectAtIndexedSubscript:index];
            MTLBlitPassSampleBufferAttachmentDescriptor::from_ptr(ptr)
        }
    }
    
    /// Sets the object at the specified index.
    pub fn set_object(&self, attachment: &MTLBlitPassSampleBufferAttachmentDescriptor, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setObject:attachment.as_ptr() atIndexedSubscript:index];
        }
    }
}

impl fmt::Debug for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLBlitPassSampleBufferAttachmentDescriptorArray")
            .finish()
    }
}

impl Drop for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.as_ptr(), release];
        }
    }
}

impl Clone for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.as_ptr(), retain];
            Self::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLBlitPassDescriptor`.
pub struct MTLBlitPassDescriptorRef(Object);

/// An owned Objective-C `MTLBlitPassDescriptor`.
pub struct MTLBlitPassDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLBlitPassDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLBlitPassDescriptorRef {}
unsafe impl Sync for MTLBlitPassDescriptorRef {}

unsafe impl ForeignType for MTLBlitPassDescriptor {
    type CType = Object;
    type Ref = MTLBlitPassDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBlitPassDescriptor {
        MTLBlitPassDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBlitPassDescriptorRef> for MTLBlitPassDescriptor {
    fn as_ref(&self) -> &MTLBlitPassDescriptorRef {
        unsafe { &*(self.as_ptr().cast::<MTLBlitPassDescriptorRef>()) }
    }
}

unsafe impl Send for MTLBlitPassDescriptor {}
unsafe impl Sync for MTLBlitPassDescriptor {}

unsafe impl objc::Message for MTLBlitPassDescriptorRef {}

impl MTLBlitPassDescriptor {
    /// Creates a new blit pass descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLBlitPassDescriptor);
            let ptr: *mut Object = msg_send![class, blitPassDescriptor];
            Self::from_ptr(ptr)
        }
    }
    
    /// Returns the sample buffer attachments array.
    #[must_use]
    pub fn sample_buffer_attachments(&self) -> MTLBlitPassSampleBufferAttachmentDescriptorArray {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), sampleBufferAttachments];
            MTLBlitPassSampleBufferAttachmentDescriptorArray::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLBlitPassDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLBlitPassDescriptor")
            .finish()
    }
}

impl Drop for MTLBlitPassDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.as_ptr(), release];
        }
    }
}

impl Clone for MTLBlitPassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ptr(), copy];
            Self::from_ptr(ptr)
        }
    }
}