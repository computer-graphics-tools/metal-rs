//! MTLIndirectCommandBuffer - A Rust wrapper around Metal's indirect command buffer API.
//!
//! This module provides Rust bindings to the MTLIndirectCommandBuffer class from Apple's Metal framework.
//! MTLIndirectCommandBuffer is a buffer storing encoded commands that can be executed from the GPU.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLDevice, MTLIndirectCommandBufferDescriptor, MTLIndirectCommandType};
//! use metal_rs::metal::MTLResourceOptions;
//! 
//! let device = MTLDevice::system_default().unwrap();
//! 
//! // Create a descriptor for indirect render commands
//! let descriptor = MTLIndirectCommandBufferDescriptor::new();
//! descriptor.set_command_types(MTLIndirectCommandType::DRAW | MTLIndirectCommandType::DRAW_INDEXED);
//! descriptor.set_inherit_pipeline_state(true);
//! descriptor.set_max_vertex_buffer_bind_count(8);
//! descriptor.set_max_fragment_buffer_bind_count(4);
//! 
//! // Create the indirect command buffer
//! let max_command_count = 1000;
//! let icb = device.new_indirect_command_buffer_with_descriptor(
//!     &descriptor, 
//!     max_command_count, 
//!     MTLResourceOptions::STORAGE_MODE_SHARED
//! );
//! 
//! // Get a render command at index 0
//! let render_command = icb.indirect_render_command(0);
//! ```
//!
//! # See Also
//!
//! * [Apple Metal Documentation](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer)

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use bitflags::bitflags;

use crate::foundation::{NSRange, NSUInteger};
use crate::metal::MTLResourceRef;

bitflags! {
    /// Types of indirect commands that can be encoded in an indirect command buffer.
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MTLIndirectCommandType: NSUInteger {
        /// Draw primitives command.
        const DRAW = 1;
        /// Draw indexed primitives command.
        const DRAW_INDEXED = 2;
        /// Draw patches command.
        const DRAW_PATCHES = 4;
        /// Draw indexed patches command.
        const DRAW_INDEXED_PATCHES = 8;
        /// Concurrent dispatch command.
        const CONCURRENT_DISPATCH = 32;
        /// Concurrent dispatch threads command.
        const CONCURRENT_DISPATCH_THREADS = 64;
        /// Draw mesh threadgroups command.
        const DRAW_MESH_THREADGROUPS = 128;
        /// Draw mesh threads command.
        const DRAW_MESH_THREADS = 256;
    }
}

/// A range of commands in an indirect command buffer.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MTLIndirectCommandBufferExecutionRange {
    /// The location of the first command to execute.
    pub location: u32,
    /// The number of commands to execute.
    pub length: u32,
}

/// A reference to an Objective-C `MTLIndirectCommandBufferDescriptor`.
pub struct MTLIndirectCommandBufferDescriptorRef(Object);

/// An owned Objective-C `MTLIndirectCommandBufferDescriptor`.
pub struct MTLIndirectCommandBufferDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLIndirectCommandBufferDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLIndirectCommandBufferDescriptorRef {}
unsafe impl Sync for MTLIndirectCommandBufferDescriptorRef {}

unsafe impl ForeignType for MTLIndirectCommandBufferDescriptor {
    type CType = Object;
    type Ref = MTLIndirectCommandBufferDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIndirectCommandBufferDescriptor {
        MTLIndirectCommandBufferDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIndirectCommandBufferDescriptorRef> for MTLIndirectCommandBufferDescriptor {
    fn as_ref(&self) -> &MTLIndirectCommandBufferDescriptorRef {
        unsafe { &*(self.0.cast::<MTLIndirectCommandBufferDescriptorRef>()) }
    }
}

unsafe impl Send for MTLIndirectCommandBufferDescriptor {}
unsafe impl Sync for MTLIndirectCommandBufferDescriptor {}

unsafe impl objc::Message for MTLIndirectCommandBufferDescriptorRef {}

impl MTLIndirectCommandBufferDescriptor {
    /// Creates a new indirect command buffer descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLIndirectCommandBufferDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLIndirectCommandBufferDescriptor::from_ptr(obj)
        }
    }

    /// Gets the types of commands that can be encoded in the indirect command buffer.
    #[must_use]
    pub fn command_types(&self) -> MTLIndirectCommandType {
        unsafe {
            let raw_value: NSUInteger = msg_send![self.as_ref(), commandTypes];
            MTLIndirectCommandType::from_bits_truncate(raw_value)
        }
    }

    /// Sets the types of commands that can be encoded in the indirect command buffer.
    pub fn set_command_types(&self, command_types: MTLIndirectCommandType) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setCommandTypes:command_types.bits()];
        }
    }

    /// Gets whether the indirect command buffer inherits the pipeline state.
    #[must_use]
    pub fn inherit_pipeline_state(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), inheritPipelineState]
        }
    }

    /// Sets whether the indirect command buffer inherits the pipeline state.
    pub fn set_inherit_pipeline_state(&self, inherit_pipeline_state: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setInheritPipelineState:inherit_pipeline_state];
        }
    }

    /// Gets whether the indirect command buffer inherits buffers.
    #[must_use]
    pub fn inherit_buffers(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), inheritBuffers]
        }
    }

    /// Sets whether the indirect command buffer inherits buffers.
    pub fn set_inherit_buffers(&self, inherit_buffers: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setInheritBuffers:inherit_buffers];
        }
    }

    /// Gets the maximum number of vertex buffers that can be bound.
    #[must_use]
    pub fn max_vertex_buffer_bind_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxVertexBufferBindCount]
        }
    }

    /// Sets the maximum number of vertex buffers that can be bound.
    pub fn set_max_vertex_buffer_bind_count(&self, max_vertex_buffer_bind_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxVertexBufferBindCount:max_vertex_buffer_bind_count];
        }
    }

    /// Gets the maximum number of fragment buffers that can be bound.
    #[must_use]
    pub fn max_fragment_buffer_bind_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxFragmentBufferBindCount]
        }
    }

    /// Sets the maximum number of fragment buffers that can be bound.
    pub fn set_max_fragment_buffer_bind_count(&self, max_fragment_buffer_bind_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxFragmentBufferBindCount:max_fragment_buffer_bind_count];
        }
    }

    /// Gets the maximum number of kernel buffers that can be bound.
    #[must_use]
    pub fn max_kernel_buffer_bind_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxKernelBufferBindCount]
        }
    }

    /// Sets the maximum number of kernel buffers that can be bound.
    pub fn set_max_kernel_buffer_bind_count(&self, max_kernel_buffer_bind_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxKernelBufferBindCount:max_kernel_buffer_bind_count];
        }
    }

    /// Gets the maximum number of kernel threadgroup memory bindings.
    #[must_use]
    pub fn max_kernel_threadgroup_memory_bind_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxKernelThreadgroupMemoryBindCount]
        }
    }

    /// Sets the maximum number of kernel threadgroup memory bindings.
    pub fn set_max_kernel_threadgroup_memory_bind_count(&self, max_kernel_threadgroup_memory_bind_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxKernelThreadgroupMemoryBindCount:max_kernel_threadgroup_memory_bind_count];
        }
    }

    /// Gets the maximum number of object buffers that can be bound.
    #[must_use]
    pub fn max_object_buffer_bind_count(&self) -> NSUInteger {
        unsafe {
            let result: NSUInteger = msg_send![self.as_ref(), maxObjectBufferBindCount];
            result
        }
    }

    /// Sets the maximum number of object buffers that can be bound.
    pub fn set_max_object_buffer_bind_count(&self, max_object_buffer_bind_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxObjectBufferBindCount:max_object_buffer_bind_count];
        }
    }

    /// Gets the maximum number of mesh buffers that can be bound.
    #[must_use]
    pub fn max_mesh_buffer_bind_count(&self) -> NSUInteger {
        unsafe {
            let result: NSUInteger = msg_send![self.as_ref(), maxMeshBufferBindCount];
            result
        }
    }

    /// Sets the maximum number of mesh buffers that can be bound.
    pub fn set_max_mesh_buffer_bind_count(&self, max_mesh_buffer_bind_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxMeshBufferBindCount:max_mesh_buffer_bind_count];
        }
    }

    /// Gets the maximum number of object threadgroup memory bindings.
    #[must_use]
    pub fn max_object_threadgroup_memory_bind_count(&self) -> NSUInteger {
        unsafe {
            let result: NSUInteger = msg_send![self.as_ref(), maxObjectThreadgroupMemoryBindCount];
            result
        }
    }

    /// Sets the maximum number of object threadgroup memory bindings.
    pub fn set_max_object_threadgroup_memory_bind_count(&self, max_object_threadgroup_memory_bind_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxObjectThreadgroupMemoryBindCount:max_object_threadgroup_memory_bind_count];
        }
    }

    /// Gets whether the indirect command buffer supports ray tracing.
    #[must_use]
    pub fn support_ray_tracing(&self) -> bool {
        unsafe {
            // This is a newer API, so it might not be available on all systems
            let selector = sel!(supportRayTracing);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                msg_send![self.as_ref(), supportRayTracing]
            } else {
                false
            }
        }
    }

    /// Sets whether the indirect command buffer supports ray tracing.
    pub fn set_support_ray_tracing(&self, support_ray_tracing: bool) {
        unsafe {
            let selector = sel!(setSupportRayTracing:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                let _: () = msg_send![self.as_ref(), setSupportRayTracing:support_ray_tracing];
            }
        }
    }

    /// Gets whether the indirect command buffer supports dynamic attribute stride.
    #[must_use]
    pub fn support_dynamic_attribute_stride(&self) -> bool {
        unsafe {
            // This is a newer API, so it might not be available on all systems
            let selector = sel!(supportDynamicAttributeStride);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                msg_send![self.as_ref(), supportDynamicAttributeStride]
            } else {
                false
            }
        }
    }

    /// Sets whether the indirect command buffer supports dynamic attribute stride.
    pub fn set_support_dynamic_attribute_stride(&self, support_dynamic_attribute_stride: bool) {
        unsafe {
            let selector = sel!(setSupportDynamicAttributeStride:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if responds {
                let _: () = msg_send![self.as_ref(), setSupportDynamicAttributeStride:support_dynamic_attribute_stride];
            }
        }
    }
}

impl fmt::Debug for MTLIndirectCommandBufferDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIndirectCommandBufferDescriptor")
            .field("command_types", &self.command_types())
            .field("inherit_pipeline_state", &self.inherit_pipeline_state())
            .field("inherit_buffers", &self.inherit_buffers())
            .field("max_vertex_buffer_bind_count", &self.max_vertex_buffer_bind_count())
            .field("max_fragment_buffer_bind_count", &self.max_fragment_buffer_bind_count())
            .field("max_kernel_buffer_bind_count", &self.max_kernel_buffer_bind_count())
            .finish()
    }
}

impl Default for MTLIndirectCommandBufferDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MTLIndirectCommandBufferDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIndirectCommandBufferDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIndirectCommandBufferDescriptor::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLIndirectCommandBuffer`.
pub struct MTLIndirectCommandBufferRef(Object);

/// An owned Objective-C `MTLIndirectCommandBuffer`.
pub struct MTLIndirectCommandBuffer(*mut Object);

unsafe impl ForeignTypeRef for MTLIndirectCommandBufferRef {
    type CType = Object;
}

unsafe impl Send for MTLIndirectCommandBufferRef {}
unsafe impl Sync for MTLIndirectCommandBufferRef {}

unsafe impl ForeignType for MTLIndirectCommandBuffer {
    type CType = Object;
    type Ref = MTLIndirectCommandBufferRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIndirectCommandBuffer {
        MTLIndirectCommandBuffer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIndirectCommandBufferRef> for MTLIndirectCommandBuffer {
    fn as_ref(&self) -> &MTLIndirectCommandBufferRef {
        unsafe { &*(self.0.cast::<MTLIndirectCommandBufferRef>()) }
    }
}

impl AsRef<MTLResourceRef> for MTLIndirectCommandBuffer {
    fn as_ref(&self) -> &MTLResourceRef {
        unsafe { &*(self.0.cast::<MTLResourceRef>()) }
    }
}

unsafe impl Send for MTLIndirectCommandBuffer {}
unsafe impl Sync for MTLIndirectCommandBuffer {}

unsafe impl objc::Message for MTLIndirectCommandBufferRef {}

impl MTLIndirectCommandBuffer {
    /// Gets the number of commands in the buffer.
    #[must_use]
    pub fn size(&self) -> NSUInteger {
        unsafe {
            let buffer_ref: &MTLIndirectCommandBufferRef = self.as_ref();
            msg_send![buffer_ref, size]
        }
    }

    /// Gets the GPU resource ID (available in newer Metal versions).
    #[must_use]
    pub fn gpu_resource_id(&self) -> u64 {
        unsafe {
            // This method might not be available on all Metal versions
            let buffer_ref: &MTLIndirectCommandBufferRef = self.as_ref();
            let selector = sel!(gpuResourceID);
            let responds: bool = msg_send![buffer_ref, respondsToSelector:selector];
            if responds {
                msg_send![buffer_ref, gpuResourceID]
            } else {
                0
            }
        }
    }

    /// Resets the specified range of commands.
    pub fn reset(&self, range: NSRange) {
        unsafe {
            let buffer_ref: &MTLIndirectCommandBufferRef = self.as_ref();
            let _: () = msg_send![buffer_ref, resetWithRange:range];
        }
    }

    /// Gets the indirect render command at the specified index.
    #[must_use]
    pub fn indirect_render_command(&self, command_index: NSUInteger) -> crate::metal::indirect_command_encoder::MTLIndirectRenderCommand {
        unsafe {
            let buffer_ref: &MTLIndirectCommandBufferRef = self.as_ref();
            let ptr: *mut Object = msg_send![buffer_ref, indirectRenderCommandAtIndex:command_index];
            crate::metal::indirect_command_encoder::MTLIndirectRenderCommand::from_ptr(ptr)
        }
    }

    /// Gets the indirect compute command at the specified index.
    #[must_use]
    pub fn indirect_compute_command(&self, command_index: NSUInteger) -> crate::metal::indirect_command_encoder::MTLIndirectComputeCommand {
        unsafe {
            let buffer_ref: &MTLIndirectCommandBufferRef = self.as_ref();
            let ptr: *mut Object = msg_send![buffer_ref, indirectComputeCommandAtIndex:command_index];
            crate::metal::indirect_command_encoder::MTLIndirectComputeCommand::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLIndirectCommandBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIndirectCommandBuffer")
            .field("size", &self.size())
            .finish()
    }
}

impl Drop for MTLIndirectCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIndirectCommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIndirectCommandBuffer::from_ptr(obj)
        }
    }
}