//! MTLArgument - A Rust wrapper around Metal's MTLArgument class.
//!
//! This module provides safe Rust bindings to the MTLArgument protocol and related classes from Apple's Metal framework.
//! These types are used for shader argument reflection and communication with the GPU.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLDataType, MTLArgumentDescriptor};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create an argument descriptor
//! let descriptor = MTLArgumentDescriptor::new();
//! descriptor.set_data_type(MTLDataType::Texture);
//! descriptor.set_index(0);
//! descriptor.set_access(metal_rs::metal::MTLBindingAccess::ReadOnly);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSUInteger};
use crate::metal::texture::MTLTextureType;

/// MTLDataType - Enumeration of data types for shader arguments.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLDataType {
    None = 0,
    Struct = 1,
    Array = 2,
    Float = 3,
    Float2 = 4,
    Float3 = 5,
    Float4 = 6,
    Float2x2 = 7,
    Float2x3 = 8,
    Float2x4 = 9,
    Float3x2 = 10,
    Float3x3 = 11,
    Float3x4 = 12,
    Float4x2 = 13,
    Float4x3 = 14,
    Float4x4 = 15,
    Half = 16,
    Half2 = 17,
    Half3 = 18,
    Half4 = 19,
    Half2x2 = 20,
    Half2x3 = 21,
    Half2x4 = 22,
    Half3x2 = 23,
    Half3x3 = 24,
    Half3x4 = 25,
    Half4x2 = 26,
    Half4x3 = 27,
    Half4x4 = 28,
    Int = 29,
    Int2 = 30,
    Int3 = 31,
    Int4 = 32,
    UInt = 33,
    UInt2 = 34,
    UInt3 = 35,
    UInt4 = 36,
    Short = 37,
    Short2 = 38,
    Short3 = 39,
    Short4 = 40,
    UShort = 41,
    UShort2 = 42,
    UShort3 = 43,
    UShort4 = 44,
    Char = 45,
    Char2 = 46,
    Char3 = 47,
    Char4 = 48,
    UChar = 49,
    UChar2 = 50,
    UChar3 = 51,
    UChar4 = 52,
    Bool = 53,
    Bool2 = 54,
    Bool3 = 55,
    Bool4 = 56,
    Texture = 58,
    Sampler = 59,
    Pointer = 60,
    R8Unorm = 62,
    R8Snorm = 63,
    R16Unorm = 64,
    R16Snorm = 65,
    RG8Unorm = 66,
    RG8Snorm = 67,
    RG16Unorm = 68,
    RG16Snorm = 69,
    RGBA8Unorm = 70,
    RGBA8Unorm_sRGB = 71,
    RGBA8Snorm = 72,
    RGBA16Unorm = 73,
    RGBA16Snorm = 74,
    RGB10A2Unorm = 75,
    RG11B10Float = 76,
    RGB9E5Float = 77,
    RenderPipeline = 78,
    ComputePipeline = 79,
    IndirectCommandBuffer = 80,
    Long = 81,
    Long2 = 82,
    Long3 = 83,
    Long4 = 84,
    ULong = 85,
    ULong2 = 86,
    ULong3 = 87,
    ULong4 = 88,
    VisibleFunctionTable = 115,
    IntersectionFunctionTable = 116,
    PrimitiveAccelerationStructure = 117,
    InstanceAccelerationStructure = 118,
    BFloat = 121,
    BFloat2 = 122,
    BFloat3 = 123,
    BFloat4 = 124,
}

/// MTLBindingType - Type of binding for Metal shader resources.
#[allow(non_camel_case_types)]
#[repr(i64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLBindingType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,
    ImageblockData = 16,
    Imageblock = 17,
    VisibleFunctionTable = 24,
    PrimitiveAccelerationStructure = 25,
    InstanceAccelerationStructure = 26,
    IntersectionFunctionTable = 27,
    ObjectPayload = 34,
}

/// MTLArgumentType - Type of argument for Metal shaders.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLArgumentType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,
    ImageblockData = 16,
    Imageblock = 17,
    VisibleFunctionTable = 24,
    PrimitiveAccelerationStructure = 25,
    InstanceAccelerationStructure = 26,
    IntersectionFunctionTable = 27,
}

/// MTLBindingAccess - Access mode for resources in Metal shaders.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLBindingAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

/// A reference to an Objective-C `MTLArgumentDescriptor`.
pub struct MTLArgumentDescriptorRef(Object);

/// An owned Objective-C `MTLArgumentDescriptor`.
pub struct MTLArgumentDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLArgumentDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLArgumentDescriptorRef {}
unsafe impl Sync for MTLArgumentDescriptorRef {}

unsafe impl ForeignType for MTLArgumentDescriptor {
    type CType = Object;
    type Ref = MTLArgumentDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLArgumentDescriptor {
        MTLArgumentDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLArgumentDescriptorRef> for MTLArgumentDescriptor {
    fn as_ref(&self) -> &MTLArgumentDescriptorRef {
        unsafe { &*(self.0.cast::<MTLArgumentDescriptorRef>()) }
    }
}

unsafe impl Send for MTLArgumentDescriptor {}
unsafe impl Sync for MTLArgumentDescriptor {}

unsafe impl objc::Message for MTLArgumentDescriptorRef {}

impl MTLArgumentDescriptor {
    /// Creates a new argument descriptor.
    pub fn new() -> Self {
        Self::argument_descriptor()
    }

    /// Creates a new argument descriptor (class method).
    pub fn argument_descriptor() -> Self {
        unsafe {
            let class = class!(MTLArgumentDescriptor);
            let obj: *mut Object = msg_send![class, argumentDescriptor];
            MTLArgumentDescriptor::from_ptr(obj)
        }
    }

    /// Sets the data type of the argument.
    pub fn set_data_type(&self, data_type: MTLDataType) {
        unsafe {
            msg_send![self.as_ref(), setDataType:data_type]
        }
    }

    /// Gets the data type of the argument.
    #[must_use]
    pub fn data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self.as_ref(), dataType]
        }
    }

    /// Sets the index of the argument.
    pub fn set_index(&self, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setIndex:index]
        }
    }

    /// Gets the index of the argument.
    #[must_use]
    pub fn index(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), index]
        }
    }

    /// Sets the array length of the argument.
    pub fn set_array_length(&self, array_length: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setArrayLength:array_length]
        }
    }

    /// Gets the array length of the argument.
    #[must_use]
    pub fn array_length(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), arrayLength]
        }
    }

    /// Sets the access mode of the argument.
    pub fn set_access(&self, access: MTLBindingAccess) {
        unsafe {
            msg_send![self.as_ref(), setAccess:access]
        }
    }

    /// Gets the access mode of the argument.
    #[must_use]
    pub fn access(&self) -> MTLBindingAccess {
        unsafe {
            msg_send![self.as_ref(), access]
        }
    }

    /// Sets the texture type of the argument.
    pub fn set_texture_type(&self, texture_type: MTLTextureType) {
        unsafe {
            msg_send![self.as_ref(), setTextureType:texture_type]
        }
    }

    /// Gets the texture type of the argument.
    #[must_use]
    pub fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self.as_ref(), textureType]
        }
    }

    /// Sets the constant block alignment of the argument.
    pub fn set_constant_block_alignment(&self, alignment: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), setConstantBlockAlignment:alignment]
        }
    }

    /// Gets the constant block alignment of the argument.
    #[must_use]
    pub fn constant_block_alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), constantBlockAlignment]
        }
    }
}

impl Clone for MTLArgumentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLArgumentDescriptor::from_ptr(obj)
        }
    }
}

impl fmt::Debug for MTLArgumentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLArgumentDescriptor")
            .field("data_type", &self.data_type())
            .field("index", &self.index())
            .field("array_length", &self.array_length())
            .field("access", &self.access())
            .field("texture_type", &self.texture_type())
            .field("constant_block_alignment", &self.constant_block_alignment())
            .finish()
    }
}

impl Drop for MTLArgumentDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Default for MTLArgumentDescriptor {
    fn default() -> Self {
        Self::new()
    }
}