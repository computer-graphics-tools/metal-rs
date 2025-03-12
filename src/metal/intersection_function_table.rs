//! MTLIntersectionFunctionTable - A Rust wrapper around Metal's MTLIntersectionFunctionTable class.
//!
//! This module provides safe Rust bindings to the MTLIntersectionFunctionTable class from Apple's Metal framework.
//! MTLIntersectionFunctionTable represents a table of intersection functions used in ray tracing.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLIntersectionFunctionTableDescriptor};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a descriptor
//! let descriptor = MTLIntersectionFunctionTableDescriptor::new();
//! descriptor.set_function_count(4);
//! 
//! // Create an intersection function table
//! let intersection_table = device.new_intersection_function_table(&descriptor);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSUInteger, NSRange};
use crate::metal::resource::{MTLResource, MTLResourceRef};
use crate::metal::buffer::MTLBufferRef;
use crate::metal::visible_function_table::MTLVisibleFunctionTableRef;

/// Intersection function signature bit flags
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MTLIntersectionFunctionSignature(pub u64);

impl MTLIntersectionFunctionSignature {
    /// No special signatures
    pub const NONE: Self = Self(0);
    /// Support for instancing
    pub const INSTANCING: Self = Self(1);
    /// Support for triangle data
    pub const TRIANGLE_DATA: Self = Self(2);
    /// Support for world space data
    pub const WORLD_SPACE_DATA: Self = Self(4);
    /// Support for instance motion
    pub const INSTANCE_MOTION: Self = Self(8);
    /// Support for primitive motion
    pub const PRIMITIVE_MOTION: Self = Self(16);
    /// Support for extended limits
    pub const EXTENDED_LIMITS: Self = Self(32);
    /// Support for maximum levels
    pub const MAX_LEVELS: Self = Self(64);
    /// Support for curve data
    pub const CURVE_DATA: Self = Self(128);
}

impl std::ops::BitOr for MTLIntersectionFunctionSignature {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for MTLIntersectionFunctionSignature {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// A reference to an Objective-C `MTLIntersectionFunctionTableDescriptor`.
pub struct MTLIntersectionFunctionTableDescriptorRef(Object);

/// An owned Objective-C `MTLIntersectionFunctionTableDescriptor`.
pub struct MTLIntersectionFunctionTableDescriptor(*mut Object);

/// A reference to an Objective-C `MTLIntersectionFunctionTable`.
pub struct MTLIntersectionFunctionTableRef(Object);

/// An owned Objective-C `MTLIntersectionFunctionTable`.
pub struct MTLIntersectionFunctionTable(*mut Object);

// Implementation for MTLIntersectionFunctionTableDescriptor
unsafe impl ForeignTypeRef for MTLIntersectionFunctionTableDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLIntersectionFunctionTableDescriptorRef {}
unsafe impl Sync for MTLIntersectionFunctionTableDescriptorRef {}

unsafe impl ForeignType for MTLIntersectionFunctionTableDescriptor {
    type CType = Object;
    type Ref = MTLIntersectionFunctionTableDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIntersectionFunctionTableDescriptor {
        MTLIntersectionFunctionTableDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIntersectionFunctionTableDescriptorRef> for MTLIntersectionFunctionTableDescriptor {
    fn as_ref(&self) -> &MTLIntersectionFunctionTableDescriptorRef {
        unsafe { &*(self.0.cast::<MTLIntersectionFunctionTableDescriptorRef>()) }
    }
}

unsafe impl Send for MTLIntersectionFunctionTableDescriptor {}
unsafe impl Sync for MTLIntersectionFunctionTableDescriptor {}

unsafe impl objc::Message for MTLIntersectionFunctionTableDescriptorRef {}

// Implementation for MTLIntersectionFunctionTable
unsafe impl ForeignTypeRef for MTLIntersectionFunctionTableRef {
    type CType = Object;
}

unsafe impl Send for MTLIntersectionFunctionTableRef {}
unsafe impl Sync for MTLIntersectionFunctionTableRef {}

unsafe impl ForeignType for MTLIntersectionFunctionTable {
    type CType = Object;
    type Ref = MTLIntersectionFunctionTableRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIntersectionFunctionTable {
        MTLIntersectionFunctionTable(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIntersectionFunctionTableRef> for MTLIntersectionFunctionTable {
    fn as_ref(&self) -> &MTLIntersectionFunctionTableRef {
        unsafe { &*(self.0.cast::<MTLIntersectionFunctionTableRef>()) }
    }
}

impl AsRef<MTLResourceRef> for MTLIntersectionFunctionTable {
    fn as_ref(&self) -> &MTLResourceRef {
        unsafe { &*(self.0.cast::<MTLResourceRef>()) }
    }
}

unsafe impl Send for MTLIntersectionFunctionTable {}
unsafe impl Sync for MTLIntersectionFunctionTable {}

unsafe impl objc::Message for MTLIntersectionFunctionTableRef {}

impl MTLIntersectionFunctionTableDescriptor {
    /// Creates a new intersection function table descriptor.
    #[must_use]
    pub fn new() -> Self {
        Self::descriptor()
    }

    /// Creates a new intersection function table descriptor using the class method.
    #[must_use]
    pub fn descriptor() -> Self {
        unsafe {
            let class = class!(MTLIntersectionFunctionTableDescriptor);
            let obj: *mut Object = msg_send![class, intersectionFunctionTableDescriptor];
            MTLIntersectionFunctionTableDescriptor::from_ptr(obj)
        }
    }

    /// Returns the function count.
    #[must_use]
    pub fn function_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), functionCount]
        }
    }

    /// Sets the function count.
    pub fn set_function_count(&self, count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFunctionCount:count];
        }
    }
}

impl MTLIntersectionFunctionTable {
    /// Sets a buffer at the specified index.
    pub fn set_buffer(&self, buffer: Option<&MTLBufferRef>, offset: NSUInteger, index: NSUInteger) {
        unsafe {
            let buffer_ptr = match buffer {
                Some(buffer) => buffer.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self.as_ref(), setBuffer:buffer_ptr offset:offset atIndex:index];
        }
    }

    /// Sets multiple buffers in the specified range.
    pub fn set_buffers(&self, buffers: &[&MTLBufferRef], offsets: &[NSUInteger], range: NSRange) {
        unsafe {
            let buffers_ptr = buffers.as_ptr();
            let offsets_ptr = offsets.as_ptr();
            let _: () = msg_send![self.as_ref(), setBuffers:buffers_ptr offsets:offsets_ptr withRange:range];
        }
    }

    /// Sets a function in the table at the specified index.
    pub fn set_function(&self, function: Option<&crate::metal::function_handle::MTLFunctionHandleRef>, index: NSUInteger) {
        unsafe {
            let function_ptr = match function {
                Some(function) => function.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self.as_ref(), setFunction:function_ptr atIndex:index];
        }
    }

    /// Sets multiple functions in the table in the specified range.
    pub fn set_functions(&self, functions: &[&crate::metal::function_handle::MTLFunctionHandleRef], range: NSRange) {
        unsafe {
            let functions_ptr = functions.as_ptr();
            let _: () = msg_send![self.as_ref(), setFunctions:functions_ptr withRange:range];
        }
    }

    /// Sets an opaque triangle intersection function with the specified signature at the given index.
    pub fn set_opaque_triangle_intersection_function(&self, signature: MTLIntersectionFunctionSignature, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setOpaqueTriangleIntersectionFunctionWithSignature:signature.0 atIndex:index];
        }
    }

    /// Sets opaque triangle intersection functions with the specified signature for the given range.
    pub fn set_opaque_triangle_intersection_functions(&self, signature: MTLIntersectionFunctionSignature, range: NSRange) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setOpaqueTriangleIntersectionFunctionWithSignature:signature.0 withRange:range];
        }
    }

    /// Sets an opaque curve intersection function with the specified signature at the given index.
    pub fn set_opaque_curve_intersection_function(&self, signature: MTLIntersectionFunctionSignature, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setOpaqueCurveIntersectionFunctionWithSignature:signature.0 atIndex:index];
        }
    }

    /// Sets opaque curve intersection functions with the specified signature for the given range.
    pub fn set_opaque_curve_intersection_functions(&self, signature: MTLIntersectionFunctionSignature, range: NSRange) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setOpaqueCurveIntersectionFunctionWithSignature:signature.0 withRange:range];
        }
    }

    /// Sets a visible function table at the specified buffer index.
    pub fn set_visible_function_table(&self, function_table: Option<&MTLVisibleFunctionTableRef>, buffer_index: NSUInteger) {
        unsafe {
            let function_table_ptr = match function_table {
                Some(function_table) => function_table.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self.as_ref(), setVisibleFunctionTable:function_table_ptr atBufferIndex:buffer_index];
        }
    }

    /// Sets multiple visible function tables for the specified buffer range.
    pub fn set_visible_function_tables(&self, function_tables: &[&MTLVisibleFunctionTableRef], buffer_range: NSRange) {
        unsafe {
            let function_tables_ptr = function_tables.as_ptr();
            let _: () = msg_send![self.as_ref(), setVisibleFunctionTables:function_tables_ptr withBufferRange:buffer_range];
        }
    }

    /// Gets the GPU resource ID for this intersection function table.
    #[must_use]
    pub fn gpu_resource_id(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), gpuResourceID]
        }
    }
}

impl fmt::Debug for MTLIntersectionFunctionTableDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIntersectionFunctionTableDescriptor")
            .field("function_count", &self.function_count())
            .finish()
    }
}

impl fmt::Debug for MTLIntersectionFunctionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIntersectionFunctionTable")
            .field("gpu_resource_id", &self.gpu_resource_id())
            .finish()
    }
}

impl Drop for MTLIntersectionFunctionTableDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIntersectionFunctionTableDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLIntersectionFunctionTableDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLIntersectionFunctionTable {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIntersectionFunctionTable {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIntersectionFunctionTable::from_ptr(obj)
        }
    }
}

impl Default for MTLIntersectionFunctionTableDescriptor {
    fn default() -> Self {
        Self::new()
    }
}