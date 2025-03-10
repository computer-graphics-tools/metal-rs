//! MTLLibrary and MTLFunction - Rust wrappers around Metal's shader library and function classes.
//!
//! This module provides safe Rust bindings to the MTLLibrary and MTLFunction classes from Apple's Metal framework.
//! MTLLibrary represents a collection of shader functions, and MTLFunction represents a single shader function.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::MTLCreateSystemDefaultDevice;
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a library from MSL source code
//! let source = r#"
//!     #include <metal_stdlib>
//!     using namespace metal;
//!     
//!     vertex float4 vertex_main(uint vertexID [[vertex_id]]) {
//!         const float4 positions[3] = {
//!             float4(-0.5, -0.5, 0.0, 1.0),
//!             float4( 0.0,  0.5, 0.0, 1.0),
//!             float4( 0.5, -0.5, 0.0, 1.0)
//!         };
//!         return positions[vertexID];
//!     }
//!     
//!     fragment float4 fragment_main() {
//!         return float4(1.0, 0.0, 0.0, 1.0);
//!     }
//! "#;
//! 
//! let library = device.new_library_with_source(source, &Default::default()).unwrap();
//! 
//! // Get shader functions from the library
//! let vertex_function = library.get_function("vertex_main").unwrap();
//! let fragment_function = library.get_function("fragment_main").unwrap();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;

/// A reference to an Objective-C `MTLLibrary`.
pub struct MTLLibraryRef(Object);

/// An owned Objective-C `MTLLibrary`.
pub struct MTLLibrary(*mut Object);

/// A reference to an Objective-C `MTLFunction`.
pub struct MTLFunctionRef(Object);

/// An owned Objective-C `MTLFunction`.
pub struct MTLFunction(*mut Object);

/// A reference to an Objective-C `MTLFunctionConstantValues`.
pub struct MTLFunctionConstantValuesRef(Object);

/// An owned Objective-C `MTLFunctionConstantValues`.
pub struct MTLFunctionConstantValues(*mut Object);

/// Options for creating a Metal library.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MTLCompileOptions {
    // This would normally contain options for the compiler,
    // but we'll just use default values for simplicity.
    _private: (),
}

impl Default for MTLCompileOptions {
    fn default() -> Self {
        MTLCompileOptions { _private: () }
    }
}

// Implementation for MTLLibrary
unsafe impl ForeignTypeRef for MTLLibraryRef {
    type CType = Object;
}

unsafe impl Send for MTLLibraryRef {}
unsafe impl Sync for MTLLibraryRef {}

unsafe impl ForeignType for MTLLibrary {
    type CType = Object;
    type Ref = MTLLibraryRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLLibrary {
        MTLLibrary(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLLibraryRef> for MTLLibrary {
    fn as_ref(&self) -> &MTLLibraryRef {
        unsafe { &*(self.0.cast::<MTLLibraryRef>()) }
    }
}

unsafe impl Send for MTLLibrary {}
unsafe impl Sync for MTLLibrary {}

unsafe impl objc::Message for MTLLibraryRef {}

// Implementation for MTLFunction
unsafe impl ForeignTypeRef for MTLFunctionRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionRef {}
unsafe impl Sync for MTLFunctionRef {}

// Implementation for MTLFunctionConstantValues
unsafe impl ForeignTypeRef for MTLFunctionConstantValuesRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionConstantValuesRef {}
unsafe impl Sync for MTLFunctionConstantValuesRef {}

unsafe impl ForeignType for MTLFunction {
    type CType = Object;
    type Ref = MTLFunctionRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunction {
        MTLFunction(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

unsafe impl ForeignType for MTLFunctionConstantValues {
    type CType = Object;
    type Ref = MTLFunctionConstantValuesRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionConstantValues {
        MTLFunctionConstantValues(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionRef> for MTLFunction {
    fn as_ref(&self) -> &MTLFunctionRef {
        unsafe { &*(self.0.cast::<MTLFunctionRef>()) }
    }
}

impl AsRef<MTLFunctionConstantValuesRef> for MTLFunctionConstantValues {
    fn as_ref(&self) -> &MTLFunctionConstantValuesRef {
        unsafe { &*(self.0.cast::<MTLFunctionConstantValuesRef>()) }
    }
}

unsafe impl Send for MTLFunction {}
unsafe impl Sync for MTLFunction {}

unsafe impl Send for MTLFunctionConstantValues {}
unsafe impl Sync for MTLFunctionConstantValues {}

unsafe impl objc::Message for MTLFunctionRef {}
unsafe impl objc::Message for MTLFunctionConstantValuesRef {}

/// A simple error type for library operations.
#[derive(Debug, Clone)]
pub struct LibraryError(pub String);

impl std::fmt::Display for LibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Metal library error: {}", self.0)
    }
}

impl std::error::Error for LibraryError {}

impl MTLLibrary {
    /// Returns the label of the library.
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
    
    /// Sets the label of the library.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Retrieves a shader function from the library by name.
    pub fn get_function(&self, name: &str) -> Result<MTLFunction, LibraryError> {
        self.get_function_with_constants(name, None)
    }
    
    /// Retrieves a shader function from the library by name with constant values.
    pub fn get_function_with_constants(&self, name: &str, constant_values: Option<&MTLFunctionConstantValues>) -> Result<MTLFunction, LibraryError> {
        unsafe {
            let ns_string = NSString::from_rust_str(name);
            
            if constant_values.is_none() {
                // Use the simpler method if no constants are provided
                let ptr: *mut Object = msg_send![self.as_ref(), newFunctionWithName:ns_string.as_ptr()];
                
                if ptr.is_null() {
                    return Err(LibraryError(format!("Function '{}' not found in library", name)));
                } else {
                    return Ok(MTLFunction::from_ptr(ptr));
                }
            }
            
            let mut err: *mut Object = std::ptr::null_mut();
            let constants_ptr = constant_values.map_or(std::ptr::null_mut(), |c| c.as_ptr());
            
            let ptr: *mut Object = msg_send![self.as_ref(), newFunctionWithName:ns_string.as_ptr()
                                                    constantValues:constants_ptr
                                                            error:&mut err];
            
            if !err.is_null() {
                let error = NSString::from_ptr(msg_send![err, localizedDescription]);
                let error_str = error.to_rust_string();
                Err(LibraryError(error_str))
            } else if ptr.is_null() {
                Err(LibraryError(format!("Function '{}' not found in library", name)))
            } else {
                Ok(MTLFunction::from_ptr(ptr))
            }
        }
    }
    
    /// Returns the device that created this library.
    #[must_use]
    pub fn device(&self) -> crate::metal::MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            crate::metal::MTLDevice::from_ptr(ptr)
        }
    }
}

impl MTLFunction {
    /// Returns the label of the function.
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
    
    /// Sets the label of the function.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Returns the name of the function.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), name];
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }
    
    /// Returns the library from which this function was created.
    #[must_use]
    pub fn library(&self) -> MTLLibrary {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), library];
            MTLLibrary::from_ptr(ptr)
        }
    }
    
    /// Returns the device that created this function.
    #[must_use]
    pub fn device(&self) -> crate::metal::MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            crate::metal::MTLDevice::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLLibrary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLLibrary {{ label: {} }}", label)
    }
}

impl fmt::Debug for MTLFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name();
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLFunction {{ name: {}, label: {} }}", name, label)
    }
}

impl Drop for MTLLibrary {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLLibrary {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLLibrary::from_ptr(obj)
        }
    }
}

impl Drop for MTLFunction {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunction {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunction::from_ptr(obj)
        }
    }
}

impl Drop for MTLFunctionConstantValues {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionConstantValues {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionConstantValues::from_ptr(obj)
        }
    }
}

impl MTLFunctionConstantValues {
    /// Creates a new function constant values object.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = objc::class!(MTLFunctionConstantValues);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLFunctionConstantValues::from_ptr(obj)
        }
    }
}

impl Default for MTLFunctionConstantValues {
    fn default() -> Self {
        Self::new()
    }
}