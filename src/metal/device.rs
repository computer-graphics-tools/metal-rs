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
use crate::foundation::NSString;
use crate::metal::command_queue::MTLCommandQueue;
use crate::metal::event::{MTLEvent, MTLSharedEvent};

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
                                                                 options:options
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