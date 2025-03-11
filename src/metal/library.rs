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

/// Define the range structure to match Metal's NSRange
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRange {
    /// The start index
    pub location: usize,
    /// The number of elements
    pub length: usize,
}

/// An enum representing the data types supported in Metal.
/// This matches the Metal API's MTLDataType enum.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLDataType {
    /// No data type or unknown data type
    None = 0,
    
    /// Struct type
    Struct = 1,
    
    /// Array type
    Array = 2,
    
    /// 32-bit floating-point
    Float = 3,
    
    /// 32-bit floating-point vector of length 2
    Float2 = 4,
    
    /// 32-bit floating-point vector of length 3
    Float3 = 5,
    
    /// 32-bit floating-point vector of length 4
    Float4 = 6,
    
    /// 32-bit floating-point 2x2 matrix
    Float2x2 = 7,
    
    /// 32-bit floating-point 2x3 matrix
    Float2x3 = 8,
    
    /// 32-bit floating-point 2x4 matrix
    Float2x4 = 9,
    
    /// 32-bit floating-point 3x2 matrix
    Float3x2 = 10,
    
    /// 32-bit floating-point 3x3 matrix
    Float3x3 = 11,
    
    /// 32-bit floating-point 3x4 matrix
    Float3x4 = 12,
    
    /// 32-bit floating-point 4x2 matrix
    Float4x2 = 13,
    
    /// 32-bit floating-point 4x3 matrix
    Float4x3 = 14,
    
    /// 32-bit floating-point 4x4 matrix
    Float4x4 = 15,
    
    /// 16-bit floating-point
    Half = 16,
    
    /// 16-bit floating-point vector of length 2
    Half2 = 17,
    
    /// 16-bit floating-point vector of length 3
    Half3 = 18,
    
    /// 16-bit floating-point vector of length 4
    Half4 = 19,
    
    /// 16-bit floating-point 2x2 matrix
    Half2x2 = 20,
    
    /// 16-bit floating-point 2x3 matrix
    Half2x3 = 21,
    
    /// 16-bit floating-point 2x4 matrix
    Half2x4 = 22,
    
    /// 16-bit floating-point 3x2 matrix
    Half3x2 = 23,
    
    /// 16-bit floating-point 3x3 matrix
    Half3x3 = 24,
    
    /// 16-bit floating-point 3x4 matrix
    Half3x4 = 25,
    
    /// 16-bit floating-point 4x2 matrix
    Half4x2 = 26,
    
    /// 16-bit floating-point 4x3 matrix
    Half4x3 = 27,
    
    /// 16-bit floating-point 4x4 matrix
    Half4x4 = 28,
    
    /// 32-bit signed integer
    Int = 29,
    
    /// 32-bit signed integer vector of length 2
    Int2 = 30,
    
    /// 32-bit signed integer vector of length 3
    Int3 = 31,
    
    /// 32-bit signed integer vector of length 4
    Int4 = 32,
    
    /// 32-bit unsigned integer
    UInt = 33,
    
    /// 32-bit unsigned integer vector of length 2
    UInt2 = 34,
    
    /// 32-bit unsigned integer vector of length 3
    UInt3 = 35,
    
    /// 32-bit unsigned integer vector of length 4
    UInt4 = 36,
    
    /// 8-bit signed integer
    Char = 45,
    
    /// 8-bit signed integer vector of length 2
    Char2 = 46,
    
    /// 8-bit signed integer vector of length 3
    Char3 = 47,
    
    /// 8-bit signed integer vector of length 4
    Char4 = 48,
    
    /// 8-bit unsigned integer
    UChar = 49,
    
    /// 8-bit unsigned integer vector of length 2
    UChar2 = 50,
    
    /// 8-bit unsigned integer vector of length 3
    UChar3 = 51,
    
    /// 8-bit unsigned integer vector of length 4
    UChar4 = 52,
    
    /// 16-bit signed integer
    Short = 53,
    
    /// 16-bit signed integer vector of length 2
    Short2 = 54,
    
    /// 16-bit signed integer vector of length 3
    Short3 = 55,
    
    /// 16-bit signed integer vector of length 4
    Short4 = 56,
    
    /// 16-bit unsigned integer
    UShort = 57,
    
    /// 16-bit unsigned integer vector of length 2
    UShort2 = 58,
    
    /// 16-bit unsigned integer vector of length 3
    UShort3 = 59,
    
    /// 16-bit unsigned integer vector of length 4
    UShort4 = 60,
    
    /// Boolean
    Bool = 61,
    
    /// Boolean vector of length 2
    Bool2 = 62,
    
    /// Boolean vector of length 3
    Bool3 = 63,
    
    /// Boolean vector of length 4
    Bool4 = 64,
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
    
    /// Sets a constant value by index.
    ///
    /// # Arguments
    ///
    /// * `value` - A pointer to the constant value.
    /// * `type` - The data type of the constant.
    /// * `index` - The index of the constant.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `value` points to a valid memory location containing
    /// data of the correct type and size for the specified `type`.
    pub unsafe fn set_constant_value(&self, value: *const std::ffi::c_void, type_: MTLDataType, index: usize) {
        let _: () = msg_send![self.as_ref(), setConstantValue:value type:type_ as u64 index:index];
    }
    
    /// Sets a constant value by name.
    ///
    /// # Arguments
    ///
    /// * `value` - A pointer to the constant value.
    /// * `type` - The data type of the constant.
    /// * `name` - The name of the constant.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `value` points to a valid memory location containing
    /// data of the correct type and size for the specified `type`.
    pub unsafe fn set_constant_value_with_name(&self, value: *const std::ffi::c_void, type_: MTLDataType, name: &str) {
        let ns_string = NSString::from_rust_str(name);
        let _: () = msg_send![self.as_ref(), setConstantValue:value type:type_ as u64 name:ns_string.as_ptr()];
    }
    
    /// Sets multiple constant values in a range.
    ///
    /// # Arguments
    ///
    /// * `values` - A pointer to the constant values.
    /// * `type` - The data type of the constants.
    /// * `range` - The range of constants to set.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `values` points to a valid memory location containing
    /// an array of values of the correct type and size for the specified `type`, and that
    /// the array contains at least `range.length` elements.
    pub unsafe fn set_constant_values(&self, values: *const std::ffi::c_void, type_: MTLDataType, range: MTLRange) {
        let _: () = msg_send![self.as_ref(), setConstantValues:values type:type_ as u64 range:range];
    }
    
    /// Resets all constant values.
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), reset];
        }
    }
    
    /// Type-safe wrapper for setting a single f32 constant value by index.
    pub fn set_float_constant(&self, value: f32, index: usize) {
        unsafe {
            self.set_constant_value(&value as *const f32 as *const std::ffi::c_void, MTLDataType::Float, index);
        }
    }
    
    /// Type-safe wrapper for setting a single f32 constant value by name.
    pub fn set_float_constant_by_name(&self, value: f32, name: &str) {
        unsafe {
            self.set_constant_value_with_name(&value as *const f32 as *const std::ffi::c_void, MTLDataType::Float, name);
        }
    }
    
    /// Type-safe wrapper for setting a single i32 constant value by index.
    pub fn set_int_constant(&self, value: i32, index: usize) {
        unsafe {
            self.set_constant_value(&value as *const i32 as *const std::ffi::c_void, MTLDataType::Int, index);
        }
    }
    
    /// Type-safe wrapper for setting a single i32 constant value by name.
    pub fn set_int_constant_by_name(&self, value: i32, name: &str) {
        unsafe {
            self.set_constant_value_with_name(&value as *const i32 as *const std::ffi::c_void, MTLDataType::Int, name);
        }
    }
    
    /// Type-safe wrapper for setting a single u32 constant value by index.
    pub fn set_uint_constant(&self, value: u32, index: usize) {
        unsafe {
            self.set_constant_value(&value as *const u32 as *const std::ffi::c_void, MTLDataType::UInt, index);
        }
    }
    
    /// Type-safe wrapper for setting a single u32 constant value by name.
    pub fn set_uint_constant_by_name(&self, value: u32, name: &str) {
        unsafe {
            self.set_constant_value_with_name(&value as *const u32 as *const std::ffi::c_void, MTLDataType::UInt, name);
        }
    }
    
    /// Type-safe wrapper for setting a single bool constant value by index.
    pub fn set_bool_constant(&self, value: bool, index: usize) {
        unsafe {
            self.set_constant_value(&value as *const bool as *const std::ffi::c_void, MTLDataType::Bool, index);
        }
    }
    
    /// Type-safe wrapper for setting a single bool constant value by name.
    pub fn set_bool_constant_by_name(&self, value: bool, name: &str) {
        unsafe {
            self.set_constant_value_with_name(&value as *const bool as *const std::ffi::c_void, MTLDataType::Bool, name);
        }
    }
    
    /// Type-safe wrapper for setting a [f32; 2] constant value by index.
    pub fn set_float2_constant(&self, value: [f32; 2], index: usize) {
        unsafe {
            self.set_constant_value(&value as *const [f32; 2] as *const std::ffi::c_void, MTLDataType::Float2, index);
        }
    }
    
    /// Type-safe wrapper for setting a [f32; 2] constant value by name.
    pub fn set_float2_constant_by_name(&self, value: [f32; 2], name: &str) {
        unsafe {
            self.set_constant_value_with_name(&value as *const [f32; 2] as *const std::ffi::c_void, MTLDataType::Float2, name);
        }
    }
    
    /// Type-safe wrapper for setting a [f32; 3] constant value by index.
    pub fn set_float3_constant(&self, value: [f32; 3], index: usize) {
        unsafe {
            self.set_constant_value(&value as *const [f32; 3] as *const std::ffi::c_void, MTLDataType::Float3, index);
        }
    }
    
    /// Type-safe wrapper for setting a [f32; 3] constant value by name.
    pub fn set_float3_constant_by_name(&self, value: [f32; 3], name: &str) {
        unsafe {
            self.set_constant_value_with_name(&value as *const [f32; 3] as *const std::ffi::c_void, MTLDataType::Float3, name);
        }
    }
    
    /// Type-safe wrapper for setting a [f32; 4] constant value by index.
    pub fn set_float4_constant(&self, value: [f32; 4], index: usize) {
        unsafe {
            self.set_constant_value(&value as *const [f32; 4] as *const std::ffi::c_void, MTLDataType::Float4, index);
        }
    }
    
    /// Type-safe wrapper for setting a [f32; 4] constant value by name.
    pub fn set_float4_constant_by_name(&self, value: [f32; 4], name: &str) {
        unsafe {
            self.set_constant_value_with_name(&value as *const [f32; 4] as *const std::ffi::c_void, MTLDataType::Float4, name);
        }
    }
    
    /// Type-safe wrapper for setting multiple f32 constant values in a range.
    pub fn set_float_constants(&self, values: &[f32], start_index: usize) {
        let range = MTLRange {
            location: start_index,
            length: values.len(),
        };
        unsafe {
            self.set_constant_values(values.as_ptr() as *const std::ffi::c_void, MTLDataType::Float, range);
        }
    }
    
    /// Type-safe wrapper for setting multiple i32 constant values in a range.
    pub fn set_int_constants(&self, values: &[i32], start_index: usize) {
        let range = MTLRange {
            location: start_index,
            length: values.len(),
        };
        unsafe {
            self.set_constant_values(values.as_ptr() as *const std::ffi::c_void, MTLDataType::Int, range);
        }
    }
    
    /// Type-safe wrapper for setting multiple u32 constant values in a range.
    pub fn set_uint_constants(&self, values: &[u32], start_index: usize) {
        let range = MTLRange {
            location: start_index,
            length: values.len(),
        };
        unsafe {
            self.set_constant_values(values.as_ptr() as *const std::ffi::c_void, MTLDataType::UInt, range);
        }
    }
}

impl Default for MTLFunctionConstantValues {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for MTLFunctionConstantValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLFunctionConstantValues {{ .. }}")
    }
}