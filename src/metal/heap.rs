//! MTLHeap - A Rust wrapper around Metal's MTLHeap class.
//!
//! This module provides safe Rust bindings to the MTLHeap class from Apple's Metal framework.
//! MTLHeap represents a block of memory that can be used to allocate resources (buffers, textures) in a more
//! efficient way than allocating them individually.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLHeapDescriptor, MTLStorageMode, MTLHeapType};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a heap descriptor
//! let descriptor = MTLHeapDescriptor::new();
//! descriptor.set_size(1024 * 1024 * 16); // 16MB heap
//! descriptor.set_storage_mode(MTLStorageMode::Shared);
//! descriptor.set_type(MTLHeapType::Automatic);
//! 
//! // Create a heap using the descriptor
//! let heap = device.new_heap(&descriptor);
//! 
//! // Create a buffer from the heap
//! let buffer = heap.new_buffer(1024, metal_rs::metal::MTLResourceOptions::StorageModeShared);
//! 
//! // Create a texture from the heap
//! let texture_descriptor = metal_rs::metal::MTLTextureDescriptor::new();
//! texture_descriptor.set_width(256);
//! texture_descriptor.set_height(256);
//! texture_descriptor.set_pixel_format(metal_rs::metal::MTLPixelFormat::RGBA8Unorm);
//! let texture = heap.new_texture(&texture_descriptor);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSString, NSUInteger};
use crate::metal::device::MTLDeviceRef;
use crate::metal::{
    MTLStorageMode, MTLCPUCacheMode, MTLHazardTrackingMode, MTLResourceOptions,
    MTLPurgeableState, MTLBuffer, MTLTexture, MTLTextureDescriptorRef
};

/// MTLHeapType - Type of the heap.
#[allow(non_camel_case_types)]
#[repr(i64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLHeapType {
    /// Automatic memory placement
    Automatic = 0,
    /// Manual memory placement (offset-based allocation)
    Placement = 1,
    /// Sparse memory management
    Sparse = 2,
}

/// MTLSparsePageSize - Size of sparse texture pages.
#[allow(non_camel_case_types)]
#[repr(i64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLSparsePageSize {
    /// 16 KB page size
    Size16 = 101,
    /// 64 KB page size
    Size64 = 102,
    /// 256 KB page size
    Size256 = 103,
}

/// A reference to an Objective-C `MTLHeapDescriptor`.
pub struct MTLHeapDescriptorRef(Object);

/// An owned Objective-C `MTLHeapDescriptor`.
pub struct MTLHeapDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLHeapDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLHeapDescriptorRef {}
unsafe impl Sync for MTLHeapDescriptorRef {}

unsafe impl ForeignType for MTLHeapDescriptor {
    type CType = Object;
    type Ref = MTLHeapDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLHeapDescriptor {
        MTLHeapDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLHeapDescriptorRef> for MTLHeapDescriptor {
    fn as_ref(&self) -> &MTLHeapDescriptorRef {
        unsafe { &*(self.0.cast::<MTLHeapDescriptorRef>()) }
    }
}

unsafe impl Send for MTLHeapDescriptor {}
unsafe impl Sync for MTLHeapDescriptor {}

unsafe impl objc::Message for MTLHeapDescriptorRef {}

impl MTLHeapDescriptor {
    /// Creates a new heap descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLHeapDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLHeapDescriptor::from_ptr(obj)
        }
    }

    /// Sets the size of the heap.
    pub fn set_size(&self, size: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setSize:size]
        }
    }

    /// Gets the size of the heap.
    #[must_use]
    pub fn size(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), size]
        }
    }

    /// Sets the storage mode for resources allocated from this heap.
    pub fn set_storage_mode(&self, storage_mode: MTLStorageMode) {
        unsafe {
            msg_send![self.as_ref(), setStorageMode:storage_mode]
        }
    }

    /// Gets the storage mode for resources allocated from this heap.
    #[must_use]
    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self.as_ref(), storageMode]
        }
    }

    /// Sets the CPU cache mode for resources allocated from this heap.
    pub fn set_cpu_cache_mode(&self, cpu_cache_mode: MTLCPUCacheMode) {
        unsafe {
            msg_send![self.as_ref(), setCpuCacheMode:cpu_cache_mode]
        }
    }

    /// Gets the CPU cache mode for resources allocated from this heap.
    #[must_use]
    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self.as_ref(), cpuCacheMode]
        }
    }

    /// Sets the sparse page size for resources allocated from this heap.
    pub fn set_sparse_page_size(&self, sparse_page_size: MTLSparsePageSize) {
        unsafe {
            msg_send![self.as_ref(), setSparsePageSize:sparse_page_size]
        }
    }

    /// Gets the sparse page size for resources allocated from this heap.
    #[must_use]
    pub fn sparse_page_size(&self) -> MTLSparsePageSize {
        unsafe {
            msg_send![self.as_ref(), sparsePageSize]
        }
    }

    /// Sets the hazard tracking mode for resources allocated from this heap.
    pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: MTLHazardTrackingMode) {
        unsafe {
            msg_send![self.as_ref(), setHazardTrackingMode:hazard_tracking_mode]
        }
    }

    /// Gets the hazard tracking mode for resources allocated from this heap.
    #[must_use]
    pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode {
        unsafe {
            msg_send![self.as_ref(), hazardTrackingMode]
        }
    }

    /// Sets the resource options for resources allocated from this heap.
    pub fn set_resource_options(&self, resource_options: MTLResourceOptions) {
        unsafe {
            msg_send![self.as_ref(), setResourceOptions:resource_options]
        }
    }

    /// Gets the resource options for resources allocated from this heap.
    #[must_use]
    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe {
            msg_send![self.as_ref(), resourceOptions]
        }
    }

    /// Sets the type of the heap.
    pub fn set_type(&self, heap_type: MTLHeapType) {
        unsafe {
            msg_send![self.as_ref(), setType:heap_type]
        }
    }

    /// Gets the type of the heap.
    #[must_use]
    pub fn type_(&self) -> MTLHeapType {
        unsafe {
            msg_send![self.as_ref(), type]
        }
    }
}

impl Drop for MTLHeapDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLHeapDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLHeapDescriptor::from_ptr(obj)
        }
    }
}

impl fmt::Debug for MTLHeapDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLHeapDescriptor")
            .field("size", &self.size())
            .field("storage_mode", &self.storage_mode())
            .field("cpu_cache_mode", &self.cpu_cache_mode())
            .field("hazard_tracking_mode", &self.hazard_tracking_mode())
            .field("resource_options", &self.resource_options())
            .field("type", &self.type_())
            .finish()
    }
}

impl Default for MTLHeapDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

/// A reference to an Objective-C `MTLHeap`.
pub struct MTLHeapRef(Object);

/// An owned Objective-C `MTLHeap`.
pub struct MTLHeap(*mut Object);

unsafe impl ForeignTypeRef for MTLHeapRef {
    type CType = Object;
}

unsafe impl Send for MTLHeapRef {}
unsafe impl Sync for MTLHeapRef {}

unsafe impl ForeignType for MTLHeap {
    type CType = Object;
    type Ref = MTLHeapRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLHeap {
        MTLHeap(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLHeapRef> for MTLHeap {
    fn as_ref(&self) -> &MTLHeapRef {
        unsafe { &*(self.0.cast::<MTLHeapRef>()) }
    }
}

unsafe impl Send for MTLHeap {}
unsafe impl Sync for MTLHeap {}

unsafe impl objc::Message for MTLHeapRef {}

impl MTLHeap {
    /// Returns the device that created this heap.
    #[must_use]
    pub fn device(&self) -> &MTLDeviceRef {
        unsafe {
            let device: *mut Object = msg_send![self.as_ref(), device];
            &*(device as *mut MTLDeviceRef)
        }
    }

    /// Gets the label for this heap.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), label];
            if ns_string.is_null() {
                return String::new();
            }
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }

    /// Sets the label for this heap.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            msg_send![self.as_ref(), setLabel:ns_string.as_ptr()]
        }
    }

    /// Gets the storage mode of this heap.
    #[must_use]
    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self.as_ref(), storageMode]
        }
    }

    /// Gets the CPU cache mode of this heap.
    #[must_use]
    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self.as_ref(), cpuCacheMode]
        }
    }

    /// Gets the hazard tracking mode of this heap.
    #[must_use]
    pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode {
        unsafe {
            msg_send![self.as_ref(), hazardTrackingMode]
        }
    }

    /// Gets the resource options of this heap.
    #[must_use]
    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe {
            msg_send![self.as_ref(), resourceOptions]
        }
    }

    /// Gets the size of this heap in bytes.
    #[must_use]
    pub fn size(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), size]
        }
    }

    /// Gets the used size of this heap in bytes.
    #[must_use]
    pub fn used_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), usedSize]
        }
    }

    /// Gets the current allocated size of this heap in bytes.
    #[must_use]
    pub fn current_allocated_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), currentAllocatedSize]
        }
    }

    /// Gets the maximum available size that can be allocated from this heap with the given alignment.
    #[must_use]
    pub fn max_available_size(&self, alignment: NSUInteger) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), maxAvailableSizeWithAlignment:alignment]
        }
    }

    /// Creates a new buffer from this heap.
    #[must_use]
    pub fn new_buffer(&self, length: NSUInteger, options: MTLResourceOptions) -> MTLBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newBufferWithLength:length options:options];
            MTLBuffer::from_ptr(ptr)
        }
    }

    /// Creates a new buffer at a specific offset within this heap.
    #[must_use]
    pub fn new_buffer_with_offset(&self, length: NSUInteger, options: MTLResourceOptions, offset: NSUInteger) -> MTLBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newBufferWithLength:length options:options offset:offset];
            MTLBuffer::from_ptr(ptr)
        }
    }

    /// Creates a new texture from this heap.
    #[must_use]
    pub fn new_texture(&self, descriptor: &impl AsRef<MTLTextureDescriptorRef>) -> MTLTexture {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newTextureWithDescriptor:descriptor.as_ref().as_ptr()];
            MTLTexture::from_ptr(ptr)
        }
    }

    /// Creates a new texture at a specific offset within this heap.
    #[must_use]
    pub fn new_texture_with_offset(&self, descriptor: &impl AsRef<MTLTextureDescriptorRef>, offset: NSUInteger) -> MTLTexture {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newTextureWithDescriptor:descriptor.as_ref().as_ptr() offset:offset];
            MTLTexture::from_ptr(ptr)
        }
    }

    /// Sets the purgeable state for the heap.
    pub fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        unsafe {
            msg_send![self.as_ref(), setPurgeableState:state]
        }
    }

    /// Gets the heap type.
    #[must_use]
    pub fn type_(&self) -> MTLHeapType {
        unsafe {
            msg_send![self.as_ref(), type]
        }
    }
}

impl fmt::Debug for MTLHeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLHeap")
            .field("label", &self.label())
            .field("size", &self.size())
            .field("used_size", &self.used_size())
            .field("current_allocated_size", &self.current_allocated_size())
            .field("storage_mode", &self.storage_mode())
            .field("cpu_cache_mode", &self.cpu_cache_mode())
            .field("hazard_tracking_mode", &self.hazard_tracking_mode())
            .field("type", &self.type_())
            .finish()
    }
}

impl Drop for MTLHeap {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLHeap {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLHeap::from_ptr(obj)
        }
    }
}