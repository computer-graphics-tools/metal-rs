//! MTLAccelerationStructure - A Rust wrapper around Metal's MTLAccelerationStructure class.
//!
//! This module provides safe Rust bindings to the MTLAccelerationStructure class from Apple's Metal framework.
//! MTLAccelerationStructure represents a data structure used for ray tracing that accelerates ray-geometry intersection tests.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLAccelerationStructureDescriptor};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a descriptor
//! let descriptor = MTLAccelerationStructureDescriptor::new();
//! 
//! // Get the size requirements for the acceleration structure
//! let sizes = device.acceleration_structure_sizes(descriptor);
//! 
//! // Create an acceleration structure
//! let acceleration_structure = device.new_acceleration_structure(sizes.acceleration_structure_size);
//! 
//! // In a real application, you would build the acceleration structure using an
//! // MTLAccelerationStructureCommandEncoder
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSUInteger;
use crate::metal::resource::{MTLResource, MTLResourceRef};

/// Options for controlling the behavior of acceleration structures.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MTLAccelerationStructureUsage(pub u64);

impl MTLAccelerationStructureUsage {
    /// Default usage - no special options.
    pub const NONE: Self = Self(0);
    /// Optimize for structures that will be refit.
    pub const REFIT: Self = Self(1);
    /// Prefer fast build over runtime performance.
    pub const PREFER_FAST_BUILD: Self = Self(2);
    /// Support extended limits.
    pub const EXTENDED_LIMITS: Self = Self(4);
}

impl std::ops::BitOr for MTLAccelerationStructureUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for MTLAccelerationStructureUsage {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Describes the size requirements for an acceleration structure.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureSizes {
    /// Size in bytes required for the acceleration structure.
    pub acceleration_structure_size: NSUInteger,
    /// Size in bytes required for the build scratch buffer.
    pub build_scratch_buffer_size: NSUInteger,
    /// Size in bytes required for the refit scratch buffer.
    pub refit_scratch_buffer_size: NSUInteger,
}

/// A reference to an Objective-C `MTLAccelerationStructureDescriptor`.
pub struct MTLAccelerationStructureDescriptorRef(Object);

/// An owned Objective-C `MTLAccelerationStructureDescriptor`.
pub struct MTLAccelerationStructureDescriptor(*mut Object);

/// A reference to an Objective-C `MTLAccelerationStructure`.
pub struct MTLAccelerationStructureRef(Object);

/// An owned Objective-C `MTLAccelerationStructure`.
pub struct MTLAccelerationStructure(*mut Object);

// Implementation for MTLAccelerationStructureDescriptor
unsafe impl ForeignTypeRef for MTLAccelerationStructureDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLAccelerationStructureDescriptorRef {}
unsafe impl Sync for MTLAccelerationStructureDescriptorRef {}

unsafe impl ForeignType for MTLAccelerationStructureDescriptor {
    type CType = Object;
    type Ref = MTLAccelerationStructureDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLAccelerationStructureDescriptor {
        MTLAccelerationStructureDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLAccelerationStructureDescriptorRef> for MTLAccelerationStructureDescriptor {
    fn as_ref(&self) -> &MTLAccelerationStructureDescriptorRef {
        unsafe { &*(self.0.cast::<MTLAccelerationStructureDescriptorRef>()) }
    }
}

unsafe impl Send for MTLAccelerationStructureDescriptor {}
unsafe impl Sync for MTLAccelerationStructureDescriptor {}

unsafe impl objc::Message for MTLAccelerationStructureDescriptorRef {}

// Implementation for MTLAccelerationStructure
unsafe impl ForeignTypeRef for MTLAccelerationStructureRef {
    type CType = Object;
}

unsafe impl Send for MTLAccelerationStructureRef {}
unsafe impl Sync for MTLAccelerationStructureRef {}

unsafe impl ForeignType for MTLAccelerationStructure {
    type CType = Object;
    type Ref = MTLAccelerationStructureRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLAccelerationStructure {
        MTLAccelerationStructure(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLAccelerationStructureRef> for MTLAccelerationStructure {
    fn as_ref(&self) -> &MTLAccelerationStructureRef {
        unsafe { &*(self.0.cast::<MTLAccelerationStructureRef>()) }
    }
}

impl AsRef<MTLResourceRef> for MTLAccelerationStructure {
    fn as_ref(&self) -> &MTLResourceRef {
        unsafe { &*(self.0.cast::<MTLResourceRef>()) }
    }
}

unsafe impl Send for MTLAccelerationStructure {}
unsafe impl Sync for MTLAccelerationStructure {}

unsafe impl objc::Message for MTLAccelerationStructureRef {}

impl MTLAccelerationStructureDescriptor {
    /// Creates a new acceleration structure descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLAccelerationStructureDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLAccelerationStructureDescriptor::from_ptr(obj)
        }
    }

    /// Gets the usage of this acceleration structure.
    #[must_use]
    pub fn usage(&self) -> MTLAccelerationStructureUsage {
        unsafe {
            let usage: u64 = msg_send![self.as_ref(), usage];
            MTLAccelerationStructureUsage(usage)
        }
    }

    /// Sets the usage of this acceleration structure.
    pub fn set_usage(&self, usage: MTLAccelerationStructureUsage) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setUsage:usage.0];
        }
    }
}

impl MTLAccelerationStructureRef {
    /// Gets the size of this acceleration structure in bytes.
    #[must_use]
    pub fn size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, size]
        }
    }

    /// Gets the GPU resource ID of this acceleration structure.
    #[must_use]
    pub fn gpu_resource_id(&self) -> u64 {
        unsafe {
            msg_send![self, gpuResourceID]
        }
    }
}

impl MTLAccelerationStructure {
    /// Gets the size of this acceleration structure in bytes.
    #[must_use]
    pub fn size(&self) -> NSUInteger {
        let accel_struct_ref: &MTLAccelerationStructureRef = self.as_ref();
        accel_struct_ref.size()
    }

    /// Gets the GPU resource ID of this acceleration structure.
    #[must_use]
    pub fn gpu_resource_id(&self) -> u64 {
        let accel_struct_ref: &MTLAccelerationStructureRef = self.as_ref();
        accel_struct_ref.gpu_resource_id()
    }
}

impl fmt::Debug for MTLAccelerationStructureDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLAccelerationStructureDescriptor")
            .field("usage", &self.usage())
            .finish()
    }
}

impl fmt::Debug for MTLAccelerationStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLAccelerationStructure")
            .field("size", &self.size())
            .field("gpu_resource_id", &self.gpu_resource_id())
            .finish()
    }
}

impl Drop for MTLAccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLAccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLAccelerationStructureDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLAccelerationStructure {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLAccelerationStructure {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLAccelerationStructure::from_ptr(obj)
        }
    }
}

impl Default for MTLAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}