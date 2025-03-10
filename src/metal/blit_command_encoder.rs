//! MTLBlitCommandEncoder - A Rust wrapper around Metal's MTLBlitCommandEncoder class.
//!
//! This module provides safe Rust bindings to the MTLBlitCommandEncoder class from Apple's Metal framework.
//! MTLBlitCommandEncoder is used to copy data between buffers and textures, fill buffers with a value,
//! generate mipmaps, and perform other memory operations.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLSize, MTLOrigin, MTLBlitOption};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue
//! let command_queue = device.new_command_queue();
//! 
//! // Create a command buffer
//! let command_buffer = command_queue.new_command_buffer();
//! 
//! // Create a blit command encoder
//! let blit_encoder = command_buffer.new_blit_command_encoder();
//! 
//! // Create source and destination buffers
//! let source_buffer = device.new_buffer(1024, MTLResourceOptions::StorageModeShared);
//! let dest_buffer = device.new_buffer(1024, MTLResourceOptions::StorageModeShared);
//! 
//! // Copy data from source to destination
//! blit_encoder.copy_from_buffer(
//!     &source_buffer, 
//!     0, 
//!     &dest_buffer, 
//!     0, 
//!     1024
//! );
//! 
//! // End encoding
//! blit_encoder.end_encoding();
//! 
//! // Commit the command buffer
//! command_buffer.commit();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSRange;
use crate::metal::{MTLBuffer, MTLBufferRef};
use crate::metal::MTLTexture;
use crate::metal::texture::{MTLSize, MTLOrigin};

/// MTLBlitOption - Options for blit operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u64)]
pub enum MTLBlitOption {
    None = 0,
    DepthFromDepthStencil = 1,
    StencilFromDepthStencil = 2,
    RowLinearPVRTC = 4,
}

impl From<MTLBlitOption> for u64 {
    fn from(option: MTLBlitOption) -> Self {
        option as u64
    }
}

/// A reference to an Objective-C `MTLBlitCommandEncoder`.
pub struct MTLBlitCommandEncoderRef(Object);

/// An owned Objective-C `MTLBlitCommandEncoder`.
pub struct MTLBlitCommandEncoder(*mut Object);

unsafe impl ForeignTypeRef for MTLBlitCommandEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLBlitCommandEncoderRef {}
unsafe impl Sync for MTLBlitCommandEncoderRef {}

unsafe impl ForeignType for MTLBlitCommandEncoder {
    type CType = Object;
    type Ref = MTLBlitCommandEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBlitCommandEncoder {
        MTLBlitCommandEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBlitCommandEncoderRef> for MTLBlitCommandEncoder {
    fn as_ref(&self) -> &MTLBlitCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLBlitCommandEncoderRef>()) }
    }
}

unsafe impl Send for MTLBlitCommandEncoder {}
unsafe impl Sync for MTLBlitCommandEncoder {}

unsafe impl objc::Message for MTLBlitCommandEncoderRef {}

impl MTLBlitCommandEncoder {
    /// End encoding of the current command encoder.
    pub fn end_encoding(&self) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, endEncoding]
        }
    }
    
    /// Synchronize the contents of a resource.
    pub fn synchronize_resource(&self, resource: &impl AsRef<MTLBufferRef>) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, synchronizeResource:resource.as_ref().as_ptr()]
        }
    }
    
    /// Synchronize a specific slice and level of a texture.
    pub fn synchronize_texture(&self, texture: &MTLTexture, slice: u64, level: u64) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, synchronizeTexture:texture.as_ptr()
                                          slice:slice
                                          level:level]
        }
    }
    
    /// Copy from a texture to another texture.
    pub fn copy_from_texture_to_texture(
        &self,
        source_texture: &MTLTexture,
        source_slice: u64,
        source_level: u64,
        source_origin: MTLOrigin,
        source_size: MTLSize,
        destination_texture: &MTLTexture,
        destination_slice: u64,
        destination_level: u64,
        destination_origin: MTLOrigin
    ) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, copyFromTexture:source_texture.as_ptr()
                                      sourceSlice:source_slice
                                      sourceLevel:source_level
                                    sourceOrigin:source_origin
                                      sourceSize:source_size
                                       toTexture:destination_texture.as_ptr()
                                destinationSlice:destination_slice
                                destinationLevel:destination_level
                              destinationOrigin:destination_origin]
        }
    }
    
    /// Simple copy from one texture to another.
    pub fn copy_from_texture(&self, source_texture: &MTLTexture, destination_texture: &MTLTexture) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, copyFromTexture:source_texture.as_ptr()
                                       toTexture:destination_texture.as_ptr()]
        }
    }
    
    /// Copy data from a buffer to a texture.
    pub fn copy_from_buffer_to_texture(
        &self,
        source_buffer: &MTLBuffer,
        source_offset: u64,
        source_bytes_per_row: u64,
        source_bytes_per_image: u64,
        source_size: MTLSize,
        destination_texture: &MTLTexture,
        destination_slice: u64,
        destination_level: u64,
        destination_origin: MTLOrigin
    ) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, copyFromBuffer:source_buffer.as_ptr()
                                     sourceOffset:source_offset
                               sourceBytesPerRow:source_bytes_per_row
                            sourceBytesPerImage:source_bytes_per_image
                                     sourceSize:source_size
                                      toTexture:destination_texture.as_ptr()
                               destinationSlice:destination_slice
                               destinationLevel:destination_level
                             destinationOrigin:destination_origin]
        }
    }
    
    /// Copy data from a buffer to a texture with additional options.
    pub fn copy_from_buffer_to_texture_with_options(
        &self,
        source_buffer: &MTLBuffer,
        source_offset: u64,
        source_bytes_per_row: u64,
        source_bytes_per_image: u64,
        source_size: MTLSize,
        destination_texture: &MTLTexture,
        destination_slice: u64,
        destination_level: u64,
        destination_origin: MTLOrigin,
        options: MTLBlitOption
    ) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            let options_val: u64 = options.into();
            msg_send![encoder_ref, copyFromBuffer:source_buffer.as_ptr()
                                     sourceOffset:source_offset
                               sourceBytesPerRow:source_bytes_per_row
                            sourceBytesPerImage:source_bytes_per_image
                                     sourceSize:source_size
                                      toTexture:destination_texture.as_ptr()
                               destinationSlice:destination_slice
                               destinationLevel:destination_level
                             destinationOrigin:destination_origin
                                       options:options_val]
        }
    }
    
    /// Copy data from a texture to a buffer.
    pub fn copy_from_texture_to_buffer(
        &self,
        source_texture: &MTLTexture,
        source_slice: u64,
        source_level: u64,
        source_origin: MTLOrigin,
        source_size: MTLSize,
        destination_buffer: &MTLBuffer,
        destination_offset: u64,
        destination_bytes_per_row: u64,
        destination_bytes_per_image: u64
    ) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, copyFromTexture:source_texture.as_ptr()
                                      sourceSlice:source_slice
                                      sourceLevel:source_level
                                    sourceOrigin:source_origin
                                      sourceSize:source_size
                                        toBuffer:destination_buffer.as_ptr()
                               destinationOffset:destination_offset
                         destinationBytesPerRow:destination_bytes_per_row
                      destinationBytesPerImage:destination_bytes_per_image]
        }
    }
    
    /// Copy data from a texture to a buffer with additional options.
    pub fn copy_from_texture_to_buffer_with_options(
        &self,
        source_texture: &MTLTexture,
        source_slice: u64,
        source_level: u64,
        source_origin: MTLOrigin,
        source_size: MTLSize,
        destination_buffer: &MTLBuffer,
        destination_offset: u64,
        destination_bytes_per_row: u64,
        destination_bytes_per_image: u64,
        options: MTLBlitOption
    ) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            let options_val: u64 = options.into();
            msg_send![encoder_ref, copyFromTexture:source_texture.as_ptr()
                                      sourceSlice:source_slice
                                      sourceLevel:source_level
                                    sourceOrigin:source_origin
                                      sourceSize:source_size
                                        toBuffer:destination_buffer.as_ptr()
                               destinationOffset:destination_offset
                         destinationBytesPerRow:destination_bytes_per_row
                      destinationBytesPerImage:destination_bytes_per_image
                                       options:options_val]
        }
    }
    
    /// Generate mipmaps for a texture.
    pub fn generate_mipmaps(&self, texture: &MTLTexture) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, generateMipmapsForTexture:texture.as_ptr()]
        }
    }
    
    /// Fill a buffer with a value.
    pub fn fill_buffer(&self, buffer: &MTLBuffer, range: NSRange, value: u8) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, fillBuffer:buffer.as_ptr()
                                        range:range
                                        value:value]
        }
    }
    
    /// Copy from one buffer to another.
    pub fn copy_from_buffer(
        &self,
        source_buffer: &MTLBuffer,
        source_offset: u64,
        destination_buffer: &MTLBuffer,
        destination_offset: u64,
        size: u64
    ) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, copyFromBuffer:source_buffer.as_ptr()
                                     sourceOffset:source_offset
                                         toBuffer:destination_buffer.as_ptr()
                               destinationOffset:destination_offset
                                            size:size]
        }
    }
    
    /// Optimize texture contents for GPU access.
    pub fn optimize_contents_for_gpu_access(&self, texture: &MTLTexture) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, optimizeContentsForGPUAccess:texture.as_ptr()]
        }
    }
    
    /// Optimize texture contents for CPU access.
    pub fn optimize_contents_for_cpu_access(&self, texture: &MTLTexture) {
        unsafe {
            let encoder_ref: &MTLBlitCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, optimizeContentsForCPUAccess:texture.as_ptr()]
        }
    }
}

// MTLBlitCommandEncoder already has an end_encoding method which matches the behavior
// of the base MTLCommandEncoder.

impl fmt::Debug for MTLBlitCommandEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLBlitCommandEncoder")
            .finish()
    }
}

impl Drop for MTLBlitCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLBlitCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLBlitCommandEncoder::from_ptr(obj)
        }
    }
}