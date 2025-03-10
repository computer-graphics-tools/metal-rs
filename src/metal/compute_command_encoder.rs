//! MTLComputeCommandEncoder - A Rust wrapper around Metal's MTLComputeCommandEncoder protocol.
//!
//! This module provides safe Rust bindings to the MTLComputeCommandEncoder protocol from Apple's Metal framework.
//! MTLComputeCommandEncoder is used to encode compute commands into a command buffer.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLSize};
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
//! // Create a compute command encoder
//! let compute_encoder = command_buffer.new_compute_command_encoder();
//! 
//! // Set compute pipeline state (if we had one)
//! // compute_encoder.set_compute_pipeline_state(&compute_pipeline_state);
//! 
//! // Set buffers, textures, etc.
//! 
//! // Dispatch compute work
//! let threads_per_threadgroup = MTLSize::new(16, 16, 1);
//! let threadgroups = MTLSize::new(8, 8, 1);
//! compute_encoder.dispatch_threadgroups(threadgroups, threads_per_threadgroup);
//! 
//! // End encoding and commit
//! compute_encoder.end_encoding();
//! command_buffer.commit();
//! ```

use std::fmt;
use std::os::raw::c_void;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};

use crate::foundation::{NSRange, NSUInteger};
use crate::metal::{
    MTLCommandEncoderRef,
    MTLBufferRef,
    MTLTextureRef,
    MTLSize, MTLRegion,
    MTLResource, MTLResourceRef
};

/// Dispatch type for compute command encoders.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u64)]
pub enum MTLDispatchType {
    /// Serial dispatch type.
    Serial = 0,
    /// Concurrent dispatch type.
    Concurrent = 1,
}

/// Resource usage options.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u64)]
pub enum MTLResourceUsage {
    /// Read access.
    Read = 1,
    /// Write access.
    Write = 2,
    /// Combined read and write access.
    ReadWrite = 3,
}

/// Memory barrier scope options.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u64)]
pub enum MTLBarrierScope {
    /// Buffers barrier scope.
    Buffers = 1,
    /// Textures barrier scope.
    Textures = 2,
    /// Combined buffers and textures barrier scope.
    RenderTargets = 4,
}

/// Arguments for dispatching threadgroups indirectly.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    /// Number of threadgroups in each dimension.
    pub threadgroups_per_grid: [u32; 3],
}

/// Arguments for stage in region indirect parameters.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct MTLStageInRegionIndirectArguments {
    /// The origin of the stage in region.
    pub stage_in_origin: [u32; 3],
    /// The size of the stage in region.
    pub stage_in_size: [u32; 3],
}

/// A reference to an Objective-C `MTLComputeCommandEncoder`.
pub struct MTLComputeCommandEncoderRef(Object);

/// An owned Objective-C `MTLComputeCommandEncoder`.
pub struct MTLComputeCommandEncoder(*mut Object);

unsafe impl ForeignTypeRef for MTLComputeCommandEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLComputeCommandEncoderRef {}
unsafe impl Sync for MTLComputeCommandEncoderRef {}

unsafe impl ForeignType for MTLComputeCommandEncoder {
    type CType = Object;
    type Ref = MTLComputeCommandEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLComputeCommandEncoder {
        MTLComputeCommandEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLComputeCommandEncoderRef> for MTLComputeCommandEncoder {
    fn as_ref(&self) -> &MTLComputeCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLComputeCommandEncoderRef>()) }
    }
}

impl AsRef<MTLCommandEncoderRef> for MTLComputeCommandEncoder {
    fn as_ref(&self) -> &MTLCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLCommandEncoderRef>()) }
    }
}

unsafe impl Send for MTLComputeCommandEncoder {}
unsafe impl Sync for MTLComputeCommandEncoder {}

unsafe impl objc::Message for MTLComputeCommandEncoderRef {}

impl MTLComputeCommandEncoder {
    /// Returns the dispatch type of the compute command encoder.
    #[must_use]
    pub fn dispatch_type(&self) -> MTLDispatchType {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            msg_send![encoder_ref, dispatchType]
        }
    }

    /// Sets the compute pipeline state object.
    pub fn set_compute_pipeline_state(&self, state: &impl AsRef<MTLComputePipelineStateRef>) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setComputePipelineState:state.as_ref().as_ptr()];
        }
    }

    /// Sets bytes directly for a compute function argument.
    pub fn set_bytes(&self, bytes: &[u8], index: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let bytes_ptr = bytes.as_ptr() as *const c_void;
            let bytes_len = bytes.len();
            let _: () = msg_send![encoder_ref, setBytes:bytes_ptr length:bytes_len atIndex:index];
        }
    }

    /// Sets a buffer for a compute function argument.
    pub fn set_buffer(&self, buffer: Option<&MTLBufferRef>, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let buffer_ptr = match buffer {
                Some(buffer) => buffer.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![encoder_ref, setBuffer:buffer_ptr offset:offset atIndex:index];
        }
    }

    /// Sets a buffer offset for a compute function argument.
    pub fn set_buffer_offset(&self, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setBufferOffset:offset atIndex:index];
        }
    }

    /// Sets an array of buffers for compute function arguments.
    pub fn set_buffers(&self, buffers: &[&MTLBufferRef], offsets: &[NSUInteger], range: NSRange) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setBuffers:buffers.as_ptr() offsets:offsets.as_ptr() withRange:range];
        }
    }

    /// Sets a buffer with stride for a compute function argument.
    pub fn set_buffer_with_stride(&self, buffer: Option<&MTLBufferRef>, offset: NSUInteger, stride: NSUInteger, index: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let buffer_ptr = match buffer {
                Some(buffer) => buffer.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![encoder_ref, setBuffer:buffer_ptr offset:offset attributeStride:stride atIndex:index];
        }
    }

    /// Sets a texture for a compute function argument.
    pub fn set_texture(&self, texture: Option<&MTLTextureRef>, index: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let texture_ptr = match texture {
                Some(texture) => texture.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![encoder_ref, setTexture:texture_ptr atIndex:index];
        }
    }

    /// Sets an array of textures for compute function arguments.
    pub fn set_textures(&self, textures: &[&MTLTextureRef], range: NSRange) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setTextures:textures.as_ptr() withRange:range];
        }
    }

    /// Sets the length of threadgroup memory.
    pub fn set_threadgroup_memory_length(&self, length: NSUInteger, index: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setThreadgroupMemoryLength:length atIndex:index];
        }
    }

    /// Sets the dimensions of an imageblock.
    pub fn set_imageblock_width(&self, width: NSUInteger, height: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setImageblockWidth:width height:height];
        }
    }

    /// Sets the stage-in region.
    pub fn set_stage_in_region(&self, region: MTLRegion) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setStageInRegion:region];
        }
    }

    /// Sets the stage-in region from an indirect buffer.
    pub fn set_stage_in_region_indirect(&self, indirect_buffer: &MTLBufferRef, offset: NSUInteger) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, setStageInRegionWithIndirectBuffer:indirect_buffer.as_ptr() indirectBufferOffset:offset];
        }
    }

    /// Dispatches threadgroups.
    pub fn dispatch_threadgroups(&self, threadgroups_per_grid: MTLSize, threads_per_threadgroup: MTLSize) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, dispatchThreadgroups:threadgroups_per_grid threadsPerThreadgroup:threads_per_threadgroup];
        }
    }

    /// Dispatches threadgroups from an indirect buffer.
    pub fn dispatch_threadgroups_indirect(&self, indirect_buffer: &MTLBufferRef, offset: NSUInteger, threads_per_threadgroup: MTLSize) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, dispatchThreadgroupsWithIndirectBuffer:indirect_buffer.as_ptr() indirectBufferOffset:offset threadsPerThreadgroup:threads_per_threadgroup];
        }
    }

    /// Dispatches threads directly.
    pub fn dispatch_threads(&self, threads_per_grid: MTLSize, threads_per_threadgroup: MTLSize) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, dispatchThreads:threads_per_grid threadsPerThreadgroup:threads_per_threadgroup];
        }
    }

    /// Creates a memory barrier.
    pub fn memory_barrier(&self, scope: MTLBarrierScope) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, memoryBarrierWithScope:scope];
        }
    }

    /// Creates a memory barrier for specific resources.
    pub fn memory_barrier_with_resources(&self, resources: &[&MTLResourceRef]) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let count = resources.len();
            let _: () = msg_send![encoder_ref, memoryBarrierWithResources:resources.as_ptr() count:count];
        }
    }

    /// Marks a resource as used.
    pub fn use_resource(&self, resource: &MTLResource, usage: MTLResourceUsage) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, useResource:resource.as_ptr() usage:usage];
        }
    }

    /// Marks multiple resources as used.
    pub fn use_resources(&self, resources: &[&MTLResource], usage: MTLResourceUsage) {
        unsafe {
            let encoder_ref: &MTLComputeCommandEncoderRef = self.as_ref();
            let count = resources.len();
            let _: () = msg_send![encoder_ref, useResources:resources.as_ptr() count:count usage:usage];
        }
    }
}


/// A reference to an Objective-C `MTLComputePipelineState`.
pub struct MTLComputePipelineStateRef(Object);

/// An owned Objective-C `MTLComputePipelineState`.
pub struct MTLComputePipelineState(*mut Object);

unsafe impl ForeignTypeRef for MTLComputePipelineStateRef {
    type CType = Object;
}

unsafe impl Send for MTLComputePipelineStateRef {}
unsafe impl Sync for MTLComputePipelineStateRef {}

unsafe impl ForeignType for MTLComputePipelineState {
    type CType = Object;
    type Ref = MTLComputePipelineStateRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLComputePipelineState {
        MTLComputePipelineState(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLComputePipelineStateRef> for MTLComputePipelineState {
    fn as_ref(&self) -> &MTLComputePipelineStateRef {
        unsafe { &*(self.0.cast::<MTLComputePipelineStateRef>()) }
    }
}

unsafe impl Send for MTLComputePipelineState {}
unsafe impl Sync for MTLComputePipelineState {}

unsafe impl objc::Message for MTLComputePipelineStateRef {}

impl MTLComputePipelineState {
    /// Gets the maximum total threadgroup size for this compute pipeline state.
    #[must_use]
    pub fn max_total_threads_per_threadgroup(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), maxTotalThreadsPerThreadgroup]
        }
    }
    
    /// Gets the thread execution width for this compute pipeline state.
    #[must_use]
    pub fn thread_execution_width(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), threadExecutionWidth]
        }
    }
}

impl fmt::Debug for MTLComputePipelineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLComputePipelineState {{ max_threads: {} }}", self.max_total_threads_per_threadgroup())
    }
}

impl Drop for MTLComputePipelineState {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLComputePipelineState {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLComputePipelineState::from_ptr(obj)
        }
    }
}

impl fmt::Debug for MTLComputeCommandEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd_encoder: &MTLCommandEncoderRef = self.as_ref();
        let label = unsafe {
            use crate::foundation::NSString;
            
            let label: *mut Object = msg_send![cmd_encoder.as_ptr(), label];
            if label.is_null() {
                "Unlabeled".to_string()
            } else {
                let ns_string = NSString::from_ptr(label);
                ns_string.to_rust_string()
            }
        };
        
        write!(f, "MTLComputeCommandEncoder {{ label: {} }}", label)
    }
}

impl Drop for MTLComputeCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLComputeCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLComputeCommandEncoder::from_ptr(obj)
        }
    }
}