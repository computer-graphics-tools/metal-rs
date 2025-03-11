//! MTLParallelRenderCommandEncoder - A Rust wrapper around Metal's parallel render command encoder.
//!
//! This module provides safe Rust bindings to the parallel render command encoder class from Apple's Metal framework,
//! which allows for creating multiple render command encoders that can execute in parallel on multiple CPU threads.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLLoadAction, MTLStoreAction, MTLClearColor};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue and command buffer
//! let command_queue = device.new_command_queue();
//! let command_buffer = command_queue.new_command_buffer();
//! 
//! // Set up a render pass descriptor
//! let render_pass_descriptor = metal_rs::metal::MTLRenderPassDescriptor::new();
//! let color_attachment = render_pass_descriptor.color_attachments().object(0);
//! color_attachment.set_texture(&texture); // texture would be from a CAMetalDrawable or created separately
//! color_attachment.set_load_action(MTLLoadAction::Clear);
//! color_attachment.set_store_action(MTLStoreAction::Store);
//! color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
//! 
//! // Create the parallel render command encoder
//! let parallel_encoder = command_buffer.new_parallel_render_command_encoder(&render_pass_descriptor);
//! 
//! // Create individual render command encoders for parallel work
//! let encoder1 = parallel_encoder.render_command_encoder();
//! let encoder2 = parallel_encoder.render_command_encoder();
//! 
//! // Use each encoder to render different parts of the scene
//! // ...
//! 
//! // End encoding and commit the command buffer
//! encoder1.end_encoding();
//! encoder2.end_encoding();
//! parallel_encoder.end_encoding();
//! command_buffer.commit();
//! ```

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::command_encoder::CommandEncoder;
use crate::metal::render_command_encoder::MTLRenderCommandEncoder;
use crate::metal::render_pass::{MTLStoreAction, MTLStoreActionOptions};

/// A reference to an Objective-C `MTLParallelRenderCommandEncoder`.
pub struct MTLParallelRenderCommandEncoderRef(Object);

/// An owned Objective-C `MTLParallelRenderCommandEncoder`.
pub struct MTLParallelRenderCommandEncoder(*mut Object);

// Implementation for MTLParallelRenderCommandEncoder
unsafe impl ForeignTypeRef for MTLParallelRenderCommandEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLParallelRenderCommandEncoderRef {}
unsafe impl Sync for MTLParallelRenderCommandEncoderRef {}

unsafe impl ForeignType for MTLParallelRenderCommandEncoder {
    type CType = Object;
    type Ref = MTLParallelRenderCommandEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLParallelRenderCommandEncoder {
        MTLParallelRenderCommandEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLParallelRenderCommandEncoderRef> for MTLParallelRenderCommandEncoder {
    fn as_ref(&self) -> &MTLParallelRenderCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLParallelRenderCommandEncoderRef>()) }
    }
}

// Removed the MTLCommandEncoderRef AsRef implementation to avoid ambiguity

unsafe impl Send for MTLParallelRenderCommandEncoder {}
unsafe impl Sync for MTLParallelRenderCommandEncoder {}

unsafe impl objc::Message for MTLParallelRenderCommandEncoderRef {}

impl Drop for MTLParallelRenderCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLParallelRenderCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLParallelRenderCommandEncoder::from_ptr(obj)
        }
    }
}

impl std::fmt::Debug for MTLParallelRenderCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MTLParallelRenderCommandEncoder")
            .finish()
    }
}

impl MTLParallelRenderCommandEncoder {
    /// Creates a new render command encoder for encoding commands in parallel.
    ///
    /// # Returns
    ///
    /// A new render command encoder.
    #[must_use]
    pub fn render_command_encoder(&self) -> MTLRenderCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), renderCommandEncoder];
            MTLRenderCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Sets the store action for a color attachment.
    ///
    /// # Arguments
    ///
    /// * `store_action` - The store action to use.
    /// * `color_attachment_index` - The index of the color attachment.
    pub fn set_color_store_action(&self, store_action: MTLStoreAction, color_attachment_index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setColorStoreAction:store_action
                                                   atIndex:color_attachment_index];
        }
    }
    
    /// Sets the store action for the depth attachment.
    ///
    /// # Arguments
    ///
    /// * `store_action` - The store action to use.
    pub fn set_depth_store_action(&self, store_action: MTLStoreAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDepthStoreAction:store_action];
        }
    }
    
    /// Sets the store action for the stencil attachment.
    ///
    /// # Arguments
    ///
    /// * `store_action` - The store action to use.
    pub fn set_stencil_store_action(&self, store_action: MTLStoreAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStencilStoreAction:store_action];
        }
    }
    
    /// Sets the store action options for a color attachment.
    ///
    /// # Arguments
    ///
    /// * `store_action_options` - The store action options to use.
    /// * `color_attachment_index` - The index of the color attachment.
    pub fn set_color_store_action_options(&self, store_action_options: MTLStoreActionOptions, color_attachment_index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setColorStoreActionOptions:store_action_options
                                                           atIndex:color_attachment_index];
        }
    }
    
    /// Sets the store action options for the depth attachment.
    ///
    /// # Arguments
    ///
    /// * `store_action_options` - The store action options to use.
    pub fn set_depth_store_action_options(&self, store_action_options: MTLStoreActionOptions) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDepthStoreActionOptions:store_action_options];
        }
    }
    
    /// Sets the store action options for the stencil attachment.
    ///
    /// # Arguments
    ///
    /// * `store_action_options` - The store action options to use.
    pub fn set_stencil_store_action_options(&self, store_action_options: MTLStoreActionOptions) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStencilStoreActionOptions:store_action_options];
        }
    }
}

// Implement CommandEncoder trait for MTLParallelRenderCommandEncoder
impl CommandEncoder for MTLParallelRenderCommandEncoder {
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
    
    fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), endEncoding];
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
}