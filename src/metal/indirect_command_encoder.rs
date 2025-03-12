//! MTLIndirectCommandEncoder - A Rust wrapper around Metal's indirect command encoder API.
//!
//! This module provides Rust bindings to the MTLIndirectRenderCommand and MTLIndirectComputeCommand
//! classes from Apple's Metal framework. These encoders allow you to encode rendering and compute
//! commands into an indirect command buffer.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLDevice, MTLIndirectCommandBufferDescriptor, MTLIndirectCommandType};
//! use metal_rs::metal::MTLResourceOptions;
//! 
//! # let device = MTLDevice::system_default().unwrap();
//! # let descriptor = MTLIndirectCommandBufferDescriptor::new();
//! # descriptor.set_command_types(MTLIndirectCommandType::DRAW | MTLIndirectCommandType::DRAW_INDEXED);
//! # let icb = device.new_indirect_command_buffer_with_descriptor(&descriptor, 100, MTLResourceOptions::STORAGE_MODE_SHARED);
//! 
//! // Get a render command at index 0
//! let render_command = icb.indirect_render_command(0);
//! 
//! // Configure the render command
//! if let Some(pipeline_state) = device.new_render_pipeline_state_with_descriptor(&descriptor).ok() {
//!     render_command.set_render_pipeline_state(&pipeline_state);
//!     
//!     // Draw primitives
//!     render_command.draw_primitives(
//!         metal_rs::metal::MTLPrimitiveType::Triangle,
//!         0,  // vertex start
//!         3,  // vertex count
//!         1,  // instance count
//!         0   // base instance
//!     );
//! }
//! ```
//!
//! # See Also
//!
//! * [Apple Metal Documentation](https://developer.apple.com/documentation/metal/mtlindirectrendercommand)

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};

use crate::foundation::NSUInteger;
use crate::metal::{MTLSize, MTLRegion, MTLPrimitiveType, MTLIndexType};

/// A reference to an Objective-C `MTLIndirectRenderCommand`.
pub struct MTLIndirectRenderCommandRef(Object);

/// An owned Objective-C `MTLIndirectRenderCommand`.
pub struct MTLIndirectRenderCommand(*mut Object);

unsafe impl ForeignTypeRef for MTLIndirectRenderCommandRef {
    type CType = Object;
}

unsafe impl Send for MTLIndirectRenderCommandRef {}
unsafe impl Sync for MTLIndirectRenderCommandRef {}

unsafe impl ForeignType for MTLIndirectRenderCommand {
    type CType = Object;
    type Ref = MTLIndirectRenderCommandRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIndirectRenderCommand {
        MTLIndirectRenderCommand(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIndirectRenderCommandRef> for MTLIndirectRenderCommand {
    fn as_ref(&self) -> &MTLIndirectRenderCommandRef {
        unsafe { &*(self.0.cast::<MTLIndirectRenderCommandRef>()) }
    }
}

unsafe impl Send for MTLIndirectRenderCommand {}
unsafe impl Sync for MTLIndirectRenderCommand {}

unsafe impl objc::Message for MTLIndirectRenderCommandRef {}

impl MTLIndirectRenderCommand {
    /// Sets the render pipeline state for the command.
    pub fn set_render_pipeline_state(&self, pipeline_state: &crate::metal::render_pipeline::MTLRenderPipelineState) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setRenderPipelineState:pipeline_state.as_ptr()];
        }
    }

    /// Sets a vertex buffer for the command.
    pub fn set_vertex_buffer(&self, buffer: &crate::metal::buffer::MTLBuffer, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVertexBuffer:buffer.as_ptr() offset:offset atIndex:index];
        }
    }

    /// Sets a fragment buffer for the command.
    pub fn set_fragment_buffer(&self, buffer: &crate::metal::buffer::MTLBuffer, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFragmentBuffer:buffer.as_ptr() offset:offset atIndex:index];
        }
    }

    /// Sets a vertex buffer with a stride for the command.
    pub fn set_vertex_buffer_with_stride(&self, buffer: &crate::metal::buffer::MTLBuffer, offset: NSUInteger, stride: NSUInteger, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVertexBuffer:buffer.as_ptr() offset:offset attributeStride:stride atIndex:index];
        }
    }

    /// Commands the GPU to draw primitives.
    pub fn draw_primitives(&self, primitive_type: MTLPrimitiveType, vertex_start: NSUInteger, vertex_count: NSUInteger, instance_count: NSUInteger, base_instance: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), drawPrimitives:primitive_type 
                                                   vertexStart:vertex_start 
                                                   vertexCount:vertex_count 
                                                 instanceCount:instance_count 
                                                  baseInstance:base_instance];
        }
    }

    /// Commands the GPU to draw indexed primitives.
    pub fn draw_indexed_primitives(&self, primitive_type: MTLPrimitiveType, index_count: NSUInteger, index_type: MTLIndexType, index_buffer: &crate::metal::buffer::MTLBuffer, index_buffer_offset: NSUInteger, instance_count: NSUInteger, base_vertex: isize, base_instance: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), drawIndexedPrimitives:primitive_type 
                                                           indexCount:index_count 
                                                            indexType:index_type 
                                                          indexBuffer:index_buffer.as_ptr() 
                                                    indexBufferOffset:index_buffer_offset 
                                                        instanceCount:instance_count 
                                                           baseVertex:base_vertex 
                                                         baseInstance:base_instance];
        }
    }

    /// Sets a memory barrier to ensure resource access ordering.
    pub fn set_barrier(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setBarrier];
        }
    }

    /// Clears a previously set memory barrier.
    pub fn clear_barrier(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), clearBarrier];
        }
    }

    /// Resets the command to its initial state.
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), reset];
        }
    }

    /// Sets the threadgroup memory length for an object shader.
    pub fn set_object_threadgroup_memory_length(&self, length: NSUInteger, index: NSUInteger) {
        unsafe {
            let selector = sel!(setObjectThreadgroupMemoryLength:atIndex:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                let _: () = msg_send![self.as_ref(), setObjectThreadgroupMemoryLength:length atIndex:index];
            }
        }
    }

    /// Sets an object buffer for the command.
    pub fn set_object_buffer(&self, buffer: &crate::metal::buffer::MTLBuffer, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let selector = sel!(setObjectBuffer:offset:atIndex:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                let _: () = msg_send![self.as_ref(), setObjectBuffer:buffer.as_ptr() offset:offset atIndex:index];
            }
        }
    }

    /// Sets a mesh buffer for the command.
    pub fn set_mesh_buffer(&self, buffer: &crate::metal::buffer::MTLBuffer, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let selector = sel!(setMeshBuffer:offset:atIndex:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                let _: () = msg_send![self.as_ref(), setMeshBuffer:buffer.as_ptr() offset:offset atIndex:index];
            }
        }
    }

    /// Commands the GPU to draw mesh threadgroups.
    pub fn draw_mesh_threadgroups(&self, threadgroups_per_grid: MTLSize, threads_per_object_threadgroup: MTLSize, threads_per_mesh_threadgroup: MTLSize) {
        unsafe {
            let selector = sel!(drawMeshThreadgroups:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                let _: () = msg_send![self.as_ref(), drawMeshThreadgroups:threadgroups_per_grid 
                                          threadsPerObjectThreadgroup:threads_per_object_threadgroup 
                                            threadsPerMeshThreadgroup:threads_per_mesh_threadgroup];
            }
        }
    }

    /// Commands the GPU to draw mesh threads.
    pub fn draw_mesh_threads(&self, threads_per_grid: MTLSize, threads_per_object_threadgroup: MTLSize, threads_per_mesh_threadgroup: MTLSize) {
        unsafe {
            let selector = sel!(drawMeshThreads:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                let _: () = msg_send![self.as_ref(), drawMeshThreads:threads_per_grid 
                                      threadsPerObjectThreadgroup:threads_per_object_threadgroup 
                                        threadsPerMeshThreadgroup:threads_per_mesh_threadgroup];
            }
        }
    }
}

impl fmt::Debug for MTLIndirectRenderCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIndirectRenderCommand")
            .finish()
    }
}

impl Drop for MTLIndirectRenderCommand {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIndirectRenderCommand {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIndirectRenderCommand::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLIndirectComputeCommand`.
pub struct MTLIndirectComputeCommandRef(Object);

/// An owned Objective-C `MTLIndirectComputeCommand`.
pub struct MTLIndirectComputeCommand(*mut Object);

unsafe impl ForeignTypeRef for MTLIndirectComputeCommandRef {
    type CType = Object;
}

unsafe impl Send for MTLIndirectComputeCommandRef {}
unsafe impl Sync for MTLIndirectComputeCommandRef {}

unsafe impl ForeignType for MTLIndirectComputeCommand {
    type CType = Object;
    type Ref = MTLIndirectComputeCommandRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIndirectComputeCommand {
        MTLIndirectComputeCommand(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIndirectComputeCommandRef> for MTLIndirectComputeCommand {
    fn as_ref(&self) -> &MTLIndirectComputeCommandRef {
        unsafe { &*(self.0.cast::<MTLIndirectComputeCommandRef>()) }
    }
}

unsafe impl Send for MTLIndirectComputeCommand {}
unsafe impl Sync for MTLIndirectComputeCommand {}

unsafe impl objc::Message for MTLIndirectComputeCommandRef {}

impl MTLIndirectComputeCommand {
    /// Sets the compute pipeline state for the command.
    pub fn set_compute_pipeline_state(&self, pipeline_state: &crate::metal::compute_command_encoder::MTLComputePipelineState) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setComputePipelineState:pipeline_state.as_ptr()];
        }
    }

    /// Sets a kernel buffer for the command.
    pub fn set_kernel_buffer(&self, buffer: &crate::metal::buffer::MTLBuffer, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setKernelBuffer:buffer.as_ptr() offset:offset atIndex:index];
        }
    }

    /// Sets a kernel buffer with a stride for the command.
    pub fn set_kernel_buffer_with_stride(&self, buffer: &crate::metal::buffer::MTLBuffer, offset: NSUInteger, stride: NSUInteger, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setKernelBuffer:buffer.as_ptr() offset:offset attributeStride:stride atIndex:index];
        }
    }

    /// Concurrently dispatches compute threadgroups.
    pub fn concurrent_dispatch_threadgroups(&self, threadgroups_per_grid: MTLSize, threads_per_threadgroup: MTLSize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), concurrentDispatchThreadgroups:threadgroups_per_grid threadsPerThreadgroup:threads_per_threadgroup];
        }
    }

    /// Concurrently dispatches compute threads.
    pub fn concurrent_dispatch_threads(&self, threads_per_grid: MTLSize, threads_per_threadgroup: MTLSize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), concurrentDispatchThreads:threads_per_grid threadsPerThreadgroup:threads_per_threadgroup];
        }
    }

    /// Sets a memory barrier to ensure resource access ordering.
    pub fn set_barrier(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setBarrier];
        }
    }

    /// Clears a previously set memory barrier.
    pub fn clear_barrier(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), clearBarrier];
        }
    }

    /// Sets the dimensions for an imageblock.
    pub fn set_imageblock_width(&self, width: NSUInteger, height: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setImageblockWidth:width height:height];
        }
    }

    /// Resets the command to its initial state.
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), reset];
        }
    }

    /// Sets the threadgroup memory length.
    pub fn set_threadgroup_memory_length(&self, length: NSUInteger, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setThreadgroupMemoryLength:length atIndex:index];
        }
    }

    /// Sets the stage-in region for the command.
    pub fn set_stage_in_region(&self, region: MTLRegion) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStageInRegion:region];
        }
    }
}

impl fmt::Debug for MTLIndirectComputeCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIndirectComputeCommand")
            .finish()
    }
}

impl Drop for MTLIndirectComputeCommand {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIndirectComputeCommand {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIndirectComputeCommand::from_ptr(obj)
        }
    }
}