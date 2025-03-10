//! MTLBuffer - A Rust wrapper around Metal's MTLBuffer class.
//!
//! This module provides safe Rust bindings to the MTLBuffer class from Apple's Metal framework.
//! MTLBuffer represents memory allocated for storing data that the GPU can access.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLResourceOptions};
//! use std::mem;
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Data to store in buffer
//! let vertices = [
//!     0.0f32, 0.5, 0.0,
//!     -0.5, -0.5, 0.0,
//!     0.5, -0.5, 0.0
//! ];
//! 
//! // Calculate size in bytes
//! let buffer_size = mem::size_of_val(&vertices);
//! 
//! // Create buffer from existing data
//! let vertex_buffer = device.new_buffer_with_data(
//!     vertices.as_ptr() as *const std::ffi::c_void,
//!     buffer_size,
//!     MTLResourceOptions::StorageModeShared
//! );
//! 
//! // Create empty buffer and fill it ourselves
//! let uniform_buffer = device.new_buffer(
//!     256, // size in bytes
//!     MTLResourceOptions::StorageModeShared
//! );
//! 
//! // Get contents pointer (only for shared or managed memory)
//! let contents = uniform_buffer.contents();
//! // In a real app, you'd fill this with your data
//! ```

use std::fmt;
use std::ptr;
use std::slice;
use std::ops::Range;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;

/// Options determining how memory for a resource is allocated and accessed.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLResourceOptions {
    /// The default option.
    CPUCacheModeDefaultCache = 0,
    
    /// CPU access is not needed.
    CPUCacheModeWriteCombined = 1,
    
    /// Resources are stored in system memory and are accessible by both the CPU and GPU.
    StorageModeManaged = 16,
    
    /// Resources are stored in memory only accessible from the GPU.
    StorageModePrivate = 32,
    
    /// Resources are stored on the device, but there will be memory accessible by both the CPU and GPU.
    StorageModeMemoryless = 48,
    
    /// A commonly used configuration for shared memory.
    StorageModeSharedCpuCacheModeWriteCombined = 17,
}

/// A reference to an Objective-C `MTLBuffer`.
pub struct MTLBufferRef(Object);

/// An owned Objective-C `MTLBuffer`.
pub struct MTLBuffer(*mut Object);

unsafe impl ForeignTypeRef for MTLBufferRef {
    type CType = Object;
}

unsafe impl Send for MTLBufferRef {}
unsafe impl Sync for MTLBufferRef {}

unsafe impl ForeignType for MTLBuffer {
    type CType = Object;
    type Ref = MTLBufferRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBuffer {
        MTLBuffer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBufferRef> for MTLBuffer {
    fn as_ref(&self) -> &MTLBufferRef {
        unsafe { &*(self.0.cast::<MTLBufferRef>()) }
    }
}

unsafe impl Send for MTLBuffer {}
unsafe impl Sync for MTLBuffer {}

unsafe impl objc::Message for MTLBufferRef {}

impl MTLBuffer {
    /// Returns the label of the buffer.
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
    
    /// Sets the label of the buffer.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Returns the size of the buffer.
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), length]
        }
    }
    
    /// Returns a pointer to the contents of the buffer.
    ///
    /// Note: This is only valid for buffers with MTLStorageModeShared or MTLStorageModeManaged.
    #[must_use]
    pub fn contents(&self) -> *mut std::ffi::c_void {
        unsafe {
            msg_send![self.as_ref(), contents]
        }
    }
    
    /// Returns the options used to create the buffer.
    #[must_use]
    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe {
            msg_send![self.as_ref(), resourceOptions]
        }
    }
    
    /// Returns a new buffer that represents a subsection of this buffer.
    ///
    /// Note: The original buffer will be retained, and both buffers will share the same backing memory.
    #[must_use]
    pub fn new_buffer_with_byte_range(&self, range: Range<usize>) -> MTLBuffer {
        let location = range.start;
        let length = range.end - range.start;
        
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newBufferWithBytesNoCopy:self.contents().byte_add(location)
                                                         length:length
                                                         options:self.resource_options()
                                                         deallocator:ptr::null_mut::<Object>()];
            MTLBuffer::from_ptr(ptr)
        }
    }
    
    /// Returns a slice that represents the contents of the buffer.
    ///
    /// Note: This is only valid for buffers with MTLStorageModeShared or MTLStorageModeManaged.
    #[must_use]
    pub fn as_slice<T>(&self) -> &[T] {
        unsafe {
            let count = self.length() / std::mem::size_of::<T>();
            slice::from_raw_parts(self.contents() as *const T, count)
        }
    }
    
    /// Returns a mutable slice that represents the contents of the buffer.
    ///
    /// Note: This is only valid for buffers with MTLStorageModeShared or MTLStorageModeManaged.
    #[must_use]
    pub fn as_slice_mut<T>(&mut self) -> &mut [T] {
        unsafe {
            let count = self.length() / std::mem::size_of::<T>();
            slice::from_raw_parts_mut(self.contents() as *mut T, count)
        }
    }
    
    /// Copies data from one buffer to another. Both buffers must have shared storage mode.
    pub fn copy_from(&mut self, source: &MTLBuffer, source_range: Range<usize>, destination_offset: usize) {
        let src_ptr = unsafe { (source.contents() as *const u8).add(source_range.start) };
        let dst_ptr = unsafe { (self.contents() as *mut u8).add(destination_offset) };
        let len = source_range.end - source_range.start;
        
        unsafe {
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        }
    }
    
    /// Copies data from a slice into the buffer. The buffer must have shared storage mode.
    pub fn copy_from_slice<T>(&mut self, data: &[T], offset: usize) {
        let src_ptr = data.as_ptr() as *const u8;
        let dst_ptr = unsafe { (self.contents() as *mut u8).add(offset) };
        let len = data.len() * std::mem::size_of::<T>();
        
        unsafe {
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        }
    }
    
    /// Returns whether the buffer was created with shared storage mode.
    #[must_use]
    pub fn is_shared_storage_mode(&self) -> bool {
        self.resource_options() as u64 & (3 << 4) == 0
    }
}

impl fmt::Debug for MTLBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLBuffer {{ label: {}, length: {} bytes }}", label, self.length())
    }
}

impl Drop for MTLBuffer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLBuffer::from_ptr(obj)
        }
    }
}