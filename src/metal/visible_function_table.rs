//! MTLVisibleFunctionTable - A Rust wrapper around Metal's MTLVisibleFunctionTable class.
//!
//! This module provides safe Rust bindings to the MTLVisibleFunctionTable class from Apple's Metal framework.
//! MTLVisibleFunctionTable represents a table of function references that can be used in shaders.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLVisibleFunctionTableDescriptor};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a descriptor
//! let descriptor = MTLVisibleFunctionTableDescriptor::new();
//! descriptor.set_function_count(4);
//! 
//! // Create a visible function table
//! let function_table = device.new_visible_function_table(&descriptor);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSUInteger, NSRange};
use crate::metal::resource::{MTLResource, MTLResourceRef};

/// A reference to an Objective-C `MTLVisibleFunctionTableDescriptor`.
pub struct MTLVisibleFunctionTableDescriptorRef(Object);

/// An owned Objective-C `MTLVisibleFunctionTableDescriptor`.
pub struct MTLVisibleFunctionTableDescriptor(*mut Object);

/// A reference to an Objective-C `MTLVisibleFunctionTable`.
pub struct MTLVisibleFunctionTableRef(Object);

/// An owned Objective-C `MTLVisibleFunctionTable`.
pub struct MTLVisibleFunctionTable(*mut Object);

// Implementation for MTLVisibleFunctionTableDescriptor
unsafe impl ForeignTypeRef for MTLVisibleFunctionTableDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLVisibleFunctionTableDescriptorRef {}
unsafe impl Sync for MTLVisibleFunctionTableDescriptorRef {}

unsafe impl ForeignType for MTLVisibleFunctionTableDescriptor {
    type CType = Object;
    type Ref = MTLVisibleFunctionTableDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLVisibleFunctionTableDescriptor {
        MTLVisibleFunctionTableDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLVisibleFunctionTableDescriptorRef> for MTLVisibleFunctionTableDescriptor {
    fn as_ref(&self) -> &MTLVisibleFunctionTableDescriptorRef {
        unsafe { &*(self.0.cast::<MTLVisibleFunctionTableDescriptorRef>()) }
    }
}

unsafe impl Send for MTLVisibleFunctionTableDescriptor {}
unsafe impl Sync for MTLVisibleFunctionTableDescriptor {}

unsafe impl objc::Message for MTLVisibleFunctionTableDescriptorRef {}

// Implementation for MTLVisibleFunctionTable
unsafe impl ForeignTypeRef for MTLVisibleFunctionTableRef {
    type CType = Object;
}

unsafe impl Send for MTLVisibleFunctionTableRef {}
unsafe impl Sync for MTLVisibleFunctionTableRef {}

unsafe impl ForeignType for MTLVisibleFunctionTable {
    type CType = Object;
    type Ref = MTLVisibleFunctionTableRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLVisibleFunctionTable {
        MTLVisibleFunctionTable(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLVisibleFunctionTableRef> for MTLVisibleFunctionTable {
    fn as_ref(&self) -> &MTLVisibleFunctionTableRef {
        unsafe { &*(self.0.cast::<MTLVisibleFunctionTableRef>()) }
    }
}

impl AsRef<MTLResourceRef> for MTLVisibleFunctionTable {
    fn as_ref(&self) -> &MTLResourceRef {
        unsafe { &*(self.0.cast::<MTLResourceRef>()) }
    }
}

unsafe impl Send for MTLVisibleFunctionTable {}
unsafe impl Sync for MTLVisibleFunctionTable {}

unsafe impl objc::Message for MTLVisibleFunctionTableRef {}

impl MTLVisibleFunctionTableDescriptor {
    /// Creates a new function table descriptor.
    #[must_use]
    pub fn new() -> Self {
        Self::descriptor()
    }

    /// Creates a new function table descriptor using the class method.
    #[must_use]
    pub fn descriptor() -> Self {
        unsafe {
            let class = class!(MTLVisibleFunctionTableDescriptor);
            let obj: *mut Object = msg_send![class, visibleFunctionTableDescriptor];
            MTLVisibleFunctionTableDescriptor::from_ptr(obj)
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

impl MTLVisibleFunctionTable {
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

    /// Gets the GPU resource ID for this visible function table.
    #[must_use]
    pub fn gpu_resource_id(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), gpuResourceID]
        }
    }
}

impl fmt::Debug for MTLVisibleFunctionTableDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLVisibleFunctionTableDescriptor")
            .field("function_count", &self.function_count())
            .finish()
    }
}

impl fmt::Debug for MTLVisibleFunctionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLVisibleFunctionTable")
            .field("gpu_resource_id", &self.gpu_resource_id())
            .finish()
    }
}

impl Drop for MTLVisibleFunctionTableDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLVisibleFunctionTableDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLVisibleFunctionTableDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLVisibleFunctionTable {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLVisibleFunctionTable {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLVisibleFunctionTable::from_ptr(obj)
        }
    }
}

impl Default for MTLVisibleFunctionTableDescriptor {
    fn default() -> Self {
        Self::new()
    }
}