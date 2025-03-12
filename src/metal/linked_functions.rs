//! MTLLinkedFunctions - A Rust wrapper around Metal's linked functions API.
//!
//! This module provides safe Rust bindings to the MTLLinkedFunctions class from Apple's Metal framework.
//! MTLLinkedFunctions is used to specify a set of functions to be linked together in a pipeline.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLLinkedFunctions};
//! use metal_rs::foundation::NSArray;
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a compute pipeline descriptor
//! let pipeline_descriptor = metal_rs::metal::MTLComputePipelineDescriptor::new();
//! 
//! // Configure pipeline for a compute shader
//! let library = device.new_library_with_source(SHADER_SRC, &Default::default()).unwrap();
//! let compute_function = library.get_function("compute_main").unwrap();
//! let helper_function = library.get_function("helper_function").unwrap();
//! 
//! // Create linked functions
//! let linked_functions = MTLLinkedFunctions::new();
//! let functions = NSArray::from_slice(&[&helper_function]);
//! linked_functions.set_functions(&functions);
//! 
//! // Set linked functions on pipeline descriptor
//! pipeline_descriptor.set_linked_functions(&linked_functions);
//! pipeline_descriptor.set_compute_function(&compute_function);
//! ```

use std::fmt;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSArray, NSArrayRef};

/// A reference to an Objective-C `MTLLinkedFunctions`.
pub struct MTLLinkedFunctionsRef(Object);

/// An owned Objective-C `MTLLinkedFunctions`.
pub struct MTLLinkedFunctions(*mut Object);

unsafe impl ForeignTypeRef for MTLLinkedFunctionsRef {
    type CType = Object;
}

unsafe impl Send for MTLLinkedFunctionsRef {}
unsafe impl Sync for MTLLinkedFunctionsRef {}

unsafe impl ForeignType for MTLLinkedFunctions {
    type CType = Object;
    type Ref = MTLLinkedFunctionsRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLLinkedFunctions {
        MTLLinkedFunctions(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLLinkedFunctionsRef> for MTLLinkedFunctions {
    fn as_ref(&self) -> &MTLLinkedFunctionsRef {
        unsafe { &*(self.0.cast::<MTLLinkedFunctionsRef>()) }
    }
}

unsafe impl Send for MTLLinkedFunctions {}
unsafe impl Sync for MTLLinkedFunctions {}

unsafe impl objc::Message for MTLLinkedFunctionsRef {}

impl MTLLinkedFunctions {
    /// Creates a new MTLLinkedFunctions instance.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLLinkedFunctions);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            Self::from_ptr(obj)
        }
    }
    
    /// Creates a new MTLLinkedFunctions instance using the class factory method.
    #[must_use]
    pub fn linked_functions() -> Self {
        unsafe {
            let class = class!(MTLLinkedFunctions);
            let obj: *mut Object = msg_send![class, linkedFunctions];
            Self::from_ptr(obj)
        }
    }
    
    /// Gets the array of functions in the linked functions object.
    #[must_use]
    pub fn functions(&self) -> Option<NSArray> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), functions];
            if ptr.is_null() {
                None
            } else {
                Some(NSArray::from_ptr(ptr))
            }
        }
    }
    
    /// Sets the array of functions in the linked functions object.
    pub fn set_functions(&self, functions: &impl AsRef<NSArrayRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFunctions:functions.as_ref().as_ptr()];
        }
    }
    
    /// Gets the array of binary functions in the linked functions object.
    #[must_use]
    pub fn binary_functions(&self) -> Option<NSArray> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), binaryFunctions];
            if ptr.is_null() {
                None
            } else {
                Some(NSArray::from_ptr(ptr))
            }
        }
    }
    
    /// Sets the array of binary functions in the linked functions object.
    pub fn set_binary_functions(&self, binary_functions: &impl AsRef<NSArrayRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setBinaryFunctions:binary_functions.as_ref().as_ptr()];
        }
    }
    
    /// Gets the dictionary of function groups in the linked functions object.
    ///
    /// Note: The dictionary API is not fully implemented yet,
    /// so this method is provided for future compatibility.
    #[must_use]
    pub fn has_groups(&self) -> bool {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), groups];
            !ptr.is_null()
        }
    }
    
    /// Sets the dictionary of function groups in the linked functions object.
    ///
    /// Note: The dictionary API is not fully implemented yet,
    /// so this method is not available at this time.
    pub fn set_groups_unavailable(&self) {
        // Not implemented yet - future API
    }
    
    /// Gets the array of private functions in the linked functions object.
    #[must_use]
    pub fn private_functions(&self) -> Option<NSArray> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), privateFunctions];
            if ptr.is_null() {
                None
            } else {
                Some(NSArray::from_ptr(ptr))
            }
        }
    }
    
    /// Sets the array of private functions in the linked functions object.
    pub fn set_private_functions(&self, private_functions: &impl AsRef<NSArrayRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setPrivateFunctions:private_functions.as_ref().as_ptr()];
        }
    }
}

impl fmt::Debug for MTLLinkedFunctions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLLinkedFunctions")
            .field("functions_count", &self.functions().map(|f| f.count()))
            .field("binary_functions_count", &self.binary_functions().map(|f| f.count()))
            .field("has_groups", &self.has_groups())
            .field("private_functions_count", &self.private_functions().map(|f| f.count()))
            .finish()
    }
}

impl Drop for MTLLinkedFunctions {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLLinkedFunctions {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLLinkedFunctions::from_ptr(obj)
        }
    }
}

impl Default for MTLLinkedFunctions {
    fn default() -> Self {
        Self::new()
    }
}