//! MTLComputePipeline - A Rust wrapper around Metal's compute pipeline types.
//!
//! This module provides safe Rust bindings to the compute pipeline classes from Apple's Metal framework,
//! including compute pipeline state and descriptors.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLPipelineOption};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a compute pipeline descriptor
//! let pipeline_descriptor = metal_rs::metal::MTLComputePipelineDescriptor::new();
//! pipeline_descriptor.set_label("Basic Compute Pipeline");
//! 
//! // Configure pipeline for a compute shader
//! let library = device.new_library_with_source(SHADER_SRC, &Default::default()).unwrap();
//! let compute_function = library.get_function("compute_main").unwrap();
//! 
//! pipeline_descriptor.set_compute_function(&compute_function);
//! 
//! // Create the pipeline state
//! let pipeline_state = device.new_compute_pipeline_state_with_descriptor(&pipeline_descriptor, 
//!                                                                MTLPipelineOption::None).unwrap();
//! ```

use std::fmt;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSUInteger, NSString};
use crate::metal::library::{MTLFunction, MTLFunctionRef};
use crate::metal::linked_functions::{MTLLinkedFunctions, MTLLinkedFunctionsRef};
use crate::metal::pipeline::MTLPipelineBufferDescriptorArray;

/// A reference to an Objective-C `MTLComputePipelineDescriptor`.
pub struct MTLComputePipelineDescriptorRef(Object);

/// An owned Objective-C `MTLComputePipelineDescriptor`.
pub struct MTLComputePipelineDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLComputePipelineDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLComputePipelineDescriptorRef {}
unsafe impl Sync for MTLComputePipelineDescriptorRef {}

unsafe impl ForeignType for MTLComputePipelineDescriptor {
    type CType = Object;
    type Ref = MTLComputePipelineDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLComputePipelineDescriptor {
        MTLComputePipelineDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLComputePipelineDescriptorRef> for MTLComputePipelineDescriptor {
    fn as_ref(&self) -> &MTLComputePipelineDescriptorRef {
        unsafe { &*(self.0.cast::<MTLComputePipelineDescriptorRef>()) }
    }
}

unsafe impl Send for MTLComputePipelineDescriptor {}
unsafe impl Sync for MTLComputePipelineDescriptor {}

unsafe impl objc::Message for MTLComputePipelineDescriptorRef {}

impl MTLComputePipelineDescriptor {
    /// Creates a new compute pipeline descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLComputePipelineDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            Self::from_ptr(obj)
        }
    }
    
    /// Gets the label of the compute pipeline descriptor.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            NSString::from_ptr(label).to_string()
        }
    }
    
    /// Sets the label of the compute pipeline descriptor.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Gets the compute function of the compute pipeline descriptor.
    #[must_use]
    pub fn compute_function(&self) -> Option<MTLFunction> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), computeFunction];
            if ptr.is_null() {
                None
            } else {
                Some(MTLFunction::from_ptr(ptr))
            }
        }
    }
    
    /// Sets the compute function of the compute pipeline descriptor.
    pub fn set_compute_function(&self, function: &impl AsRef<MTLFunctionRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setComputeFunction:function.as_ref().as_ptr()];
        }
    }
    
    /// Gets whether thread group size is explicitly specified.
    #[must_use]
    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), threadGroupSizeIsMultipleOfThreadExecutionWidth]
        }
    }
    
    /// Sets whether thread group size should be a multiple of thread execution width.
    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&self, value: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setThreadGroupSizeIsMultipleOfThreadExecutionWidth:value];
        }
    }
    
    /// Gets the maximum total threads per threadgroup.
    #[must_use]
    pub fn max_total_threads_per_threadgroup(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxTotalThreadsPerThreadgroup]
        }
    }
    
    /// Sets the maximum total threads per threadgroup.
    pub fn set_max_total_threads_per_threadgroup(&self, value: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaxTotalThreadsPerThreadgroup:value];
        }
    }
    
    /// Gets the buffer descriptor array.
    #[must_use]
    pub fn buffers(&self) -> MTLPipelineBufferDescriptorArray {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), buffers];
            MTLPipelineBufferDescriptorArray::from_ptr(ptr)
        }
    }
    
    /// Gets whether the pipeline supports indirect command buffers.
    #[must_use]
    pub fn supports_indirect_command_buffers(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), supportIndirectCommandBuffers]
        }
    }
    
    /// Sets whether the pipeline supports indirect command buffers.
    pub fn set_supports_indirect_command_buffers(&self, value: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setSupportIndirectCommandBuffers:value];
        }
    }
    
    /// Gets the linked functions for the compute pipeline.
    #[must_use]
    pub fn linked_functions(&self) -> Option<MTLLinkedFunctions> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), linkedFunctions];
            if ptr.is_null() {
                None
            } else {
                Some(MTLLinkedFunctions::from_ptr(ptr))
            }
        }
    }
    
    /// Sets the linked functions for the compute pipeline.
    pub fn set_linked_functions(&self, linked_functions: &impl AsRef<MTLLinkedFunctionsRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setLinkedFunctions:linked_functions.as_ref().as_ptr()];
        }
    }
}

impl fmt::Debug for MTLComputePipelineDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLComputePipelineDescriptor")
            .field("label", &self.label())
            .field("thread_group_size_is_multiple_of_thread_execution_width", &self.thread_group_size_is_multiple_of_thread_execution_width())
            .field("max_total_threads_per_threadgroup", &self.max_total_threads_per_threadgroup())
            .field("supports_indirect_command_buffers", &self.supports_indirect_command_buffers())
            .field("linked_functions", &self.linked_functions())
            .finish()
    }
}

impl Drop for MTLComputePipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLComputePipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLComputePipelineDescriptor::from_ptr(obj)
        }
    }
}

impl Default for MTLComputePipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}