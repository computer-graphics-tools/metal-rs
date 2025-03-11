//! MTLCommandEncoder - A Rust wrapper around Metal's MTLCommandEncoder protocol.
//!
//! This module provides safe Rust bindings to the MTLCommandEncoder protocol from Apple's Metal framework.
//! MTLCommandEncoder is the base protocol for objects that encode GPU commands into a command buffer.
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
//! // In a real implementation, you'd create specialized encoders like:
//! // let render_encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
//! // render_encoder.set_label("My Render Encoder");
//! // render_encoder.end_encoding();
//! 
//! command_buffer.commit();
//! ```

/// Trait for common command encoder functionality.
/// 
/// This trait defines the common methods that all command encoders share.
pub trait CommandEncoder {
    /// Returns the label of the command encoder.
    fn label(&self) -> Option<String>;
    
    /// Sets the label of the command encoder.
    fn set_label(&self, label: &str);
    
    /// Returns the device associated with this encoder.
    fn device(&self) -> crate::metal::MTLDevice;
    
    /// Marks the end of encoding commands into the command buffer.
    fn end_encoding(&self);
    
    /// Marks the place in the command encoder when debugging.
    fn push_debug_group(&self, name: &str);
    
    /// Removes the most recently added debug group marker.
    fn pop_debug_group(&self);
    
    /// Inserts a debug marker into the command encoder.
    fn insert_debug_signpost(&self, name: &str);
    
    /// Signals an event with a value when the command buffer reaches this point in the encoder.
    fn signal_event(&self, event: &impl AsRef<crate::metal::event::MTLEventRef>, value: u64);
    
    /// Makes the GPU wait for an event to reach a value before executing further commands.
    fn wait_for_event(&self, event: &impl AsRef<crate::metal::event::MTLEventRef>, value: u64);
}

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::event::MTLEventRef;

/// A reference to an Objective-C `MTLCommandEncoder`.
pub struct MTLCommandEncoderRef(Object);

/// An owned Objective-C `MTLCommandEncoder`.
pub struct MTLCommandEncoder(*mut Object);

unsafe impl ForeignTypeRef for MTLCommandEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLCommandEncoderRef {}
unsafe impl Sync for MTLCommandEncoderRef {}

unsafe impl ForeignType for MTLCommandEncoder {
    type CType = Object;
    type Ref = MTLCommandEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCommandEncoder {
        MTLCommandEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCommandEncoderRef> for MTLCommandEncoder {
    fn as_ref(&self) -> &MTLCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLCommandEncoderRef>()) }
    }
}

unsafe impl Send for MTLCommandEncoder {}
unsafe impl Sync for MTLCommandEncoder {}

unsafe impl objc::Message for MTLCommandEncoderRef {}

// Implement the CommandEncoder trait for MTLCommandEncoder
impl CommandEncoder for MTLCommandEncoder {
    fn label(&self) -> Option<String> {
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
    
    fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    fn device(&self) -> crate::metal::MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            crate::metal::MTLDevice::from_ptr(ptr)
        }
    }
    
    fn push_debug_group(&self, name: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            let _: () = msg_send![self.as_ref(), pushDebugGroup:ns_string.as_ptr()];
        }
    }
    
    fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), popDebugGroup];
        }
    }
    
    fn insert_debug_signpost(&self, name: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            let _: () = msg_send![self.as_ref(), insertDebugSignpost:ns_string.as_ptr()];
        }
    }
    
    fn signal_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), signalEvent:event.as_ref().as_ptr() value:value];
        }
    }
    
    fn wait_for_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), waitForEvent:event.as_ref().as_ptr() value:value];
        }
    }
    
    fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), endEncoding];
        }
    }
}

// Also provide the methods as regular associated functions for backward compatibility
impl MTLCommandEncoder {
    /// Returns the label of the command encoder.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        <Self as CommandEncoder>::label(self)
    }
    
    /// Sets the label of the command encoder.
    pub fn set_label(&self, label: &str) {
        <Self as CommandEncoder>::set_label(self, label)
    }
    
    /// Returns the device associated with this encoder.
    #[must_use]
    pub fn device(&self) -> crate::metal::MTLDevice {
        <Self as CommandEncoder>::device(self)
    }
    
    /// Marks the place in the command encoder when debugging.
    pub fn push_debug_group(&self, name: &str) {
        <Self as CommandEncoder>::push_debug_group(self, name)
    }
    
    /// Removes the most recently added debug group marker.
    pub fn pop_debug_group(&self) {
        <Self as CommandEncoder>::pop_debug_group(self)
    }
    
    /// Inserts a debug marker into the command encoder.
    pub fn insert_debug_signpost(&self, name: &str) {
        <Self as CommandEncoder>::insert_debug_signpost(self, name)
    }
    
    /// Marks the end of encoding commands into the command buffer.
    pub fn end_encoding(&self) {
        <Self as CommandEncoder>::end_encoding(self)
    }
    
    /// Signals an event with a value when the command buffer reaches this point in the encoder.
    pub fn signal_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        <Self as CommandEncoder>::signal_event(self, event, value)
    }
    
    /// Makes the GPU wait for an event to reach a value before executing further commands.
    pub fn wait_for_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        <Self as CommandEncoder>::wait_for_event(self, event, value)
    }
}

impl fmt::Debug for MTLCommandEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLCommandEncoder {{ label: {} }}", label)
    }
}

impl Drop for MTLCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCommandEncoder::from_ptr(obj)
        }
    }
}