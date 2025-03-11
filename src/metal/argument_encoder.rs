//! MTLArgumentEncoder - A Rust wrapper around Metal's MTLArgumentEncoder class.
//!
//! This module provides safe Rust bindings to the MTLArgumentEncoder class from Apple's Metal framework.
//! MTLArgumentEncoder is used to encode arguments into a buffer for indirect access in shaders.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLArgumentDescriptor, MTLDataType};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create an argument descriptor
//! let descriptor = MTLArgumentDescriptor::new();
//! descriptor.set_data_type(MTLDataType::Texture);
//! descriptor.set_index(0);
//! 
//! // Create an array of descriptors
//! let descriptors = metal_rs::foundation::NSArray::from_slice(&[&descriptor]);
//! 
//! // Create an argument encoder
//! let encoder = device.new_argument_encoder(&descriptors);
//! 
//! // Create a buffer for the encoded arguments
//! let buffer = device.new_buffer(encoder.encoded_length(), metal_rs::metal::MTLResourceOptions::StorageModeShared);
//! 
//! // Set the argument buffer and encode a texture
//! encoder.set_argument_buffer(&buffer, 0);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSString, NSUInteger, NSRange};
use crate::metal::device::MTLDeviceRef;
use crate::metal::{MTLBuffer, MTLBufferRef, MTLTexture, MTLTextureRef};
use crate::metal::sampler::{MTLSamplerState, MTLSamplerStateRef};
use crate::metal::render_pipeline::{MTLRenderPipelineState, MTLRenderPipelineStateRef};
use crate::metal::compute_command_encoder::MTLComputePipelineStateRef;

/// The constant used for attribute stride for vertex attribute layouts.
pub const ATTRIBUTE_STRIDE_STATIC: usize = usize::MAX;

/// A reference to an Objective-C `MTLArgumentEncoder`.
pub struct MTLArgumentEncoderRef(Object);

/// An owned Objective-C `MTLArgumentEncoder`.
pub struct MTLArgumentEncoder(*mut Object);

unsafe impl ForeignTypeRef for MTLArgumentEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLArgumentEncoderRef {}
unsafe impl Sync for MTLArgumentEncoderRef {}

unsafe impl ForeignType for MTLArgumentEncoder {
    type CType = Object;
    type Ref = MTLArgumentEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLArgumentEncoder {
        MTLArgumentEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLArgumentEncoderRef> for MTLArgumentEncoder {
    fn as_ref(&self) -> &MTLArgumentEncoderRef {
        unsafe { &*(self.0.cast::<MTLArgumentEncoderRef>()) }
    }
}

unsafe impl Send for MTLArgumentEncoder {}
unsafe impl Sync for MTLArgumentEncoder {}

unsafe impl objc::Message for MTLArgumentEncoderRef {}

impl MTLArgumentEncoder {
    /// Returns the device that created this argument encoder.
    #[must_use]
    pub fn device(&self) -> &MTLDeviceRef {
        unsafe {
            let device: *mut Object = msg_send![self.as_ref(), device];
            &*(device as *mut MTLDeviceRef)
        }
    }

    /// Gets the label for this argument encoder.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), label];
            if ns_string.is_null() {
                return String::new();
            }
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }

    /// Sets the label for this argument encoder.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            msg_send![self.as_ref(), setLabel:ns_string.as_ptr()]
        }
    }

    /// Gets the encoded length of the argument buffer.
    #[must_use]
    pub fn encoded_length(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), encodedLength]
        }
    }

    /// Gets the alignment of the argument buffer.
    #[must_use]
    pub fn alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), alignment]
        }
    }

    /// Sets the argument buffer for encoding.
    pub fn set_argument_buffer(&self, argument_buffer: &MTLBuffer, offset: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setArgumentBuffer:argument_buffer.as_ptr() offset:offset]
        }
    }

    /// Sets the argument buffer for encoding with array element.
    pub fn set_argument_buffer_with_array_element(&self, argument_buffer: &MTLBuffer, start_offset: NSUInteger, array_element: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setArgumentBuffer:argument_buffer.as_ptr() startOffset:start_offset arrayElement:array_element]
        }
    }

    /// Sets a buffer in the argument buffer.
    pub fn set_buffer(&self, buffer: &MTLBuffer, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setBuffer:buffer.as_ptr() offset:offset atIndex:index]
        }
    }

    /// Sets multiple buffers in the argument buffer.
    pub fn set_buffers(&self, buffers: &[&MTLBufferRef], offsets: &[NSUInteger], range: NSRange) {
        unsafe {
            let buffers_ptr = buffers.as_ptr() as *const *const Object;
            let offsets_ptr = offsets.as_ptr();
            msg_send![self.as_ref(), setBuffers:buffers_ptr offsets:offsets_ptr withRange:range]
        }
    }

    /// Sets a texture in the argument buffer.
    pub fn set_texture(&self, texture: &MTLTexture, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setTexture:texture.as_ptr() atIndex:index]
        }
    }

    /// Sets multiple textures in the argument buffer.
    pub fn set_textures(&self, textures: &[&MTLTextureRef], range: NSRange) {
        unsafe {
            let textures_ptr = textures.as_ptr() as *const *const Object;
            msg_send![self.as_ref(), setTextures:textures_ptr withRange:range]
        }
    }

    /// Sets a sampler state in the argument buffer.
    pub fn set_sampler_state(&self, sampler: &MTLSamplerState, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setSamplerState:sampler.as_ptr() atIndex:index]
        }
    }

    /// Sets multiple sampler states in the argument buffer.
    pub fn set_sampler_states(&self, samplers: &[&MTLSamplerStateRef], range: NSRange) {
        unsafe {
            let samplers_ptr = samplers.as_ptr() as *const *const Object;
            msg_send![self.as_ref(), setSamplerStates:samplers_ptr withRange:range]
        }
    }

    /// Gets a pointer to constant data at the specified index.
    #[must_use]
    pub fn constant_data(&self, index: NSUInteger) -> *mut std::ffi::c_void {
        unsafe {
            msg_send![self.as_ref(), constantDataAtIndex:index]
        }
    }

    /// Sets a render pipeline state in the argument buffer.
    pub fn set_render_pipeline_state(&self, pipeline: &MTLRenderPipelineState, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setRenderPipelineState:pipeline.as_ptr() atIndex:index]
        }
    }

    /// Sets multiple render pipeline states in the argument buffer.
    pub fn set_render_pipeline_states(&self, pipelines: &[&MTLRenderPipelineStateRef], range: NSRange) {
        unsafe {
            let pipelines_ptr = pipelines.as_ptr() as *const *const Object;
            msg_send![self.as_ref(), setRenderPipelineStates:pipelines_ptr withRange:range]
        }
    }

    /// Sets a compute pipeline state in the argument buffer.
    pub fn set_compute_pipeline_state(&self, pipeline: &impl AsRef<MTLComputePipelineStateRef>, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setComputePipelineState:pipeline.as_ref().as_ptr() atIndex:index]
        }
    }

    /// Creates a new argument encoder for a nested buffer at the specified index.
    #[must_use]
    pub fn new_argument_encoder_for_buffer(&self, index: NSUInteger) -> MTLArgumentEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newArgumentEncoderForBufferAtIndex:index];
            MTLArgumentEncoder::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLArgumentEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLArgumentEncoder")
            .field("label", &self.label())
            .field("encoded_length", &self.encoded_length())
            .field("alignment", &self.alignment())
            .finish()
    }
}

impl Drop for MTLArgumentEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLArgumentEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLArgumentEncoder::from_ptr(obj)
        }
    }
}