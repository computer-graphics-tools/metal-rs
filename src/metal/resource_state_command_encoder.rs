//! MTLResourceStateCommandEncoder - A Rust wrapper around Metal's MTLResourceStateCommandEncoder class.
//!
//! This module provides safe Rust bindings to the MTLResourceStateCommandEncoder class from Apple's Metal framework.
//! MTLResourceStateCommandEncoder is used to manage resource state transitions and sparse texture mappings.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLSparseTextureMappingMode, MTLRegion, MTLSize, MTLOrigin};
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
//! // Create a resource state command encoder
//! let resource_state_encoder = command_buffer.new_resource_state_command_encoder();
//! 
//! // Create a sparse texture
//! let texture_desc = device.new_texture_descriptor();
//! texture_desc.set_texture_type(MTLTextureType::Type2D);
//! texture_desc.set_width(1024);
//! texture_desc.set_height(1024);
//! texture_desc.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
//! texture_desc.set_storage_mode(MTLStorageMode::Private);
//! texture_desc.set_usage(MTLTextureUsage::ShaderRead);
//! texture_desc.set_sparse(true);
//! 
//! let sparse_texture = device.new_texture(&texture_desc);
//! 
//! // Define a region to map
//! let region = MTLRegion {
//!     origin: MTLOrigin { x: 0, y: 0, z: 0 },
//!     size: MTLSize { width: 64, height: 64, depth: 1 }
//! };
//! 
//! // Map a tile of the sparse texture
//! resource_state_encoder.update_texture_mapping(
//!     &sparse_texture,
//!     MTLSparseTextureMappingMode::Map,
//!     &region,
//!     0, // mipmap level
//!     0, // slice
//! );
//! 
//! // End encoding
//! resource_state_encoder.end_encoding();
//! 
//! // Commit the command buffer
//! command_buffer.commit();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSUInteger, NSString};
use crate::metal::{MTLTextureRef, MTLBufferRef};
use crate::metal::texture::{MTLRegion, MTLOrigin, MTLSize};
use crate::metal::fence::MTLFenceRef;
use crate::metal::command_encoder::{MTLCommandEncoderRef, CommandEncoder};
use crate::metal::event::MTLEventRef;
use crate::metal::MTLDevice;

/// Specifies whether to map or unmap sparse texture tiles.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLSparseTextureMappingMode {
    /// Map sparse texture tiles.
    Map = 0,
    /// Unmap sparse texture tiles.
    Unmap = 1,
}

/// Arguments structure for indirect sparse texture region mapping.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSparseTextureRegionUpdateIndirectArguments {
    /// The region to update.
    pub region: MTLRegion,
    /// The mipmap level to update.
    pub mip_map_level: NSUInteger,
    /// The slice to update.
    pub slice: NSUInteger,
    /// The sparse texture mapping mode.
    pub mapping_mode: MTLSparseTextureMappingMode,
}

/// A reference to an Objective-C `MTLResourceStateCommandEncoder`.
pub struct MTLResourceStateCommandEncoderRef(Object);

/// An owned Objective-C `MTLResourceStateCommandEncoder`.
pub struct MTLResourceStateCommandEncoder(*mut Object);

unsafe impl ForeignTypeRef for MTLResourceStateCommandEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLResourceStateCommandEncoderRef {}
unsafe impl Sync for MTLResourceStateCommandEncoderRef {}

unsafe impl ForeignType for MTLResourceStateCommandEncoder {
    type CType = Object;
    type Ref = MTLResourceStateCommandEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLResourceStateCommandEncoder {
        MTLResourceStateCommandEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLResourceStateCommandEncoderRef> for MTLResourceStateCommandEncoder {
    fn as_ref(&self) -> &MTLResourceStateCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLResourceStateCommandEncoderRef>()) }
    }
}

impl AsRef<MTLCommandEncoderRef> for MTLResourceStateCommandEncoder {
    fn as_ref(&self) -> &MTLCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLCommandEncoderRef>()) }
    }
}

unsafe impl Send for MTLResourceStateCommandEncoder {}
unsafe impl Sync for MTLResourceStateCommandEncoder {}

unsafe impl objc::Message for MTLResourceStateCommandEncoderRef {}

// Implement the CommandEncoder trait for MTLResourceStateCommandEncoder
impl CommandEncoder for MTLResourceStateCommandEncoder {
    fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref() as &MTLCommandEncoderRef, label];
            if label.is_null() {
                None
            } else {
                Some(NSString::from_ptr(label).to_string())
            }
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref() as &MTLCommandEncoderRef, setLabel:ns_string.as_ptr()];
        }
    }

    fn device(&self) -> MTLDevice {
        unsafe {
            let device: *mut Object = msg_send![self.as_ref() as &MTLCommandEncoderRef, device];
            MTLDevice::from_ptr(device)
        }
    }

    fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref() as &MTLCommandEncoderRef, endEncoding];
        }
    }

    fn push_debug_group(&self, name: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            let _: () = msg_send![self.as_ref() as &MTLCommandEncoderRef, pushDebugGroup:ns_string.as_ptr()];
        }
    }

    fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref() as &MTLCommandEncoderRef, popDebugGroup];
        }
    }

    fn insert_debug_signpost(&self, name: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            let _: () = msg_send![self.as_ref() as &MTLCommandEncoderRef, insertDebugSignpost:ns_string.as_ptr()];
        }
    }

    fn signal_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref() as &MTLCommandEncoderRef, signalEvent:event.as_ref().as_ptr() value:value];
        }
    }

    fn wait_for_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref() as &MTLCommandEncoderRef, waitForEvent:event.as_ref().as_ptr() value:value];
        }
    }
}

impl MTLResourceStateCommandEncoder {
    /// Updates the fence to establish a dependency between this command encoder and subsequent command encoders.
    pub fn update_fence(&self, fence: &impl AsRef<MTLFenceRef>) {
        unsafe {
            let encoder_ref: &MTLResourceStateCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, updateFence:fence.as_ref().as_ptr()];
        }
    }
    
    /// Waits for the specified fence before executing subsequent commands.
    pub fn wait_for_fence(&self, fence: &impl AsRef<MTLFenceRef>) {
        unsafe {
            let encoder_ref: &MTLResourceStateCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, waitForFence:fence.as_ref().as_ptr()];
        }
    }
    
    /// Updates mappings for a sparse texture.
    pub fn update_texture_mapping(
        &self,
        texture: &impl AsRef<MTLTextureRef>,
        mapping_mode: MTLSparseTextureMappingMode,
        region: &MTLRegion,
        mipmap_level: NSUInteger,
        slice: NSUInteger
    ) {
        unsafe {
            let encoder_ref: &MTLResourceStateCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, updateTextureMapping:texture.as_ref().as_ptr()
                                                            mode:mapping_mode
                                                          region:*region
                                                    mipMapLevel:mipmap_level
                                                          slice:slice];
        }
    }
    
    /// Updates mappings for a sparse texture using indirect arguments buffer.
    pub fn update_texture_mapping_indirect(
        &self,
        texture: &impl AsRef<MTLTextureRef>,
        indirect_arguments_buffer: &impl AsRef<MTLBufferRef>,
        indirect_arguments_offset: NSUInteger
    ) {
        unsafe {
            let encoder_ref: &MTLResourceStateCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, updateTextureMappings:texture.as_ref().as_ptr()
                                           indirectArgumentsBuffer:indirect_arguments_buffer.as_ref().as_ptr()
                                           indirectArgumentsOffset:indirect_arguments_offset];
        }
    }
    
    /// Updates multiple regions for a sparse texture.
    pub fn update_texture_mappings(
        &self,
        texture: &impl AsRef<MTLTextureRef>,
        mapping_mode: MTLSparseTextureMappingMode,
        regions: &[MTLRegion],
        mipmap_levels: &[NSUInteger],
        slices: &[NSUInteger],
        num_regions: NSUInteger
    ) {
        unsafe {
            let encoder_ref: &MTLResourceStateCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, updateTextureMappings:texture.as_ref().as_ptr()
                                                              mode:mapping_mode
                                                            regions:regions.as_ptr()
                                                         mipMapLevels:mipmap_levels.as_ptr()
                                                              slices:slices.as_ptr()
                                                         numRegions:num_regions];
        }
    }
    
    /// Moves texture mappings from one texture to another.
    pub fn move_texture_mappings_from_texture(
        &self,
        source_texture: &impl AsRef<MTLTextureRef>,
        source_slice: NSUInteger,
        source_level: NSUInteger,
        source_origin: MTLOrigin,
        destination_texture: &impl AsRef<MTLTextureRef>,
        destination_slice: NSUInteger,
        destination_level: NSUInteger,
        destination_origin: MTLOrigin,
        size: MTLSize
    ) {
        unsafe {
            let encoder_ref: &MTLResourceStateCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, moveTextureMappingsFromTexture:source_texture.as_ref().as_ptr()
                                                                 sourceSlice:source_slice
                                                                 sourceLevel:source_level
                                                               sourceOrigin:source_origin
                                                                  toTexture:destination_texture.as_ref().as_ptr()
                                                           destinationSlice:destination_slice
                                                           destinationLevel:destination_level
                                                         destinationOrigin:destination_origin
                                                                     size:size];
        }
    }
}

impl fmt::Debug for MTLResourceStateCommandEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLResourceStateCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

impl Drop for MTLResourceStateCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLResourceStateCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLResourceStateCommandEncoder::from_ptr(obj)
        }
    }
}