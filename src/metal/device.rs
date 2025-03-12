//! MTLDevice - A Rust wrapper around Metal's MTLDevice class.
//!
//! This module provides safe Rust bindings to the MTLDevice class from Apple's Metal framework.
//! MTLDevice represents a Metal GPU device that can create resources and perform rendering and compute operations.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLFeatureSet};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Get device properties
//! let name = device.name();
//! let low_power = device.is_low_power();
//! let headless = device.is_headless();
//! 
//! // Check feature set support
//! let supports_macos_gpufamily1_v1 = device.supports_feature_set(MTLFeatureSet::MacOSGPUFamily1_v1);
//! 
//! println!("Device: {}, Low Power: {}, Headless: {}", name, low_power, headless);
//! ```

#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    // This is empty, but links with the Metal framework
}

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSArray, NSString, NSURL};
use crate::metal::command_queue::MTLCommandQueue;
use crate::metal::counters::{MTLCounterSampleBuffer, MTLCounterSampleBufferDescriptorRef, MTLCounterSamplingPoint};
use crate::metal::event::{MTLEvent, MTLSharedEvent};
use crate::metal::function_stitching::MTLStitchedLibraryDescriptor;
use crate::metal::indirect_command_buffer::{MTLIndirectCommandBufferDescriptor, MTLIndirectCommandBuffer};
use crate::metal::binary_archive::MTLBinaryArchiveDescriptor;
use crate::metal::log_state::{MTLLogState, MTLLogStateDescriptor, MTLLogStateError};
use crate::metal::io_command_queue::{MTLIOCommandQueue, MTLIOCommandQueueDescriptor};

/// MTLFeatureSet - Enum representing Metal feature sets.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLFeatureSet {
    // macOS
    MacOSGPUFamily1_v1 = 10000,
    MacOSGPUFamily1_v2 = 10001,
    MacOSGPUFamily1_v3 = 10003,
    MacOSGPUFamily1_v4 = 10004,
    MacOSGPUFamily2_v1 = 10005,
    
    // iOS
    iOSGPUFamily1_v1 = 0,
    iOSGPUFamily1_v2 = 2,
    iOSGPUFamily1_v3 = 3,
    iOSGPUFamily1_v4 = 4,
    iOSGPUFamily1_v5 = 5,
    
    iOSGPUFamily2_v1 = 1,
    iOSGPUFamily2_v2 = 6,
    iOSGPUFamily2_v3 = 7,
    iOSGPUFamily2_v4 = 8,
    iOSGPUFamily2_v5 = 9,
    
    iOSGPUFamily3_v1 = 10,
    iOSGPUFamily3_v2 = 11,
    iOSGPUFamily3_v3 = 12,
    iOSGPUFamily3_v4 = 13,
    
    iOSGPUFamily4_v1 = 14,
    iOSGPUFamily4_v2 = 15,
    iOSGPUFamily4_v3 = 16,
    
    iOSGPUFamily5_v1 = 17,
    
    // tvOS
    tvOSGPUFamily1_v1 = 30000,
    tvOSGPUFamily1_v2 = 30001,
    tvOSGPUFamily1_v3 = 30002,
    tvOSGPUFamily1_v4 = 30004,
    
    tvOSGPUFamily2_v1 = 30003,
    tvOSGPUFamily2_v2 = 30005,
}

/// A reference to an Objective-C `MTLDevice`.
pub struct MTLDeviceRef(Object);

/// An owned Objective-C `MTLDevice`.
pub struct MTLDevice(*mut Object);

unsafe impl ForeignTypeRef for MTLDeviceRef {
    type CType = Object;
}

unsafe impl Send for MTLDeviceRef {}
unsafe impl Sync for MTLDeviceRef {}

unsafe impl ForeignType for MTLDevice {
    type CType = Object;
    type Ref = MTLDeviceRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLDevice {
        MTLDevice(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLDeviceRef> for MTLDevice {
    fn as_ref(&self) -> &MTLDeviceRef {
        unsafe { &*(self.0.cast::<MTLDeviceRef>()) }
    }
}

unsafe impl Send for MTLDevice {}
unsafe impl Sync for MTLDevice {}

unsafe impl objc::Message for MTLDeviceRef {}

impl MTLDevice {
    /// Returns the name of the device.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), name];
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }
    
    /// Returns whether the device is low power.
    #[must_use]
    pub fn is_low_power(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), isLowPower]
        }
    }
    
    /// Returns whether the device is headless (has no display).
    #[must_use]
    pub fn is_headless(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), isHeadless]
        }
    }
    
    /// Returns whether the device supports unified memory architecture.
    #[must_use]
    pub fn has_unified_memory(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), hasUnifiedMemory]
        }
    }
    
    /// Checks if the device supports a given feature set.
    #[must_use]
    pub fn supports_feature_set(&self, feature_set: MTLFeatureSet) -> bool {
        unsafe {
            msg_send![self.as_ref(), supportsFeatureSet:feature_set]
        }
    }
    
    /// Creates a new command queue.
    #[must_use]
    pub fn new_command_queue(&self) -> MTLCommandQueue {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newCommandQueue];
            MTLCommandQueue::from_ptr(ptr)
        }
    }
    
    /// Creates a new command queue with a specific maximum command buffer count.
    #[must_use]
    pub fn new_command_queue_with_max_command_buffer_count(&self, max_command_buffer_count: u64) -> MTLCommandQueue {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newCommandQueueWithMaxCommandBufferCount:max_command_buffer_count];
            MTLCommandQueue::from_ptr(ptr)
        }
    }
    
    /// Creates a buffer with the specified length and options.
    #[must_use]
    pub fn new_buffer(&self, length: usize, options: crate::metal::MTLResourceOptions) -> crate::metal::MTLBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newBufferWithLength:length
                                                                  options:options];
            crate::metal::MTLBuffer::from_ptr(ptr)
        }
    }
    
    /// Creates a buffer with the specified data and options.
    #[must_use]
    pub fn new_buffer_with_data(
        &self, 
        data: *const std::ffi::c_void, 
        length: usize, 
        options: crate::metal::MTLResourceOptions
    ) -> crate::metal::MTLBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newBufferWithBytes:data
                                                                  length:length
                                                                 options:options];
            crate::metal::MTLBuffer::from_ptr(ptr)
        }
    }
    
    /// Creates a new library from the source code in the Metal Shading Language.
    pub fn new_library_with_source(
        &self, 
        source: &str, 
        options: &crate::metal::library::MTLCompileOptions
    ) -> Result<crate::metal::library::MTLLibrary, crate::metal::library::LibraryError> {
        unsafe {
            let source_str = NSString::from_rust_str(source);
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newLibraryWithSource:source_str.as_ptr()
                                                                 options:options.as_ptr()
                                                                   error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(crate::metal::library::LibraryError(error_str))
            } else {
                Ok(crate::metal::library::MTLLibrary::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new texture from a descriptor.
    #[must_use]
    pub fn new_texture(&self, descriptor: &crate::metal::texture::MTLTextureDescriptor) -> crate::metal::MTLTexture {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newTextureWithDescriptor:descriptor.as_ptr()];
            crate::metal::MTLTexture::from_ptr(ptr)
        }
    }
    
    /// Creates a new depth stencil state from a descriptor.
    #[must_use]
    pub fn new_depth_stencil_state(&self, descriptor: &crate::metal::depth_stencil::MTLDepthStencilDescriptor) -> crate::metal::depth_stencil::MTLDepthStencilState {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newDepthStencilStateWithDescriptor:descriptor.as_ptr()];
            crate::metal::depth_stencil::MTLDepthStencilState::from_ptr(ptr)
        }
    }
    
    /// Creates a new compute pipeline state from a compute function.
    pub fn new_compute_pipeline_state_with_function(&self, function: &impl AsRef<crate::metal::library::MTLFunctionRef>) -> Result<crate::metal::compute_command_encoder::MTLComputePipelineState, String> {
        unsafe {
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newComputePipelineStateWithFunction:function.as_ref().as_ptr()
                                                                     error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(error_str)
            } else {
                Ok(crate::metal::compute_command_encoder::MTLComputePipelineState::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new compute pipeline state from a descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The compute pipeline descriptor.
    /// * `options` - The options to use when creating the compute pipeline state.
    ///
    /// # Returns
    ///
    /// A result containing the compute pipeline state if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the compute pipeline state could not be created.
    pub fn new_compute_pipeline_state_with_descriptor(
        &self, 
        descriptor: &crate::metal::compute_pipeline::MTLComputePipelineDescriptor,
        options: crate::metal::render_pipeline::MTLPipelineOption
    ) -> Result<crate::metal::compute_command_encoder::MTLComputePipelineState, String> {
        unsafe {
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newComputePipelineStateWithDescriptor:descriptor.as_ptr()
                                                                options:options as u64
                                                                  error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(error_str)
            } else {
                Ok(crate::metal::compute_command_encoder::MTLComputePipelineState::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new compute pipeline state from a descriptor with a stage input output descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The compute pipeline descriptor.
    /// * `stage_input_descriptor` - The stage input output descriptor.
    /// * `options` - The options to use when creating the compute pipeline state.
    ///
    /// # Returns
    ///
    /// A result containing the compute pipeline state if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the compute pipeline state could not be created.
    pub fn new_compute_pipeline_state_with_stage_input(
        &self,
        descriptor: &crate::metal::compute_pipeline::MTLComputePipelineDescriptor,
        stage_input_descriptor: &crate::metal::stage_input_output_descriptor::MTLStageInputOutputDescriptor,
        options: crate::metal::render_pipeline::MTLPipelineOption
    ) -> Result<crate::metal::compute_command_encoder::MTLComputePipelineState, String> {
        unsafe {
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newComputePipelineStateWithDescriptor:descriptor.as_ptr()
                                                                                 stageInputDescriptor:stage_input_descriptor.as_ptr()
                                                                                             options:options as u64
                                                                                               error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(error_str)
            } else {
                Ok(crate::metal::compute_command_encoder::MTLComputePipelineState::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new sampler state from a descriptor.
    #[must_use]
    pub fn new_sampler_state(&self, descriptor: &impl AsRef<crate::metal::sampler::MTLSamplerDescriptorRef>) -> crate::metal::sampler::MTLSamplerState {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newSamplerStateWithDescriptor:descriptor.as_ref().as_ptr()];
            crate::metal::sampler::MTLSamplerState::from_ptr(ptr)
        }
    }
    
    /// Creates a new render pipeline state from a descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error if the render pipeline state could not be created.
    pub fn new_render_pipeline_state(&self, descriptor: &crate::metal::render_pipeline::MTLRenderPipelineDescriptor) -> Result<crate::metal::render_pipeline::MTLRenderPipelineState, String> {
        unsafe {
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newRenderPipelineStateWithDescriptor:descriptor.as_ptr()
                                                                error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(error_str)
            } else {
                Ok(crate::metal::render_pipeline::MTLRenderPipelineState::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new render pipeline state with reflection data.
    ///
    /// # Errors
    ///
    /// Returns an error if the render pipeline state could not be created.
    pub fn new_render_pipeline_state_with_reflection(
        &self, 
        descriptor: &crate::metal::render_pipeline::MTLRenderPipelineDescriptor,
        options: u64
    ) -> Result<(crate::metal::render_pipeline::MTLRenderPipelineState, Option<crate::metal::render_pipeline::MTLRenderPipelineReflection>), String> {
        unsafe {
            let mut err: *mut Object = std::ptr::null_mut();
            let mut reflection: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newRenderPipelineStateWithDescriptor:descriptor.as_ptr()
                                                                options:options
                                                              reflection:&mut reflection
                                                                  error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(error_str)
            } else {
                let pipeline_state = crate::metal::render_pipeline::MTLRenderPipelineState::from_ptr(ptr);
                
                let reflection_option = if reflection.is_null() {
                    None
                } else {
                    Some(crate::metal::render_pipeline::MTLRenderPipelineReflection::from_ptr(reflection))
                };
                
                Ok((pipeline_state, reflection_option))
            }
        }
    }

    /// Creates a new fence for cross-encoder synchronization.
    #[must_use]
    pub fn new_fence(&self) -> crate::metal::fence::MTLFence {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newFence];
            crate::metal::fence::MTLFence::from_ptr(ptr)
        }
    }

    /// Creates a new heap from a descriptor.
    #[must_use]
    pub fn new_heap(&self, descriptor: &impl AsRef<crate::metal::heap::MTLHeapDescriptorRef>) -> crate::metal::heap::MTLHeap {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newHeapWithDescriptor:descriptor.as_ref().as_ptr()];
            crate::metal::heap::MTLHeap::from_ptr(ptr)
        }
    }
    
    /// Creates a new event for GPU timeline synchronization.
    #[must_use]
    pub fn new_event(&self) -> MTLEvent {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newEvent];
            MTLEvent::from_ptr(ptr)
        }
    }
    
    /// Creates a new shared event for cross-process GPU synchronization.
    #[must_use]
    pub fn new_shared_event(&self) -> MTLSharedEvent {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newSharedEvent];
            MTLSharedEvent::from_ptr(ptr)
        }
    }
    
    /// Creates a new shared event from a shared event handle.
    #[must_use]
    pub fn new_shared_event_from_handle(&self, shared_event_handle: &crate::metal::event::MTLSharedEventHandle) -> MTLSharedEvent {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newSharedEventWithHandle:shared_event_handle.as_ptr()];
            MTLSharedEvent::from_ptr(ptr)
        }
    }
    
    /// Creates a new residency set with the specified descriptor.
    #[must_use]
    pub fn new_residency_set(&self, descriptor: &crate::metal::residency_set::MTLResidencySetDescriptor) -> crate::metal::residency_set::MTLResidencySet {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newResidencySetWithDescriptor:descriptor.as_ptr()];
            crate::metal::residency_set::MTLResidencySet::from_ptr(ptr)
        }
    }
    
    /// Creates a new rasterization rate map with the specified descriptor.
    #[must_use]
    pub fn new_rasterization_rate_map(&self, descriptor: &crate::metal::rasterization_rate::MTLRasterizationRateMapDescriptor) -> crate::metal::rasterization_rate::MTLRasterizationRateMap {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newRasterizationRateMapWithDescriptor:descriptor.as_ptr()];
            crate::metal::rasterization_rate::MTLRasterizationRateMap::from_ptr(ptr)
        }
    }

    /// Returns an array of counter sets supported by this device.
    ///
    /// # Returns
    ///
    /// An array of `MTLCounterSet` objects representing the counter sets supported by this device,
    /// or `None` if no counter sets are supported.
    #[must_use]
    pub fn counter_sets(&self) -> Option<NSArray> {
        unsafe {
            let sets: *mut Object = msg_send![self.as_ref(), counterSets];
            if sets.is_null() {
                None
            } else {
                Some(NSArray::from_ptr(sets))
            }
        }
    }

    /// Creates a new counter sample buffer with the given descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the counter sample buffer.
    ///
    /// # Returns
    ///
    /// The newly created counter sample buffer, or `None` if an error occurred.
    #[must_use]
    pub fn new_counter_sample_buffer(&self, descriptor: &impl AsRef<MTLCounterSampleBufferDescriptorRef>) -> Option<MTLCounterSampleBuffer> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newCounterSampleBufferWithDescriptor:descriptor.as_ref().as_ptr() error:std::ptr::null_mut::<*mut std::ffi::c_void>()];
            if ptr.is_null() {
                None
            } else {
                Some(MTLCounterSampleBuffer::from_ptr(ptr))
            }
        }
    }

    /// Returns whether the device supports counter sampling at the given sampling point.
    ///
    /// # Arguments
    ///
    /// * `sampling_point` - The sampling point to check.
    ///
    /// # Returns
    ///
    /// `true` if the device supports counter sampling at the given sampling point, `false` otherwise.
    #[must_use]
    pub fn supports_counter_sampling(&self, sampling_point: MTLCounterSamplingPoint) -> bool {
        unsafe {
            msg_send![self.as_ref(), supportsCounterSampling:sampling_point]
        }
    }

    /// Calculates the allocation size and alignment for a buffer in a heap.
    #[must_use]
    pub fn heap_buffer_size_and_align(&self, length: usize, options: crate::metal::MTLResourceOptions) -> (usize, usize) {
        unsafe {
            // SizeAndAlign struct
            #[repr(C)]
            struct SizeAndAlign {
                size: usize,
                align: usize,
            }

            let result: SizeAndAlign = msg_send![self.as_ref(), heapBufferSizeAndAlignWithLength:length options:options];
            (result.size, result.align)
        }
    }

    /// Calculates the allocation size and alignment for a texture in a heap.
    #[must_use]
    pub fn heap_texture_size_and_align(&self, descriptor: &impl AsRef<crate::metal::texture::MTLTextureDescriptorRef>) -> (usize, usize) {
        unsafe {
            // SizeAndAlign struct
            #[repr(C)]
            struct SizeAndAlign {
                size: usize,
                align: usize,
            }

            let result: SizeAndAlign = msg_send![self.as_ref(), heapTextureSizeAndAlignWithDescriptor:descriptor.as_ref().as_ptr()];
            (result.size, result.align)
        }
    }

    /// Creates a new argument encoder with an array of argument descriptors.
    #[must_use]
    pub fn new_argument_encoder(&self, arguments: &impl AsRef<crate::foundation::NSArrayRef>) -> crate::metal::argument_encoder::MTLArgumentEncoder {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newArgumentEncoderWithArguments:arguments.as_ref().as_ptr()];
            crate::metal::argument_encoder::MTLArgumentEncoder::from_ptr(ptr)
        }
    }

    /// Creates a new library by dynamically stitching functions.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor specifying how to stitch functions.
    ///
    /// # Returns
    ///
    /// A result containing the library if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the stitched library could not be created.
    pub fn new_stitched_library_with_descriptor(&self, descriptor: &MTLStitchedLibraryDescriptor) -> Result<crate::metal::library::MTLLibrary, crate::metal::library::LibraryError> {
        unsafe {
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newStitchedLibraryWithDescriptor:descriptor.as_ptr()
                                                                                  error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(crate::metal::library::LibraryError(error_str))
            } else {
                Ok(crate::metal::library::MTLLibrary::from_ptr(ptr))
            }
        }
    }

    /// Returns whether the device supports stitched libraries.
    ///
    /// Stitched libraries allow dynamic composition of shader functions at runtime.
    /// This feature is available in macOS 13.0+ and iOS 16.0+.
    #[must_use]
    pub fn supports_stitched_libraries(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), supportsStitchedLibraries]
        }
    }

    /// Creates a new library from binary data (e.g., compiled Metal libraries).
    ///
    /// # Arguments
    ///
    /// * `data` - The binary data containing the compiled Metal library.
    ///
    /// # Returns
    ///
    /// A result containing the library if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the library could not be created from the binary data.
    pub fn new_library_with_data(&self, data: &[u8]) -> Result<crate::metal::library::MTLLibrary, crate::metal::library::LibraryError> {
        unsafe {
            // Create NSData directly
            let ns_data_class = objc::class!(NSData);
            let ns_data: *mut Object = msg_send![ns_data_class, dataWithBytes:data.as_ptr() 
                                                              length:data.len()];
            
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newLibraryWithData:ns_data
                                                                error:&mut err];
            
            // Release the NSData object
            let _: () = msg_send![ns_data, release];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(crate::metal::library::LibraryError(error_str))
            } else {
                Ok(crate::metal::library::MTLLibrary::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new indirect command buffer with the specified descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the indirect command buffer.
    /// * `max_command_count` - The maximum number of commands that can be encoded in the buffer.
    /// * `options` - The resource options for the indirect command buffer.
    ///
    /// # Returns
    ///
    /// A new indirect command buffer.
    #[must_use]
    pub fn new_indirect_command_buffer_with_descriptor(
        &self,
        descriptor: &MTLIndirectCommandBufferDescriptor,
        max_command_count: u64,
        options: crate::metal::MTLResourceOptions
    ) -> MTLIndirectCommandBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newIndirectCommandBufferWithDescriptor:descriptor.as_ptr()
                                                                                   maxCommandCount:max_command_count
                                                                                           options:options];
            MTLIndirectCommandBuffer::from_ptr(ptr)
        }
    }
    
    /// Returns whether the device supports indirect command buffers.
    ///
    /// # Returns
    ///
    /// `true` if the device supports indirect command buffers, `false` otherwise.
    #[must_use]
    pub fn supports_indirect_command_buffers(&self) -> bool {
        unsafe {
            // This method was introduced in Metal 2.0, so check if it exists
            let selector = sel!(supportsFeatureSet:);
            let responds: bool = msg_send![self.as_ref(), respondsToSelector:selector];
            if !responds {
                return false;
            }
            
            // Check if the device supports at least macOS 10.14 or iOS 12.0 Metal features
            let macos_gpu_family2_v1 = MTLFeatureSet::MacOSGPUFamily2_v1 as u64;
            let ios_gpu_family3_v3 = MTLFeatureSet::iOSGPUFamily3_v3 as u64;
            
            let macos_support: bool = msg_send![self.as_ref(), supportsFeatureSet:macos_gpu_family2_v1];
            let ios_support: bool = msg_send![self.as_ref(), supportsFeatureSet:ios_gpu_family3_v3];
            
            macos_support || ios_support
        }
    }
    
    /// Creates a new binary archive from a descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the binary archive.
    ///
    /// # Returns
    ///
    /// A result containing the binary archive if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the binary archive could not be created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{BinaryArchiveDescriptor, Device};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = BinaryArchiveDescriptor::new();
    /// descriptor.set_url("file:///path/to/archive.metallib");
    /// 
    /// let binary_archive = device.new_binary_archive_with_descriptor(&descriptor)
    ///     .expect("Failed to create binary archive");
    /// ```
    pub fn new_binary_archive_with_descriptor(&self, descriptor: &crate::metal::binary_archive::MTLBinaryArchiveDescriptor) -> Result<crate::metal::binary_archive::MTLBinaryArchive, String> {
        unsafe {
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newBinaryArchiveWithDescriptor:descriptor.as_ptr()
                                                                 error:&mut err];
            
            if !err.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![err, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error creating binary archive".to_string())
                }
            } else if ptr.is_null() {
                Err("Failed to create binary archive".to_string())
            } else {
                Ok(crate::metal::binary_archive::MTLBinaryArchive::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new dynamic library from a URL.
    ///
    /// # Arguments
    ///
    /// * `url_string` - The URL string for the dynamic library.
    ///
    /// # Returns
    ///
    /// A result containing the dynamic library if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the dynamic library could not be created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::Device;
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let dynamic_library = device.new_dynamic_library("file:///path/to/library.metallib")
    ///     .expect("Failed to load dynamic library");
    /// ```
    pub fn new_dynamic_library(&self, url_string: &str) -> Result<crate::metal::dynamic_library::MTLDynamicLibrary, String> {
        unsafe {
            let ns_string = NSString::from_rust_str(url_string);
            let url_class = class!(NSURL);
            let url: *mut Object = msg_send![url_class, URLWithString: ns_string.as_ptr()];
            
            if url.is_null() {
                return Err("Invalid URL".to_string());
            }
            
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newDynamicLibraryWithURL:url
                                                            error:&mut err];
            
            if !err.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![err, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error creating dynamic library".to_string())
                }
            } else if ptr.is_null() {
                Err("Failed to create dynamic library".to_string())
            } else {
                Ok(crate::metal::dynamic_library::MTLDynamicLibrary::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new dynamic library from binary data.
    ///
    /// # Arguments
    ///
    /// * `data` - The binary data for the dynamic library.
    ///
    /// # Returns
    ///
    /// A result containing the dynamic library if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the dynamic library could not be created from the data.
    pub fn new_dynamic_library_with_data(&self, data: &[u8]) -> Result<crate::metal::dynamic_library::MTLDynamicLibrary, String> {
        unsafe {
            // Create NSData directly
            let ns_data_class = objc::class!(NSData);
            let ns_data: *mut Object = msg_send![ns_data_class, dataWithBytes:data.as_ptr() 
                                                              length:data.len()];
            
            if ns_data.is_null() {
                return Err("Failed to create data object".to_string());
            }
            
            let mut err: *mut Object = std::ptr::null_mut();
            
            let ptr: *mut Object = msg_send![self.as_ref(), newDynamicLibraryWithData:ns_data
                                                            error:&mut err];
            
            // Release the NSData object
            let _: () = msg_send![ns_data, release];
            
            if !err.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![err, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error creating dynamic library from data".to_string())
                }
            } else if ptr.is_null() {
                Err("Failed to create dynamic library from data".to_string())
            } else {
                Ok(crate::metal::dynamic_library::MTLDynamicLibrary::from_ptr(ptr))
            }
        }
    }
    
    /// Checks if the device supports binary archives.
    ///
    /// # Returns
    ///
    /// `true` if the device supports binary archives, `false` otherwise.
    #[must_use]
    pub fn supports_binary_archives(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), supportsBinaryArchives]
        }
    }
    
    /// Checks if the device supports dynamic libraries.
    ///
    /// # Returns
    ///
    /// `true` if the device supports dynamic libraries, `false` otherwise.
    #[must_use]
    pub fn supports_dynamic_libraries(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), supportsDynamicLibraries]
        }
    }
    
    /// Creates a new log state with the specified descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the log state.
    ///
    /// # Returns
    ///
    /// A result containing the log state if successful, or an error if not.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLLogStateDescriptor, MTLLogLevel};
    /// 
    /// let device = MTLCreateSystemDefaultDevice();
    /// 
    /// let descriptor = MTLLogStateDescriptor::new();
    /// descriptor.set_level(MTLLogLevel::Info);
    /// descriptor.set_buffer_size(1024);
    /// 
    /// let log_state = device.new_log_state(&descriptor)
    ///     .expect("Failed to create log state");
    /// ```
    pub fn new_log_state(&self, descriptor: &MTLLogStateDescriptor) -> Result<MTLLogState, MTLLogStateError> {
        unsafe {
            let mut err: u64 = 0;
            
            let ptr: *mut Object = msg_send![self.as_ref(), newLogStateWithDescriptor:descriptor.as_ptr()
                                                           error:&mut err];
            
            if err != 0 {
                // Convert the error code to MTLLogStateError
                let error = std::mem::transmute(err);
                Err(error)
            } else if ptr.is_null() {
                Err(MTLLogStateError::Invalid)
            } else {
                Ok(MTLLogState::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new I/O command queue with the specified descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the I/O command queue.
    ///
    /// # Returns
    ///
    /// A new I/O command queue.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLIOCommandQueueDescriptor, MTLIOCommandQueueType, MTLIOPriority};
    /// 
    /// let device = MTLCreateSystemDefaultDevice();
    /// 
    /// let descriptor = MTLIOCommandQueueDescriptor::new();
    /// descriptor.set_max_command_buffer_count(64);
    /// descriptor.set_priority(MTLIOPriority::Normal);
    /// descriptor.set_queue_type(MTLIOCommandQueueType::Serial);
    /// 
    /// let queue = device.new_io_command_queue(&descriptor);
    /// ```
    #[must_use]
    pub fn new_io_command_queue(&self, descriptor: &MTLIOCommandQueueDescriptor) -> MTLIOCommandQueue {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newIOCommandQueueWithDescriptor:descriptor.as_ptr()];
            MTLIOCommandQueue::from_ptr(ptr)
        }
    }
    
    /// Creates a new I/O file handle.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the file.
    ///
    /// # Returns
    ///
    /// A new I/O file handle, or None if the file could not be opened.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal_rs::metal::MTLCreateSystemDefaultDevice;
    /// use metal_rs::foundation::NSURL;
    /// 
    /// let device = MTLCreateSystemDefaultDevice();
    /// 
    /// let url = NSURL::file_url_with_path("/path/to/file", false);
    /// let file_handle = device.new_io_file_handle(&url);
    /// ```
    #[must_use]
    pub fn new_io_file_handle(&self, url: &crate::foundation::NSURL) -> Option<crate::metal::io_command_buffer::MTLIOFileHandle> {
        unsafe {
            // Get the raw pointer to the NSURL
            let url_ptr = std::mem::transmute::<&crate::foundation::NSURL, *mut Object>(url);
            let ptr: *mut Object = msg_send![self.as_ref(), newIOFileHandleWithURL:url_ptr];
            if ptr.is_null() {
                None
            } else {
                Some(crate::metal::io_command_buffer::MTLIOFileHandle::from_ptr(ptr))
            }
        }
    }
    
    /// Creates a new visible function table from a descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the visible function table.
    ///
    /// # Returns
    ///
    /// A new visible function table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{Device, VisibleFunctionTableDescriptor};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = VisibleFunctionTableDescriptor::new();
    /// descriptor.set_function_count(4);
    /// 
    /// let function_table = device.new_visible_function_table(&descriptor);
    /// ```
    #[must_use]
    pub fn new_visible_function_table(&self, descriptor: &crate::metal::visible_function_table::MTLVisibleFunctionTableDescriptor) -> crate::metal::visible_function_table::MTLVisibleFunctionTable {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newVisibleFunctionTableWithDescriptor:descriptor.as_ptr()];
            crate::metal::visible_function_table::MTLVisibleFunctionTable::from_ptr(ptr)
        }
    }
    
    /// Creates a new intersection function table from a descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the intersection function table.
    ///
    /// # Returns
    ///
    /// A new intersection function table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{Device, IntersectionFunctionTableDescriptor};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = IntersectionFunctionTableDescriptor::new();
    /// descriptor.set_function_count(4);
    /// 
    /// let intersection_table = device.new_intersection_function_table(&descriptor);
    /// ```
    #[must_use]
    pub fn new_intersection_function_table(&self, descriptor: &crate::metal::intersection_function_table::MTLIntersectionFunctionTableDescriptor) -> crate::metal::intersection_function_table::MTLIntersectionFunctionTable {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newIntersectionFunctionTableWithDescriptor:descriptor.as_ptr()];
            crate::metal::intersection_function_table::MTLIntersectionFunctionTable::from_ptr(ptr)
        }
    }
    
    /// Calculates the memory requirements for an acceleration structure.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the acceleration structure.
    ///
    /// # Returns
    ///
    /// The size requirements for the acceleration structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{Device, AccelerationStructureDescriptor};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = AccelerationStructureDescriptor::new();
    /// 
    /// let sizes = device.acceleration_structure_sizes(&descriptor);
    /// println!("Acceleration structure size: {} bytes", sizes.acceleration_structure_size);
    /// println!("Build scratch buffer size: {} bytes", sizes.build_scratch_buffer_size);
    /// println!("Refit scratch buffer size: {} bytes", sizes.refit_scratch_buffer_size);
    /// ```
    #[must_use]
    pub fn acceleration_structure_sizes(&self, descriptor: &crate::metal::acceleration_structure::MTLAccelerationStructureDescriptor) -> crate::metal::acceleration_structure::MTLAccelerationStructureSizes {
        unsafe {
            msg_send![self.as_ref(), accelerationStructureSizesWithDescriptor:descriptor.as_ptr()]
        }
    }
    
    /// Creates a new acceleration structure with the specified size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size in bytes of the acceleration structure.
    ///
    /// # Returns
    ///
    /// A new acceleration structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{Device, AccelerationStructureDescriptor};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = AccelerationStructureDescriptor::new();
    /// let sizes = device.acceleration_structure_sizes(&descriptor);
    /// 
    /// let accel_struct = device.new_acceleration_structure(sizes.acceleration_structure_size);
    /// ```
    #[must_use]
    pub fn new_acceleration_structure(&self, size: crate::foundation::NSUInteger) -> crate::metal::acceleration_structure::MTLAccelerationStructure {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newAccelerationStructureWithSize:size];
            crate::metal::acceleration_structure::MTLAccelerationStructure::from_ptr(ptr)
        }
    }
    
    /// Creates a new acceleration structure from a descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the acceleration structure.
    ///
    /// # Returns
    ///
    /// A new acceleration structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{Device, AccelerationStructureDescriptor};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = AccelerationStructureDescriptor::new();
    /// 
    /// let accel_struct = device.new_acceleration_structure_with_descriptor(&descriptor);
    /// ```
    #[must_use]
    pub fn new_acceleration_structure_with_descriptor(&self, descriptor: &crate::metal::acceleration_structure::MTLAccelerationStructureDescriptor) -> crate::metal::acceleration_structure::MTLAccelerationStructure {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newAccelerationStructureWithDescriptor:descriptor.as_ptr()];
            crate::metal::acceleration_structure::MTLAccelerationStructure::from_ptr(ptr)
        }
    }
    
    /// Calculates the heap size and alignment requirements for an acceleration structure.
    ///
    /// # Arguments
    ///
    /// * `size` - The size in bytes of the acceleration structure.
    ///
    /// # Returns
    ///
    /// A tuple containing the size and alignment requirements for the acceleration structure in a heap.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{Device, AccelerationStructureDescriptor};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = AccelerationStructureDescriptor::new();
    /// let sizes = device.acceleration_structure_sizes(&descriptor);
    /// 
    /// let (size, align) = device.heap_acceleration_structure_size_and_align(sizes.acceleration_structure_size);
    /// println!("Heap size: {} bytes, alignment: {} bytes", size, align);
    /// ```
    #[must_use]
    pub fn heap_acceleration_structure_size_and_align(&self, size: crate::foundation::NSUInteger) -> (usize, usize) {
        unsafe {
            // SizeAndAlign struct
            #[repr(C)]
            struct SizeAndAlign {
                size: usize,
                align: usize,
            }

            let result: SizeAndAlign = msg_send![self.as_ref(), heapAccelerationStructureSizeAndAlignWithSize:size];
            (result.size, result.align)
        }
    }
    
    /// Calculates the heap size and alignment requirements for an acceleration structure from a descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for the acceleration structure.
    ///
    /// # Returns
    ///
    /// A tuple containing the size and alignment requirements for the acceleration structure in a heap.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal::{Device, AccelerationStructureDescriptor};
    /// 
    /// let device = Device::system_default().expect("No device found");
    /// 
    /// let descriptor = AccelerationStructureDescriptor::new();
    /// 
    /// let (size, align) = device.heap_acceleration_structure_size_and_align_with_descriptor(&descriptor);
    /// println!("Heap size: {} bytes, alignment: {} bytes", size, align);
    /// ```
    #[must_use]
    pub fn heap_acceleration_structure_size_and_align_with_descriptor(&self, descriptor: &crate::metal::acceleration_structure::MTLAccelerationStructureDescriptor) -> (usize, usize) {
        unsafe {
            // SizeAndAlign struct
            #[repr(C)]
            struct SizeAndAlign {
                size: usize,
                align: usize,
            }

            let result: SizeAndAlign = msg_send![self.as_ref(), heapAccelerationStructureSizeAndAlignWithDescriptor:descriptor.as_ptr()];
            (result.size, result.align)
        }
    }
}

impl fmt::Debug for MTLDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLDevice {{ name: \"{}\", low_power: {}, headless: {} }}", 
               self.name(), self.is_low_power(), self.is_headless())
    }
}

impl Drop for MTLDevice {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLDevice {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLDevice::from_ptr(obj)
        }
    }
}

/// Creates and returns a reference to the preferred system device.
#[allow(non_snake_case)]
#[must_use]
pub fn MTLCreateSystemDefaultDevice() -> MTLDevice {
    unsafe {
        #[link(name = "Metal", kind = "framework")]
        unsafe extern "C" {
            fn MTLCreateSystemDefaultDevice() -> *mut Object;
        }
        
        let device_ptr = MTLCreateSystemDefaultDevice();
        MTLDevice::from_ptr(device_ptr)
    }
}