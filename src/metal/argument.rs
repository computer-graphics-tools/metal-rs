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

/// A reference to an Objective-C `MTLArgument`.
pub struct MTLArgumentRef(Object);

/// An owned Objective-C `MTLArgument`.
pub struct MTLArgument(*mut Object);

unsafe impl ForeignTypeRef for MTLArgumentRef {
    type CType = Object;
}

unsafe impl Send for MTLArgumentRef {}
unsafe impl Sync for MTLArgumentRef {}

unsafe impl ForeignType for MTLArgument {
    type CType = Object;
    type Ref = MTLArgumentRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLArgument {
        MTLArgument(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLArgumentRef> for MTLArgument {
    fn as_ref(&self) -> &MTLArgumentRef {
        unsafe { &*(self.0.cast::<MTLArgumentRef>()) }
    }
}

unsafe impl Send for MTLArgument {}
unsafe impl Sync for MTLArgument {}

/// A reference to an Objective-C `MTLType`.
pub struct MTLTypeRef(Object);

/// An owned Objective-C `MTLType`.
pub struct MTLType(*mut Object);

unsafe impl ForeignTypeRef for MTLTypeRef {
    type CType = Object;
}

unsafe impl Send for MTLTypeRef {}
unsafe impl Sync for MTLTypeRef {}

unsafe impl ForeignType for MTLType {
    type CType = Object;
    type Ref = MTLTypeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLType {
        MTLType(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLTypeRef> for MTLType {
    fn as_ref(&self) -> &MTLTypeRef {
        unsafe { &*(self.0.cast::<MTLTypeRef>()) }
    }
}

unsafe impl Send for MTLType {}
unsafe impl Sync for MTLType {}

/// A reference to an Objective-C `MTLStructMember`.
pub struct MTLStructMemberRef(Object);

/// An owned Objective-C `MTLStructMember`.
pub struct MTLStructMember(*mut Object);

unsafe impl ForeignTypeRef for MTLStructMemberRef {
    type CType = Object;
}

unsafe impl Send for MTLStructMemberRef {}
unsafe impl Sync for MTLStructMemberRef {}

unsafe impl ForeignType for MTLStructMember {
    type CType = Object;
    type Ref = MTLStructMemberRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLStructMember {
        MTLStructMember(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLStructMemberRef> for MTLStructMember {
    fn as_ref(&self) -> &MTLStructMemberRef {
        unsafe { &*(self.0.cast::<MTLStructMemberRef>()) }
    }
}

unsafe impl Send for MTLStructMember {}
unsafe impl Sync for MTLStructMember {}

/// A reference to an Objective-C `MTLStructType`.
pub struct MTLStructTypeRef(Object);

/// An owned Objective-C `MTLStructType`.
pub struct MTLStructType(*mut Object);

unsafe impl ForeignTypeRef for MTLStructTypeRef {
    type CType = Object;
}

unsafe impl Send for MTLStructTypeRef {}
unsafe impl Sync for MTLStructTypeRef {}

unsafe impl ForeignType for MTLStructType {
    type CType = Object;
    type Ref = MTLStructTypeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLStructType {
        MTLStructType(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLStructTypeRef> for MTLStructType {
    fn as_ref(&self) -> &MTLStructTypeRef {
        unsafe { &*(self.0.cast::<MTLStructTypeRef>()) }
    }
}

unsafe impl Send for MTLStructType {}
unsafe impl Sync for MTLStructType {}

/// A reference to an Objective-C `MTLArrayType`.
pub struct MTLArrayTypeRef(Object);

/// An owned Objective-C `MTLArrayType`.
pub struct MTLArrayType(*mut Object);

unsafe impl ForeignTypeRef for MTLArrayTypeRef {
    type CType = Object;
}

unsafe impl Send for MTLArrayTypeRef {}
unsafe impl Sync for MTLArrayTypeRef {}

unsafe impl ForeignType for MTLArrayType {
    type CType = Object;
    type Ref = MTLArrayTypeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLArrayType {
        MTLArrayType(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLArrayTypeRef> for MTLArrayType {
    fn as_ref(&self) -> &MTLArrayTypeRef {
        unsafe { &*(self.0.cast::<MTLArrayTypeRef>()) }
    }
}

unsafe impl Send for MTLArrayType {}
unsafe impl Sync for MTLArrayType {}

/// A reference to an Objective-C `MTLPointerType`.
pub struct MTLPointerTypeRef(Object);

/// An owned Objective-C `MTLPointerType`.
pub struct MTLPointerType(*mut Object);

unsafe impl ForeignTypeRef for MTLPointerTypeRef {
    type CType = Object;
}

unsafe impl Send for MTLPointerTypeRef {}
unsafe impl Sync for MTLPointerTypeRef {}

unsafe impl ForeignType for MTLPointerType {
    type CType = Object;
    type Ref = MTLPointerTypeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLPointerType {
        MTLPointerType(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLPointerTypeRef> for MTLPointerType {
    fn as_ref(&self) -> &MTLPointerTypeRef {
        unsafe { &*(self.0.cast::<MTLPointerTypeRef>()) }
    }
}

unsafe impl Send for MTLPointerType {}
unsafe impl Sync for MTLPointerType {}

/// A reference to an Objective-C `MTLTextureReferenceType`.
pub struct MTLTextureReferenceTypeRef(Object);

/// An owned Objective-C `MTLTextureReferenceType`.
pub struct MTLTextureReferenceType(*mut Object);

unsafe impl ForeignTypeRef for MTLTextureReferenceTypeRef {
    type CType = Object;
}

unsafe impl Send for MTLTextureReferenceTypeRef {}
unsafe impl Sync for MTLTextureReferenceTypeRef {}

unsafe impl ForeignType for MTLTextureReferenceType {
    type CType = Object;
    type Ref = MTLTextureReferenceTypeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLTextureReferenceType {
        MTLTextureReferenceType(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLTextureReferenceTypeRef> for MTLTextureReferenceType {
    fn as_ref(&self) -> &MTLTextureReferenceTypeRef {
        unsafe { &*(self.0.cast::<MTLTextureReferenceTypeRef>()) }
    }
}

unsafe impl Send for MTLTextureReferenceType {}
unsafe impl Sync for MTLTextureReferenceType {}

/// A reference to an Objective-C `MTLBinding`.
pub struct MTLBindingRef(Object);

/// An owned Objective-C `MTLBinding`.
pub struct MTLBinding(*mut Object);

unsafe impl ForeignTypeRef for MTLBindingRef {
    type CType = Object;
}

unsafe impl Send for MTLBindingRef {}
unsafe impl Sync for MTLBindingRef {}

unsafe impl ForeignType for MTLBinding {
    type CType = Object;
    type Ref = MTLBindingRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBinding {
        MTLBinding(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBindingRef> for MTLBinding {
    fn as_ref(&self) -> &MTLBindingRef {
        unsafe { &*(self.0.cast::<MTLBindingRef>()) }
    }
}

unsafe impl Send for MTLBinding {}
unsafe impl Sync for MTLBinding {}

/// A reference to an Objective-C `MTLBufferBinding`.
pub struct MTLBufferBindingRef(Object);

/// An owned Objective-C `MTLBufferBinding`.
pub struct MTLBufferBinding(*mut Object);

unsafe impl ForeignTypeRef for MTLBufferBindingRef {
    type CType = Object;
}

unsafe impl Send for MTLBufferBindingRef {}
unsafe impl Sync for MTLBufferBindingRef {}

unsafe impl ForeignType for MTLBufferBinding {
    type CType = Object;
    type Ref = MTLBufferBindingRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBufferBinding {
        MTLBufferBinding(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBufferBindingRef> for MTLBufferBinding {
    fn as_ref(&self) -> &MTLBufferBindingRef {
        unsafe { &*(self.0.cast::<MTLBufferBindingRef>()) }
    }
}

unsafe impl Send for MTLBufferBinding {}
unsafe impl Sync for MTLBufferBinding {}

/// A reference to an Objective-C `MTLThreadgroupBinding`.
pub struct MTLThreadgroupBindingRef(Object);

/// An owned Objective-C `MTLThreadgroupBinding`.
pub struct MTLThreadgroupBinding(*mut Object);

unsafe impl ForeignTypeRef for MTLThreadgroupBindingRef {
    type CType = Object;
}

unsafe impl Send for MTLThreadgroupBindingRef {}
unsafe impl Sync for MTLThreadgroupBindingRef {}

unsafe impl ForeignType for MTLThreadgroupBinding {
    type CType = Object;
    type Ref = MTLThreadgroupBindingRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLThreadgroupBinding {
        MTLThreadgroupBinding(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLThreadgroupBindingRef> for MTLThreadgroupBinding {
    fn as_ref(&self) -> &MTLThreadgroupBindingRef {
        unsafe { &*(self.0.cast::<MTLThreadgroupBindingRef>()) }
    }
}

unsafe impl Send for MTLThreadgroupBinding {}
unsafe impl Sync for MTLThreadgroupBinding {}

/// A reference to an Objective-C `MTLTextureBinding`.
pub struct MTLTextureBindingRef(Object);

/// An owned Objective-C `MTLTextureBinding`.
pub struct MTLTextureBinding(*mut Object);

unsafe impl ForeignTypeRef for MTLTextureBindingRef {
    type CType = Object;
}

unsafe impl Send for MTLTextureBindingRef {}
unsafe impl Sync for MTLTextureBindingRef {}

unsafe impl ForeignType for MTLTextureBinding {
    type CType = Object;
    type Ref = MTLTextureBindingRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLTextureBinding {
        MTLTextureBinding(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLTextureBindingRef> for MTLTextureBinding {
    fn as_ref(&self) -> &MTLTextureBindingRef {
        unsafe { &*(self.0.cast::<MTLTextureBindingRef>()) }
    }
}

unsafe impl Send for MTLTextureBinding {}
unsafe impl Sync for MTLTextureBinding {}

/// A reference to an Objective-C `MTLObjectPayloadBinding`.
pub struct MTLObjectPayloadBindingRef(Object);

/// An owned Objective-C `MTLObjectPayloadBinding`.
pub struct MTLObjectPayloadBinding(*mut Object);

unsafe impl ForeignTypeRef for MTLObjectPayloadBindingRef {
    type CType = Object;
}

unsafe impl Send for MTLObjectPayloadBindingRef {}
unsafe impl Sync for MTLObjectPayloadBindingRef {}

unsafe impl ForeignType for MTLObjectPayloadBinding {
    type CType = Object;
    type Ref = MTLObjectPayloadBindingRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLObjectPayloadBinding {
        MTLObjectPayloadBinding(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLObjectPayloadBindingRef> for MTLObjectPayloadBinding {
    fn as_ref(&self) -> &MTLObjectPayloadBindingRef {
        unsafe { &*(self.0.cast::<MTLObjectPayloadBindingRef>()) }
    }
}

unsafe impl Send for MTLObjectPayloadBinding {}
unsafe impl Sync for MTLObjectPayloadBinding {}

unsafe impl objc::Message for MTLTypeRef {}
unsafe impl objc::Message for MTLStructMemberRef {}
unsafe impl objc::Message for MTLStructTypeRef {}
unsafe impl objc::Message for MTLArrayTypeRef {}
unsafe impl objc::Message for MTLPointerTypeRef {}
unsafe impl objc::Message for MTLTextureReferenceTypeRef {}
unsafe impl objc::Message for MTLArgumentRef {}
unsafe impl objc::Message for MTLBindingRef {}
unsafe impl objc::Message for MTLBufferBindingRef {}
unsafe impl objc::Message for MTLThreadgroupBindingRef {}
unsafe impl objc::Message for MTLTextureBindingRef {}
unsafe impl objc::Message for MTLObjectPayloadBindingRef {}
unsafe impl objc::Message for MTLArgumentDescriptorRef {}

impl Drop for MTLType {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLType {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLType::from_ptr(obj)
        }
    }
}

impl Drop for MTLStructMember {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLStructMember {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLStructMember::from_ptr(obj)
        }
    }
}

impl Drop for MTLStructType {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLStructType {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLStructType::from_ptr(obj)
        }
    }
}

impl Drop for MTLArrayType {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLArrayType {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLArrayType::from_ptr(obj)
        }
    }
}

impl Drop for MTLPointerType {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLPointerType {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLPointerType::from_ptr(obj)
        }
    }
}

impl Drop for MTLTextureReferenceType {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLTextureReferenceType {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLTextureReferenceType::from_ptr(obj)
        }
    }
}

impl Drop for MTLArgument {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLArgument {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLArgument::from_ptr(obj)
        }
    }
}

impl Drop for MTLBinding {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLBinding {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLBinding::from_ptr(obj)
        }
    }
}

impl Drop for MTLBufferBinding {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLBufferBinding {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLBufferBinding::from_ptr(obj)
        }
    }
}

impl Drop for MTLThreadgroupBinding {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLThreadgroupBinding {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLThreadgroupBinding::from_ptr(obj)
        }
    }
}

impl Drop for MTLTextureBinding {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLTextureBinding {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLTextureBinding::from_ptr(obj)
        }
    }
}

impl Drop for MTLObjectPayloadBinding {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLObjectPayloadBinding {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLObjectPayloadBinding::from_ptr(obj)
        }
    }
}

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

impl MTLTypeRef {
    /// Gets the data type of this type.
    #[must_use]
    pub fn data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, dataType]
        }
    }
}

impl fmt::Debug for MTLType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLType")
            .field("data_type", &self.as_ref().data_type())
            .finish()
    }
}

impl MTLStructMemberRef {
    /// Gets the name of this struct member.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let name: *mut Object = msg_send![self, name];
            let string = crate::foundation::NSString::from_ptr(name);
            string.as_str().to_string()
        }
    }
    
    /// Gets the offset of this struct member in bytes.
    #[must_use]
    pub fn offset(&self) -> NSUInteger {
        unsafe {
            msg_send![self, offset]
        }
    }
    
    /// Gets the data type of this struct member.
    #[must_use]
    pub fn data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, dataType]
        }
    }
    
    /// Gets the struct type of this member, if it's a struct.
    #[must_use]
    pub fn struct_type(&self) -> Option<MTLStructType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, structType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStructType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the array type of this member, if it's an array.
    #[must_use]
    pub fn array_type(&self) -> Option<MTLArrayType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, arrayType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLArrayType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the texture reference type of this member, if it's a texture reference.
    #[must_use]
    pub fn texture_reference_type(&self) -> Option<MTLTextureReferenceType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, textureReferenceType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLTextureReferenceType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the pointer type of this member, if it's a pointer.
    #[must_use]
    pub fn pointer_type(&self) -> Option<MTLPointerType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, pointerType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLPointerType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the argument index of this struct member.
    #[must_use]
    pub fn argument_index(&self) -> NSUInteger {
        unsafe {
            msg_send![self, argumentIndex]
        }
    }
}

impl fmt::Debug for MTLStructMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLStructMember")
            .field("name", &self.as_ref().name())
            .field("offset", &self.as_ref().offset())
            .field("data_type", &self.as_ref().data_type())
            .field("argument_index", &self.as_ref().argument_index())
            .finish()
    }
}

impl MTLStructTypeRef {
    /// Gets the array of struct members in this struct type.
    #[must_use]
    pub fn members(&self) -> Vec<MTLStructMember> {
        unsafe {
            let array: *mut Object = msg_send![self, members];
            let count: NSUInteger = msg_send![array, count];
            let mut members = Vec::with_capacity(count);
            
            for i in 0..count {
                let member: *mut Object = msg_send![array, objectAtIndex:i];
                members.push(MTLStructMember::from_ptr(member));
            }
            
            members
        }
    }
    
    /// Gets a struct member by name.
    #[must_use]
    pub fn member_by_name(&self, name: &str) -> Option<MTLStructMember> {
        unsafe {
            let ns_string = crate::foundation::NSString::from_rust_str(name);
            let ptr: *mut Object = msg_send![self, memberByName:ns_string.as_ptr()];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStructMember::from_ptr(ptr))
            }
        }
    }
}

impl fmt::Debug for MTLStructType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLStructType")
            .field("data_type", &self.as_ref().data_type())
            .field("members", &self.as_ref().members())
            .finish()
    }
}

impl MTLArrayTypeRef {
    /// Gets the element type of this array.
    #[must_use]
    pub fn element_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, elementType]
        }
    }
    
    /// Gets the array length.
    #[must_use]
    pub fn array_length(&self) -> NSUInteger {
        unsafe {
            msg_send![self, arrayLength]
        }
    }
    
    /// Gets the stride in bytes between elements.
    #[must_use]
    pub fn stride(&self) -> NSUInteger {
        unsafe {
            msg_send![self, stride]
        }
    }
    
    /// Gets the stride in argument indices between elements.
    #[must_use]
    pub fn argument_index_stride(&self) -> NSUInteger {
        unsafe {
            msg_send![self, argumentIndexStride]
        }
    }
    
    /// Gets the struct type of the elements, if they are structs.
    #[must_use]
    pub fn element_struct_type(&self) -> Option<MTLStructType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, elementStructType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStructType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the array type of the elements, if they are arrays.
    #[must_use]
    pub fn element_array_type(&self) -> Option<MTLArrayType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, elementArrayType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLArrayType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the texture reference type of the elements, if they are texture references.
    #[must_use]
    pub fn element_texture_reference_type(&self) -> Option<MTLTextureReferenceType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, elementTextureReferenceType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLTextureReferenceType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the pointer type of the elements, if they are pointers.
    #[must_use]
    pub fn element_pointer_type(&self) -> Option<MTLPointerType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, elementPointerType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLPointerType::from_ptr(ptr))
            }
        }
    }
}

impl fmt::Debug for MTLArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLArrayType")
            .field("data_type", &self.as_ref().data_type())
            .field("element_type", &self.as_ref().element_type())
            .field("array_length", &self.as_ref().array_length())
            .field("stride", &self.as_ref().stride())
            .field("argument_index_stride", &self.as_ref().argument_index_stride())
            .finish()
    }
}

impl MTLPointerTypeRef {
    /// Gets the element type of this pointer.
    #[must_use]
    pub fn element_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, elementType]
        }
    }
    
    /// Gets the access mode of this pointer.
    #[must_use]
    pub fn access(&self) -> MTLBindingAccess {
        unsafe {
            msg_send![self, access]
        }
    }
    
    /// Gets the alignment of this pointer in bytes.
    #[must_use]
    pub fn alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self, alignment]
        }
    }
    
    /// Gets the data size of this pointer in bytes.
    #[must_use]
    pub fn data_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, dataSize]
        }
    }
    
    /// Returns whether the element is an argument buffer.
    #[must_use]
    pub fn element_is_argument_buffer(&self) -> bool {
        unsafe {
            msg_send![self, elementIsArgumentBuffer]
        }
    }
    
    /// Gets the struct type of the elements, if they are structs.
    #[must_use]
    pub fn element_struct_type(&self) -> Option<MTLStructType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, elementStructType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStructType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the array type of the elements, if they are arrays.
    #[must_use]
    pub fn element_array_type(&self) -> Option<MTLArrayType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, elementArrayType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLArrayType::from_ptr(ptr))
            }
        }
    }
}

impl fmt::Debug for MTLPointerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLPointerType")
            .field("data_type", &self.as_ref().data_type())
            .field("element_type", &self.as_ref().element_type())
            .field("access", &self.as_ref().access())
            .field("alignment", &self.as_ref().alignment())
            .field("data_size", &self.as_ref().data_size())
            .field("element_is_argument_buffer", &self.as_ref().element_is_argument_buffer())
            .finish()
    }
}

impl MTLTextureReferenceTypeRef {
    /// Gets the data type of this texture reference.
    #[must_use]
    pub fn texture_data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, textureDataType]
        }
    }
    
    /// Gets the texture type of this texture reference.
    #[must_use]
    pub fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self, textureType]
        }
    }
    
    /// Gets the access mode of this texture reference.
    #[must_use]
    pub fn access(&self) -> MTLBindingAccess {
        unsafe {
            msg_send![self, access]
        }
    }
    
    /// Returns whether this texture reference is a depth texture.
    #[must_use]
    pub fn is_depth_texture(&self) -> bool {
        unsafe {
            msg_send![self, isDepthTexture]
        }
    }
}

impl fmt::Debug for MTLTextureReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLTextureReferenceType")
            .field("data_type", &self.as_ref().data_type())
            .field("texture_data_type", &self.as_ref().texture_data_type())
            .field("texture_type", &self.as_ref().texture_type())
            .field("access", &self.as_ref().access())
            .field("is_depth_texture", &self.as_ref().is_depth_texture())
            .finish()
    }
}

impl MTLArgumentRef {
    /// Gets the name of this argument.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let name: *mut Object = msg_send![self, name];
            let string = crate::foundation::NSString::from_ptr(name);
            string.as_str().to_string()
        }
    }
    
    /// Gets the type of this argument.
    #[must_use]
    pub fn type_(&self) -> MTLArgumentType {
        unsafe {
            msg_send![self, type]
        }
    }
    
    /// Gets the access mode of this argument.
    #[must_use]
    pub fn access(&self) -> MTLBindingAccess {
        unsafe {
            msg_send![self, access]
        }
    }
    
    /// Gets the index of this argument.
    #[must_use]
    pub fn index(&self) -> NSUInteger {
        unsafe {
            msg_send![self, index]
        }
    }
    
    /// Returns whether this argument is active in the shader.
    #[must_use]
    pub fn active(&self) -> bool {
        unsafe {
            msg_send![self, active]
        }
    }
    
    /// Gets the alignment of this buffer argument in bytes.
    #[must_use]
    pub fn buffer_alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self, bufferAlignment]
        }
    }
    
    /// Gets the data size of this buffer argument in bytes.
    #[must_use]
    pub fn buffer_data_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, bufferDataSize]
        }
    }
    
    /// Gets the data type of this buffer argument.
    #[must_use]
    pub fn buffer_data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, bufferDataType]
        }
    }
    
    /// Gets the struct type of this buffer argument, if it's a struct.
    #[must_use]
    pub fn buffer_struct_type(&self) -> Option<MTLStructType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, bufferStructType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStructType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the pointer type of this buffer argument, if it's a pointer.
    #[must_use]
    pub fn buffer_pointer_type(&self) -> Option<MTLPointerType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, bufferPointerType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLPointerType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the alignment of this threadgroup memory argument in bytes.
    #[must_use]
    pub fn threadgroup_memory_alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self, threadgroupMemoryAlignment]
        }
    }
    
    /// Gets the data size of this threadgroup memory argument in bytes.
    #[must_use]
    pub fn threadgroup_memory_data_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, threadgroupMemoryDataSize]
        }
    }
    
    /// Gets the texture type of this texture argument.
    #[must_use]
    pub fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self, textureType]
        }
    }
    
    /// Gets the data type of this texture argument.
    #[must_use]
    pub fn texture_data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, textureDataType]
        }
    }
    
    /// Returns whether this texture argument is a depth texture.
    #[must_use]
    pub fn is_depth_texture(&self) -> bool {
        unsafe {
            msg_send![self, isDepthTexture]
        }
    }
    
    /// Gets the array length of this argument.
    #[must_use]
    pub fn array_length(&self) -> NSUInteger {
        unsafe {
            msg_send![self, arrayLength]
        }
    }
}

impl fmt::Debug for MTLArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLArgument")
            .field("name", &self.as_ref().name())
            .field("type", &self.as_ref().type_())
            .field("access", &self.as_ref().access())
            .field("index", &self.as_ref().index())
            .field("active", &self.as_ref().active())
            .finish()
    }
}

impl MTLBindingRef {
    /// Gets the name of this binding.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let name: *mut Object = msg_send![self, name];
            let string = crate::foundation::NSString::from_ptr(name);
            string.as_str().to_string()
        }
    }
    
    /// Gets the type of this binding.
    #[must_use]
    pub fn type_(&self) -> MTLBindingType {
        unsafe {
            msg_send![self, type]
        }
    }
    
    /// Gets the access mode of this binding.
    #[must_use]
    pub fn access(&self) -> MTLBindingAccess {
        unsafe {
            msg_send![self, access]
        }
    }
    
    /// Gets the index of this binding.
    #[must_use]
    pub fn index(&self) -> NSUInteger {
        unsafe {
            msg_send![self, index]
        }
    }
    
    /// Returns whether this binding is used in the shader.
    #[must_use]
    pub fn used(&self) -> bool {
        unsafe {
            msg_send![self, used]
        }
    }
    
    /// Returns whether this binding is an argument.
    #[must_use]
    pub fn argument(&self) -> bool {
        unsafe {
            msg_send![self, argument]
        }
    }
}

impl fmt::Debug for MTLBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLBinding")
            .field("name", &self.as_ref().name())
            .field("type", &self.as_ref().type_())
            .field("access", &self.as_ref().access())
            .field("index", &self.as_ref().index())
            .field("used", &self.as_ref().used())
            .field("argument", &self.as_ref().argument())
            .finish()
    }
}

impl MTLBufferBindingRef {
    /// Gets the alignment of this buffer binding in bytes.
    #[must_use]
    pub fn buffer_alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self, bufferAlignment]
        }
    }
    
    /// Gets the data size of this buffer binding in bytes.
    #[must_use]
    pub fn buffer_data_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, bufferDataSize]
        }
    }
    
    /// Gets the data type of this buffer binding.
    #[must_use]
    pub fn buffer_data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, bufferDataType]
        }
    }
    
    /// Gets the struct type of this buffer binding, if it's a struct.
    #[must_use]
    pub fn buffer_struct_type(&self) -> Option<MTLStructType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, bufferStructType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStructType::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the pointer type of this buffer binding, if it's a pointer.
    #[must_use]
    pub fn buffer_pointer_type(&self) -> Option<MTLPointerType> {
        unsafe {
            let ptr: *mut Object = msg_send![self, bufferPointerType];
            if ptr.is_null() {
                None
            } else {
                Some(MTLPointerType::from_ptr(ptr))
            }
        }
    }
}

impl fmt::Debug for MTLBufferBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLBufferBinding")
            .field("name", &self.as_ref().name())
            .field("type", &self.as_ref().type_())
            .field("access", &self.as_ref().access())
            .field("index", &self.as_ref().index())
            .field("used", &self.as_ref().used())
            .field("argument", &self.as_ref().argument())
            .field("buffer_alignment", &self.as_ref().buffer_alignment())
            .field("buffer_data_size", &self.as_ref().buffer_data_size())
            .field("buffer_data_type", &self.as_ref().buffer_data_type())
            .finish()
    }
}

impl MTLThreadgroupBindingRef {
    /// Gets the alignment of this threadgroup memory binding in bytes.
    #[must_use]
    pub fn threadgroup_memory_alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self, threadgroupMemoryAlignment]
        }
    }
    
    /// Gets the data size of this threadgroup memory binding in bytes.
    #[must_use]
    pub fn threadgroup_memory_data_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, threadgroupMemoryDataSize]
        }
    }
}

impl fmt::Debug for MTLThreadgroupBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLThreadgroupBinding")
            .field("name", &self.as_ref().name())
            .field("type", &self.as_ref().type_())
            .field("access", &self.as_ref().access())
            .field("index", &self.as_ref().index())
            .field("used", &self.as_ref().used())
            .field("argument", &self.as_ref().argument())
            .field("threadgroup_memory_alignment", &self.as_ref().threadgroup_memory_alignment())
            .field("threadgroup_memory_data_size", &self.as_ref().threadgroup_memory_data_size())
            .finish()
    }
}

impl MTLTextureBindingRef {
    /// Gets the texture type of this texture binding.
    #[must_use]
    pub fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self, textureType]
        }
    }
    
    /// Gets the data type of this texture binding.
    #[must_use]
    pub fn texture_data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, textureDataType]
        }
    }
    
    /// Returns whether this texture binding is a depth texture.
    #[must_use]
    pub fn depth_texture(&self) -> bool {
        unsafe {
            msg_send![self, depthTexture]
        }
    }
    
    /// Gets the array length of this texture binding.
    #[must_use]
    pub fn array_length(&self) -> NSUInteger {
        unsafe {
            msg_send![self, arrayLength]
        }
    }
}

impl fmt::Debug for MTLTextureBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLTextureBinding")
            .field("name", &self.as_ref().name())
            .field("type", &self.as_ref().type_())
            .field("access", &self.as_ref().access())
            .field("index", &self.as_ref().index())
            .field("used", &self.as_ref().used())
            .field("argument", &self.as_ref().argument())
            .field("texture_type", &self.as_ref().texture_type())
            .field("texture_data_type", &self.as_ref().texture_data_type())
            .field("depth_texture", &self.as_ref().depth_texture())
            .field("array_length", &self.as_ref().array_length())
            .finish()
    }
}

impl MTLObjectPayloadBindingRef {
    /// Gets the alignment of this object payload binding in bytes.
    #[must_use]
    pub fn object_payload_alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self, objectPayloadAlignment]
        }
    }
    
    /// Gets the data size of this object payload binding in bytes.
    #[must_use]
    pub fn object_payload_data_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, objectPayloadDataSize]
        }
    }
}

impl fmt::Debug for MTLObjectPayloadBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLObjectPayloadBinding")
            .field("name", &self.as_ref().name())
            .field("type", &self.as_ref().type_())
            .field("access", &self.as_ref().access())
            .field("index", &self.as_ref().index())
            .field("used", &self.as_ref().used())
            .field("argument", &self.as_ref().argument())
            .field("object_payload_alignment", &self.as_ref().object_payload_alignment())
            .field("object_payload_data_size", &self.as_ref().object_payload_data_size())
            .finish()
    }
}