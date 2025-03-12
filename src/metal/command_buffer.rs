//! MTLCommandBuffer - A Rust wrapper around Metal's MTLCommandBuffer class.
//!
//! This module provides safe Rust bindings to the MTLCommandBuffer class from Apple's Metal framework.
//! MTLCommandBuffer represents a collection of work submitted to a command queue for execution.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCommandBufferStatus};
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
//! // Do some configuration with the command buffer...
//! 
//! // Commit the command buffer
//! command_buffer.commit();
//! 
//! // Wait for it to complete
//! command_buffer.wait_until_completed();
//! 
//! // Check status
//! assert_eq!(command_buffer.status(), MTLCommandBufferStatus::Completed);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::{
    MTLRenderCommandEncoder, MTLRenderPassDescriptor, 
    MTLParallelRenderCommandEncoder,
    event::MTLEventRef,
    log::MTLLogContainer
};

/// Represents the current state of a command buffer.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLCommandBufferStatus {
    /// The command buffer is not enqueued yet.
    NotEnqueued = 0,
    
    /// The command buffer is enqueued but not committed.
    Enqueued = 1,
    
    /// The command buffer is committed but not scheduled for execution.
    Committed = 2,
    
    /// The command buffer is scheduled for execution.
    Scheduled = 3,
    
    /// The command buffer is executing.
    Executing = 4,
    
    /// The command buffer has completed execution successfully.
    Completed = 5,
    
    /// The command buffer execution has resulted in an error.
    Error = 6,
}

/// A reference to an Objective-C `MTLCommandBuffer`.
pub struct MTLCommandBufferRef(Object);

/// An owned Objective-C `MTLCommandBuffer`.
pub struct MTLCommandBuffer(*mut Object);

unsafe impl ForeignTypeRef for MTLCommandBufferRef {
    type CType = Object;
}

unsafe impl Send for MTLCommandBufferRef {}
unsafe impl Sync for MTLCommandBufferRef {}

unsafe impl ForeignType for MTLCommandBuffer {
    type CType = Object;
    type Ref = MTLCommandBufferRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCommandBuffer {
        MTLCommandBuffer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCommandBufferRef> for MTLCommandBuffer {
    fn as_ref(&self) -> &MTLCommandBufferRef {
        unsafe { &*(self.0.cast::<MTLCommandBufferRef>()) }
    }
}

unsafe impl Send for MTLCommandBuffer {}
unsafe impl Sync for MTLCommandBuffer {}

unsafe impl objc::Message for MTLCommandBufferRef {}

impl MTLCommandBuffer {
    /// Returns the label of the command buffer.
    #[must_use]
    pub fn label(&self) -> Option<String> {
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
    
    /// Sets the label of the command buffer.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Returns the current status of the command buffer.
    #[must_use]
    pub fn status(&self) -> MTLCommandBufferStatus {
        unsafe {
            msg_send![self.as_ref(), status]
        }
    }
    
    /// Returns whether this command buffer has a reference to its device.
    #[must_use]
    pub fn retains_references(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), retainsReferences]
        }
    }
    
    /// Commits this command buffer for execution as soon as possible.
    pub fn commit(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), commit];
        }
    }
    
    /// Registers a block that's called when the command buffer has completed execution.
    /// 
    /// Note: This is a placeholder. In a real implementation, you'd want to use
    /// the block crate to handle the callback properly.
    pub fn add_completed_handler(&self, _handler: impl FnOnce(MTLCommandBuffer) + Send + 'static) {
        // For a real implementation, you'd want to something like:
        // unsafe {
        //     let block = Block::new(move |buffer: *mut Object| {
        //         let buffer = MTLCommandBuffer::from_ptr(buffer);
        //         handler(buffer);
        //     });
        //     let block_ptr = &block as *const _ as *mut Block<_, _>;
        //     let _: () = msg_send![self.as_ref(), addCompletedHandler:block_ptr];
        //     std::mem::forget(block);
        // }
        
        // For now, this is just a placeholder
        unsafe {
            let _: () = msg_send![self.as_ref(), addCompletedHandler:std::ptr::null_mut::<Object>()];
        }
    }
    
    /// Marks the place in the command buffer when debugging.
    pub fn push_debug_group(&self, name: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            let _: () = msg_send![self.as_ref(), pushDebugGroup:ns_string.as_ptr()];
        }
    }
    
    /// Removes the most recently added debug group marker.
    pub fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), popDebugGroup];
        }
    }
    
    /// Enqueues this command buffer for execution as soon as possible.
    pub fn enqueue(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), enqueue];
        }
    }
    
    /// Waits until this command buffer has completed execution.
    pub fn wait_until_completed(&self) {
        unsafe {
            let command_buffer_ref = self.as_ref();
            let _: () = msg_send![command_buffer_ref, waitUntilCompleted];
        }
    }
    
    /// Indicates that this command buffer needs to wait until the specified time.
    
    /// Creates a render command encoder for rendering to the attachments specified in the render pass descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The render pass descriptor to use.
    ///
    /// # Returns
    ///
    /// A new render command encoder.
    #[must_use]
    pub fn new_render_command_encoder(&self, descriptor: &MTLRenderPassDescriptor) -> MTLRenderCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), renderCommandEncoderWithDescriptor:descriptor.as_ptr()];
            MTLRenderCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Creates a parallel render command encoder for rendering to the attachments specified in the render pass descriptor.
    ///
    /// Parallel render command encoders allow you to execute rendering commands in parallel on multiple CPU threads.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The render pass descriptor to use.
    ///
    /// # Returns
    ///
    /// A new parallel render command encoder.
    #[must_use]
    pub fn new_parallel_render_command_encoder(&self, descriptor: &MTLRenderPassDescriptor) -> MTLParallelRenderCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), parallelRenderCommandEncoderWithDescriptor:descriptor.as_ptr()];
            MTLParallelRenderCommandEncoder::from_ptr(ptr)
        }
    }
    pub fn wait_until_scheduled(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), waitUntilScheduled];
        }
    }
    
    /// Optimizes the pipeline state of render commands.
    pub fn present_drawable(&self, drawable: &crate::quartzcore::CAMetalDrawable) {
        unsafe {
            let _: () = msg_send![self.as_ref(), presentDrawable:drawable.as_ptr()];
        }
    }
    
    /// Presents a drawable after a specified time.
    pub fn present_drawable_at_time(&self, drawable: &crate::quartzcore::CAMetalDrawable, presentation_time: f64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), presentDrawable:drawable.as_ptr() atTime:presentation_time];
        }
    }
    
    /// Creates a new blit command encoder.
    #[must_use]
    pub fn new_blit_command_encoder(&self) -> crate::metal::blit_command_encoder::MTLBlitCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), blitCommandEncoder];
            crate::metal::blit_command_encoder::MTLBlitCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Creates a new blit command encoder with a blit pass descriptor.
    #[must_use]
    pub fn new_blit_command_encoder_with_descriptor(&self, descriptor: &crate::metal::blit_pass::MTLBlitPassDescriptor) -> crate::metal::blit_command_encoder::MTLBlitCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), blitCommandEncoderWithDescriptor:descriptor.as_ptr()];
            crate::metal::blit_command_encoder::MTLBlitCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Creates a new compute command encoder.
    #[must_use]
    pub fn new_compute_command_encoder(&self) -> crate::metal::compute_command_encoder::MTLComputeCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), computeCommandEncoder];
            crate::metal::compute_command_encoder::MTLComputeCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Creates a new resource state command encoder.
    #[must_use]
    pub fn new_resource_state_command_encoder(&self) -> crate::metal::resource_state_command_encoder::MTLResourceStateCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), resourceStateCommandEncoder];
            crate::metal::resource_state_command_encoder::MTLResourceStateCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Creates a new compute command encoder with a specific dispatch type.
    #[must_use]
    pub fn new_compute_command_encoder_with_dispatch_type(&self, dispatch_type: crate::metal::compute_command_encoder::MTLDispatchType) -> crate::metal::compute_command_encoder::MTLComputeCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), computeCommandEncoderWithDispatchType:dispatch_type];
            crate::metal::compute_command_encoder::MTLComputeCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Creates a new compute command encoder with a compute pass descriptor.
    #[must_use]
    pub fn new_compute_command_encoder_with_descriptor(&self, descriptor: &crate::metal::compute_pass::MTLComputePassDescriptor) -> crate::metal::compute_command_encoder::MTLComputeCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), computeCommandEncoderWithDescriptor:descriptor.as_ptr()];
            crate::metal::compute_command_encoder::MTLComputeCommandEncoder::from_ptr(ptr)
        }
    }

    /// Creates a new acceleration structure command encoder.
    #[must_use]
    pub fn new_acceleration_structure_command_encoder(&self) -> crate::metal::acceleration_structure_command_encoder::MTLAccelerationStructureCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), accelerationStructureCommandEncoder];
            crate::metal::acceleration_structure_command_encoder::MTLAccelerationStructureCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Creates a new acceleration structure command encoder with a descriptor.
    #[must_use]
    pub fn new_acceleration_structure_command_encoder_with_descriptor(&self, descriptor: &crate::metal::acceleration_structure_command_encoder::MTLAccelerationStructurePassDescriptor) -> crate::metal::acceleration_structure_command_encoder::MTLAccelerationStructureCommandEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), accelerationStructureCommandEncoderWithDescriptor:descriptor.as_ptr()];
            crate::metal::acceleration_structure_command_encoder::MTLAccelerationStructureCommandEncoder::from_ptr(ptr)
        }
    }
    
    /// Encodes a command to signal an event when this command buffer completes execution.
    pub fn encode_signal_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), encodeSignalEvent:event.as_ref().as_ptr() value:value];
        }
    }
    
    /// Encodes a command to wait for an event to reach a value before executing any further commands.
    pub fn encode_wait_for_event(&self, event: &impl AsRef<MTLEventRef>, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), encodeWaitForEvent:event.as_ref().as_ptr() value:value];
        }
    }
    
    /// Returns the logs generated during the execution of this command buffer.
    ///
    /// Logs are only available when the command buffer has completed execution.
    /// You need to enable logging when creating the library with MTLCompileOptions.
    ///
    /// # Returns
    ///
    /// An optional MTLLogContainer containing logs from the command buffer execution,
    /// or None if logging is disabled or no logs were generated.
    #[must_use]
    pub fn logs(&self) -> Option<MTLLogContainer> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), logs];
            if ptr.is_null() {
                None
            } else {
                Some(MTLLogContainer::from_ptr(ptr))
            }
        }
    }
}

impl fmt::Debug for MTLCommandBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        let status = self.status();
        write!(f, "MTLCommandBuffer {{ label: {}, status: {:?} }}", label, status)
    }
}

impl Drop for MTLCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCommandBuffer::from_ptr(obj)
        }
    }
}