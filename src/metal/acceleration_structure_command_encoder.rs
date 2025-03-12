//! MTLAccelerationStructureCommandEncoder - A Rust wrapper around Metal's MTLAccelerationStructureCommandEncoder protocol.
//!
//! This module provides safe Rust bindings to the MTLAccelerationStructureCommandEncoder protocol from Apple's Metal framework.
//! MTLAccelerationStructureCommandEncoder is used to encode acceleration structure commands into a command buffer.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{
//!     MTLCreateSystemDefaultDevice, 
//!     MTLAccelerationStructureDescriptor,
//!     MTLAccelerationStructureUsage
//! };
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
//! // Create an acceleration structure command encoder
//! let encoder = command_buffer.new_acceleration_structure_command_encoder();
//! 
//! // Create an acceleration structure descriptor
//! let descriptor = MTLAccelerationStructureDescriptor::new();
//! descriptor.set_usage(MTLAccelerationStructureUsage::NONE);
//! 
//! // Calculate required sizes
//! let sizes = device.acceleration_structure_sizes(descriptor);
//! 
//! // Create acceleration structure
//! let accel_struct = device.new_acceleration_structure(sizes.acceleration_structure_size);
//! 
//! // Create scratch buffer for building
//! let scratch_buffer = device.new_buffer(sizes.build_scratch_buffer_size, metal_rs::metal::MTLResourceOptions::StorageModePrivate);
//! 
//! // Build the acceleration structure
//! encoder.build_acceleration_structure(&accel_struct, &descriptor, &scratch_buffer, 0);
//! 
//! // End encoding and commit
//! encoder.end_encoding();
//! command_buffer.commit();
//! ```

use std::fmt;
use std::os::raw::c_void;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};

use crate::foundation::NSUInteger;
use crate::metal::{
    MTLCommandEncoderRef,
    MTLBufferRef,
    MTLResourceUsage,
    MTLResourceRef,
    MTLResource,
    MTLDataType
};
use crate::metal::acceleration_structure::{
    MTLAccelerationStructureRef, 
    MTLAccelerationStructureDescriptorRef,
};
use crate::metal::fence::MTLFenceRef;
use crate::metal::heap::MTLHeapRef;
use crate::metal::counters::MTLCounterSampleBufferRef;

/// Options for refitting acceleration structures.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MTLAccelerationStructureRefitOptions(pub u64);

impl MTLAccelerationStructureRefitOptions {
    /// Option to refit vertex data.
    pub const VERTEX_DATA: Self = Self(1);
    /// Option to refit per-primitive data.
    pub const PER_PRIMITIVE_DATA: Self = Self(2);
}

impl std::ops::BitOr for MTLAccelerationStructureRefitOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for MTLAccelerationStructureRefitOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// A reference to an Objective-C `MTLAccelerationStructureCommandEncoder`.
pub struct MTLAccelerationStructureCommandEncoderRef(Object);

/// An owned Objective-C `MTLAccelerationStructureCommandEncoder`.
pub struct MTLAccelerationStructureCommandEncoder(*mut Object);

unsafe impl ForeignTypeRef for MTLAccelerationStructureCommandEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLAccelerationStructureCommandEncoderRef {}
unsafe impl Sync for MTLAccelerationStructureCommandEncoderRef {}

unsafe impl ForeignType for MTLAccelerationStructureCommandEncoder {
    type CType = Object;
    type Ref = MTLAccelerationStructureCommandEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLAccelerationStructureCommandEncoder {
        MTLAccelerationStructureCommandEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLAccelerationStructureCommandEncoderRef> for MTLAccelerationStructureCommandEncoder {
    fn as_ref(&self) -> &MTLAccelerationStructureCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLAccelerationStructureCommandEncoderRef>()) }
    }
}

impl AsRef<MTLCommandEncoderRef> for MTLAccelerationStructureCommandEncoder {
    fn as_ref(&self) -> &MTLCommandEncoderRef {
        unsafe { &*(self.0.cast::<MTLCommandEncoderRef>()) }
    }
}

unsafe impl Send for MTLAccelerationStructureCommandEncoder {}
unsafe impl Sync for MTLAccelerationStructureCommandEncoder {}

unsafe impl objc::Message for MTLAccelerationStructureCommandEncoderRef {}

impl crate::metal::CommandEncoder for MTLAccelerationStructureCommandEncoder {}

impl MTLAccelerationStructureCommandEncoder {
    /// Builds an acceleration structure using the specified descriptor.
    ///
    /// # Arguments
    ///
    /// * `acceleration_structure` - The destination acceleration structure.
    /// * `descriptor` - The descriptor that defines the acceleration structure.
    /// * `scratch_buffer` - A buffer to use for temporary storage during the build.
    /// * `scratch_buffer_offset` - The offset within the scratch buffer to use.
    pub fn build_acceleration_structure(
        &self,
        acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        descriptor: &impl AsRef<MTLAccelerationStructureDescriptorRef>,
        scratch_buffer: &impl AsRef<MTLBufferRef>,
        scratch_buffer_offset: NSUInteger,
    ) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![
                encoder_ref,
                buildAccelerationStructure:acceleration_structure.as_ref().as_ptr()
                descriptor:descriptor.as_ref().as_ptr()
                scratchBuffer:scratch_buffer.as_ref().as_ptr()
                scratchBufferOffset:scratch_buffer_offset
            ];
        }
    }

    /// Refits an acceleration structure using the specified descriptor.
    ///
    /// # Arguments
    ///
    /// * `source_acceleration_structure` - The source acceleration structure.
    /// * `descriptor` - The descriptor that defines the acceleration structure.
    /// * `destination_acceleration_structure` - The destination acceleration structure.
    /// * `scratch_buffer` - A buffer to use for temporary storage during the refit.
    /// * `scratch_buffer_offset` - The offset within the scratch buffer to use.
    pub fn refit_acceleration_structure(
        &self,
        source_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        descriptor: &impl AsRef<MTLAccelerationStructureDescriptorRef>,
        destination_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        scratch_buffer: &impl AsRef<MTLBufferRef>,
        scratch_buffer_offset: NSUInteger,
    ) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![
                encoder_ref,
                refitAccelerationStructure:source_acceleration_structure.as_ref().as_ptr()
                descriptor:descriptor.as_ref().as_ptr()
                destination:destination_acceleration_structure.as_ref().as_ptr()
                scratchBuffer:scratch_buffer.as_ref().as_ptr()
                scratchBufferOffset:scratch_buffer_offset
            ];
        }
    }

    /// Refits an acceleration structure using the specified descriptor and options.
    ///
    /// # Arguments
    ///
    /// * `source_acceleration_structure` - The source acceleration structure.
    /// * `descriptor` - The descriptor that defines the acceleration structure.
    /// * `destination_acceleration_structure` - The destination acceleration structure.
    /// * `scratch_buffer` - A buffer to use for temporary storage during the refit.
    /// * `scratch_buffer_offset` - The offset within the scratch buffer to use.
    /// * `options` - Options that control the refit operation.
    pub fn refit_acceleration_structure_with_options(
        &self,
        source_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        descriptor: &impl AsRef<MTLAccelerationStructureDescriptorRef>,
        destination_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        scratch_buffer: &impl AsRef<MTLBufferRef>,
        scratch_buffer_offset: NSUInteger,
        options: MTLAccelerationStructureRefitOptions,
    ) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![
                encoder_ref,
                refitAccelerationStructure:source_acceleration_structure.as_ref().as_ptr()
                descriptor:descriptor.as_ref().as_ptr()
                destination:destination_acceleration_structure.as_ref().as_ptr()
                scratchBuffer:scratch_buffer.as_ref().as_ptr()
                scratchBufferOffset:scratch_buffer_offset
                options:options.0
            ];
        }
    }

    /// Copies an acceleration structure.
    ///
    /// # Arguments
    ///
    /// * `source_acceleration_structure` - The source acceleration structure.
    /// * `destination_acceleration_structure` - The destination acceleration structure.
    pub fn copy_acceleration_structure(
        &self,
        source_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        destination_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
    ) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![
                encoder_ref,
                copyAccelerationStructure:source_acceleration_structure.as_ref().as_ptr()
                toAccelerationStructure:destination_acceleration_structure.as_ref().as_ptr()
            ];
        }
    }

    /// Writes the compacted size of an acceleration structure to a buffer.
    ///
    /// # Arguments
    ///
    /// * `acceleration_structure` - The acceleration structure to query.
    /// * `buffer` - The buffer to write the size to.
    /// * `offset` - The offset within the buffer to write to.
    pub fn write_compacted_acceleration_structure_size(
        &self,
        acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        buffer: &impl AsRef<MTLBufferRef>,
        offset: NSUInteger,
    ) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![
                encoder_ref,
                writeCompactedAccelerationStructureSize:acceleration_structure.as_ref().as_ptr()
                toBuffer:buffer.as_ref().as_ptr()
                offset:offset
            ];
        }
    }

    /// Writes the compacted size of an acceleration structure to a buffer with a specified data type.
    ///
    /// # Arguments
    ///
    /// * `acceleration_structure` - The acceleration structure to query.
    /// * `buffer` - The buffer to write the size to.
    /// * `offset` - The offset within the buffer to write to.
    /// * `size_data_type` - The data type to use for the size value.
    pub fn write_compacted_acceleration_structure_size_with_data_type(
        &self,
        acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        buffer: &impl AsRef<MTLBufferRef>,
        offset: NSUInteger,
        size_data_type: MTLDataType,
    ) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![
                encoder_ref,
                writeCompactedAccelerationStructureSize:acceleration_structure.as_ref().as_ptr()
                toBuffer:buffer.as_ref().as_ptr()
                offset:offset
                sizeDataType:size_data_type
            ];
        }
    }

    /// Copies and compacts an acceleration structure.
    ///
    /// # Arguments
    ///
    /// * `source_acceleration_structure` - The source acceleration structure.
    /// * `destination_acceleration_structure` - The destination acceleration structure.
    pub fn copy_and_compact_acceleration_structure(
        &self,
        source_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
        destination_acceleration_structure: &impl AsRef<MTLAccelerationStructureRef>,
    ) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![
                encoder_ref,
                copyAndCompactAccelerationStructure:source_acceleration_structure.as_ref().as_ptr()
                toAccelerationStructure:destination_acceleration_structure.as_ref().as_ptr()
            ];
        }
    }

    /// Updates a fence to establish a dependency between commands in this encoder and commands in subsequent encoders.
    ///
    /// # Arguments
    ///
    /// * `fence` - The fence to update.
    pub fn update_fence(&self, fence: &impl AsRef<MTLFenceRef>) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, updateFence:fence.as_ref().as_ptr()];
        }
    }

    /// Makes this encoder wait for a fence from a previous encoder to complete its work.
    ///
    /// # Arguments
    ///
    /// * `fence` - The fence to wait for.
    pub fn wait_for_fence(&self, fence: &impl AsRef<MTLFenceRef>) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, waitForFence:fence.as_ref().as_ptr()];
        }
    }

    /// Marks a resource as used.
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to use.
    /// * `usage` - How the resource will be used.
    pub fn use_resource(&self, resource: &MTLResource, usage: MTLResourceUsage) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, useResource:resource.as_ptr() usage:usage];
        }
    }

    /// Marks multiple resources as used.
    ///
    /// # Arguments
    ///
    /// * `resources` - The resources to use.
    /// * `usage` - How the resources will be used.
    pub fn use_resources(&self, resources: &[&MTLResource], usage: MTLResourceUsage) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let count = resources.len();
            let resources_ptr = resources.as_ptr() as *const *const MTLResourceRef;
            let _: () = msg_send![encoder_ref, useResources:resources_ptr count:count usage:usage];
        }
    }

    /// Marks a heap as used.
    ///
    /// # Arguments
    ///
    /// * `heap` - The heap to use.
    pub fn use_heap(&self, heap: &impl AsRef<MTLHeapRef>) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, useHeap:heap.as_ref().as_ptr()];
        }
    }

    /// Marks multiple heaps as used.
    ///
    /// # Arguments
    ///
    /// * `heaps` - The heaps to use.
    pub fn use_heaps(&self, heaps: &[&impl AsRef<MTLHeapRef>]) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let count = heaps.len();
            // Create an array of pointers to the heap objects
            let mut heap_ptrs: Vec<*const Object> = Vec::with_capacity(heaps.len());
            for heap in heaps {
                heap_ptrs.push(heap.as_ref().as_ptr());
            }
            let _: () = msg_send![encoder_ref, useHeaps:heap_ptrs.as_ptr() count:count];
        }
    }

    /// Sample GPU performance counters within this encoder.
    ///
    /// # Arguments
    ///
    /// * `sample_buffer` - The counter sample buffer to store the samples in.
    /// * `sample_index` - The index into the sample buffer to store the sample at.
    /// * `barrier` - Whether to insert a barrier before taking the sample.
    pub fn sample_counters_in_buffer(&self, sample_buffer: &impl AsRef<MTLCounterSampleBufferRef>, sample_index: NSUInteger, barrier: bool) {
        unsafe {
            let encoder_ref: &MTLAccelerationStructureCommandEncoderRef = self.as_ref();
            let _: () = msg_send![encoder_ref, sampleCountersInBuffer:sample_buffer.as_ref().as_ptr() atSampleIndex:sample_index withBarrier:barrier];
        }
    }
}

impl fmt::Debug for MTLAccelerationStructureCommandEncoder {
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
        
        write!(f, "MTLAccelerationStructureCommandEncoder {{ label: {} }}", label)
    }
}

impl Drop for MTLAccelerationStructureCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLAccelerationStructureCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLAccelerationStructureCommandEncoder::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLAccelerationStructurePassDescriptor`.
pub struct MTLAccelerationStructurePassDescriptorRef(Object);

/// An owned Objective-C `MTLAccelerationStructurePassDescriptor`.
pub struct MTLAccelerationStructurePassDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLAccelerationStructurePassDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLAccelerationStructurePassDescriptorRef {}
unsafe impl Sync for MTLAccelerationStructurePassDescriptorRef {}

unsafe impl ForeignType for MTLAccelerationStructurePassDescriptor {
    type CType = Object;
    type Ref = MTLAccelerationStructurePassDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLAccelerationStructurePassDescriptor {
        MTLAccelerationStructurePassDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLAccelerationStructurePassDescriptorRef> for MTLAccelerationStructurePassDescriptor {
    fn as_ref(&self) -> &MTLAccelerationStructurePassDescriptorRef {
        unsafe { &*(self.0.cast::<MTLAccelerationStructurePassDescriptorRef>()) }
    }
}

unsafe impl Send for MTLAccelerationStructurePassDescriptor {}
unsafe impl Sync for MTLAccelerationStructurePassDescriptor {}

unsafe impl objc::Message for MTLAccelerationStructurePassDescriptorRef {}

impl MTLAccelerationStructurePassDescriptor {
    /// Creates a new acceleration structure pass descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = objc::class!(MTLAccelerationStructurePassDescriptor);
            let obj: *mut Object = msg_send![class, accelerationStructurePassDescriptor];
            MTLAccelerationStructurePassDescriptor::from_ptr(obj)
        }
    }
}

impl Default for MTLAccelerationStructurePassDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for MTLAccelerationStructurePassDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLAccelerationStructurePassDescriptor")
    }
}

impl Drop for MTLAccelerationStructurePassDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLAccelerationStructurePassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLAccelerationStructurePassDescriptor::from_ptr(obj)
        }
    }
}