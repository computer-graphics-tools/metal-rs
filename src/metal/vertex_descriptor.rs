// Copyright 2024 The metal-rs contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSUInteger;
use std::fmt;

pub const BUFFER_LAYOUT_STRIDE_DYNAMIC: NSUInteger = NSUInteger::MAX;

/// Specifies the data formats for vertex attribute data.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLVertexFormat {
    Invalid = 0,
    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,
    Char2 = 4,
    Char3 = 5,
    Char4 = 6,
    UChar2Normalized = 7,
    UChar3Normalized = 8,
    UChar4Normalized = 9,
    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,
    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,
    Short2 = 16,
    Short3 = 17,
    Short4 = 18,
    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,
    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,
    Half2 = 25,
    Half3 = 26,
    Half4 = 27,
    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,
    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,
    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,
    Int1010102Normalized = 40,
    UInt1010102Normalized = 41,
    UChar4NormalizedBGRA = 42,
    UChar = 45,
    Char = 46,
    UCharNormalized = 47,
    CharNormalized = 48,
    UShort = 49,
    Short = 50,
    UShortNormalized = 51,
    ShortNormalized = 52,
    Half = 53,
    FloatRG11B10 = 54,
    FloatRGB9E5 = 55,
}

/// Specifies how often vertex attribute data is advanced during vertex fetch.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLVertexStepFunction {
    /// The same data is used for every vertex.
    Constant = 0,
    /// Data is advanced once per vertex.
    PerVertex = 1,
    /// Data is advanced once per instance.
    PerInstance = 2,
    /// Data is advanced once per patch.
    PerPatch = 3,
    /// Data is advanced once per patch control point.
    PerPatchControlPoint = 4,
}

/// A reference to an Objective-C `MTLVertexBufferLayoutDescriptor`.
pub struct MTLVertexBufferLayoutDescriptorRef(Object);

/// An owned Objective-C `MTLVertexBufferLayoutDescriptor`.
pub struct MTLVertexBufferLayoutDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLVertexBufferLayoutDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLVertexBufferLayoutDescriptorRef {}
unsafe impl Sync for MTLVertexBufferLayoutDescriptorRef {}

unsafe impl ForeignType for MTLVertexBufferLayoutDescriptor {
    type CType = Object;
    type Ref = MTLVertexBufferLayoutDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLVertexBufferLayoutDescriptor {
        MTLVertexBufferLayoutDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLVertexBufferLayoutDescriptorRef> for MTLVertexBufferLayoutDescriptor {
    fn as_ref(&self) -> &MTLVertexBufferLayoutDescriptorRef {
        unsafe { &*(self.0.cast::<MTLVertexBufferLayoutDescriptorRef>()) }
    }
}

unsafe impl Send for MTLVertexBufferLayoutDescriptor {}
unsafe impl Sync for MTLVertexBufferLayoutDescriptor {}

unsafe impl objc::Message for MTLVertexBufferLayoutDescriptorRef {}

impl MTLVertexBufferLayoutDescriptor {
    /// Creates a new vertex buffer layout descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLVertexBufferLayoutDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLVertexBufferLayoutDescriptor::from_ptr(obj)
        }
    }
}

impl MTLVertexBufferLayoutDescriptorRef {
    /// Gets the stride of this buffer layout.
    #[must_use]
    pub fn stride(&self) -> NSUInteger {
        unsafe { msg_send![self, stride] }
    }

    /// Sets the stride of this buffer layout.
    pub fn set_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setStride: stride] }
    }

    /// Gets the step function of this buffer layout.
    #[must_use]
    pub fn step_function(&self) -> MTLVertexStepFunction {
        unsafe { msg_send![self, stepFunction] }
    }

    /// Sets the step function of this buffer layout.
    pub fn set_step_function(&self, step_function: MTLVertexStepFunction) {
        unsafe { msg_send![self, setStepFunction: step_function] }
    }

    /// Gets the step rate of this buffer layout.
    #[must_use]
    pub fn step_rate(&self) -> NSUInteger {
        unsafe { msg_send![self, stepRate] }
    }

    /// Sets the step rate of this buffer layout.
    pub fn set_step_rate(&self, step_rate: NSUInteger) {
        unsafe { msg_send![self, setStepRate: step_rate] }
    }
}

impl Clone for MTLVertexBufferLayoutDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLVertexBufferLayoutDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLVertexBufferLayoutDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLVertexBufferLayoutDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLVertexBufferLayoutDescriptor")
            .field("stride", &self.as_ref().stride())
            .field("step_function", &self.as_ref().step_function())
            .field("step_rate", &self.as_ref().step_rate())
            .finish()
    }
}

/// A reference to an Objective-C `MTLVertexBufferLayoutDescriptorArray`.
pub struct MTLVertexBufferLayoutDescriptorArrayRef(Object);

/// An owned Objective-C `MTLVertexBufferLayoutDescriptorArray`.
pub struct MTLVertexBufferLayoutDescriptorArray(*mut Object);

unsafe impl ForeignTypeRef for MTLVertexBufferLayoutDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLVertexBufferLayoutDescriptorArrayRef {}
unsafe impl Sync for MTLVertexBufferLayoutDescriptorArrayRef {}

unsafe impl ForeignType for MTLVertexBufferLayoutDescriptorArray {
    type CType = Object;
    type Ref = MTLVertexBufferLayoutDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLVertexBufferLayoutDescriptorArray {
        MTLVertexBufferLayoutDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLVertexBufferLayoutDescriptorArrayRef> for MTLVertexBufferLayoutDescriptorArray {
    fn as_ref(&self) -> &MTLVertexBufferLayoutDescriptorArrayRef {
        unsafe { &*(self.0.cast::<MTLVertexBufferLayoutDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLVertexBufferLayoutDescriptorArray {}
unsafe impl Sync for MTLVertexBufferLayoutDescriptorArray {}

unsafe impl objc::Message for MTLVertexBufferLayoutDescriptorArrayRef {}

impl Clone for MTLVertexBufferLayoutDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLVertexBufferLayoutDescriptorArray::from_ptr(obj)
        }
    }
}

impl Drop for MTLVertexBufferLayoutDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl MTLVertexBufferLayoutDescriptorArrayRef {
    /// Gets the descriptor at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> Option<MTLVertexBufferLayoutDescriptor> {
        unsafe {
            let obj: *mut Object = msg_send![self, objectAtIndexedSubscript: index];
            if obj.is_null() {
                None
            } else {
                Some(MTLVertexBufferLayoutDescriptor::from_ptr(obj))
            }
        }
    }

    /// Sets the descriptor at the specified index.
    pub fn set_object(&self, descriptor: Option<&MTLVertexBufferLayoutDescriptor>, index: NSUInteger) {
        unsafe {
            let ptr = match descriptor {
                Some(descriptor) => descriptor.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self, setObject:ptr atIndexedSubscript:index];
        }
    }
}

/// A reference to an Objective-C `MTLVertexAttributeDescriptor`.
pub struct MTLVertexAttributeDescriptorRef(Object);

/// An owned Objective-C `MTLVertexAttributeDescriptor`.
pub struct MTLVertexAttributeDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLVertexAttributeDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLVertexAttributeDescriptorRef {}
unsafe impl Sync for MTLVertexAttributeDescriptorRef {}

unsafe impl ForeignType for MTLVertexAttributeDescriptor {
    type CType = Object;
    type Ref = MTLVertexAttributeDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLVertexAttributeDescriptor {
        MTLVertexAttributeDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLVertexAttributeDescriptorRef> for MTLVertexAttributeDescriptor {
    fn as_ref(&self) -> &MTLVertexAttributeDescriptorRef {
        unsafe { &*(self.0.cast::<MTLVertexAttributeDescriptorRef>()) }
    }
}

unsafe impl Send for MTLVertexAttributeDescriptor {}
unsafe impl Sync for MTLVertexAttributeDescriptor {}

unsafe impl objc::Message for MTLVertexAttributeDescriptorRef {}

impl MTLVertexAttributeDescriptor {
    /// Creates a new vertex attribute descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLVertexAttributeDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLVertexAttributeDescriptor::from_ptr(obj)
        }
    }
}

impl MTLVertexAttributeDescriptorRef {
    /// Gets the format of this attribute.
    #[must_use]
    pub fn format(&self) -> MTLVertexFormat {
        unsafe { msg_send![self, format] }
    }

    /// Sets the format of this attribute.
    pub fn set_format(&self, format: MTLVertexFormat) {
        unsafe { msg_send![self, setFormat: format] }
    }

    /// Gets the offset of this attribute within the buffer.
    #[must_use]
    pub fn offset(&self) -> NSUInteger {
        unsafe { msg_send![self, offset] }
    }

    /// Sets the offset of this attribute within the buffer.
    pub fn set_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setOffset: offset] }
    }

    /// Gets the buffer index this attribute is associated with.
    #[must_use]
    pub fn buffer_index(&self) -> NSUInteger {
        unsafe { msg_send![self, bufferIndex] }
    }

    /// Sets the buffer index this attribute is associated with.
    pub fn set_buffer_index(&self, buffer_index: NSUInteger) {
        unsafe { msg_send![self, setBufferIndex: buffer_index] }
    }
}

impl Clone for MTLVertexAttributeDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLVertexAttributeDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLVertexAttributeDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLVertexAttributeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLVertexAttributeDescriptor")
            .field("format", &self.as_ref().format())
            .field("offset", &self.as_ref().offset())
            .field("buffer_index", &self.as_ref().buffer_index())
            .finish()
    }
}

/// A reference to an Objective-C `MTLVertexAttributeDescriptorArray`.
pub struct MTLVertexAttributeDescriptorArrayRef(Object);

/// An owned Objective-C `MTLVertexAttributeDescriptorArray`.
pub struct MTLVertexAttributeDescriptorArray(*mut Object);

unsafe impl ForeignTypeRef for MTLVertexAttributeDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLVertexAttributeDescriptorArrayRef {}
unsafe impl Sync for MTLVertexAttributeDescriptorArrayRef {}

unsafe impl ForeignType for MTLVertexAttributeDescriptorArray {
    type CType = Object;
    type Ref = MTLVertexAttributeDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLVertexAttributeDescriptorArray {
        MTLVertexAttributeDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLVertexAttributeDescriptorArrayRef> for MTLVertexAttributeDescriptorArray {
    fn as_ref(&self) -> &MTLVertexAttributeDescriptorArrayRef {
        unsafe { &*(self.0.cast::<MTLVertexAttributeDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLVertexAttributeDescriptorArray {}
unsafe impl Sync for MTLVertexAttributeDescriptorArray {}

unsafe impl objc::Message for MTLVertexAttributeDescriptorArrayRef {}

impl Clone for MTLVertexAttributeDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLVertexAttributeDescriptorArray::from_ptr(obj)
        }
    }
}

impl Drop for MTLVertexAttributeDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl MTLVertexAttributeDescriptorArrayRef {
    /// Gets the descriptor at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> Option<MTLVertexAttributeDescriptor> {
        unsafe {
            let obj: *mut Object = msg_send![self, objectAtIndexedSubscript: index];
            if obj.is_null() {
                None
            } else {
                Some(MTLVertexAttributeDescriptor::from_ptr(obj))
            }
        }
    }

    /// Sets the descriptor at the specified index.
    pub fn set_object(&self, descriptor: Option<&MTLVertexAttributeDescriptor>, index: NSUInteger) {
        unsafe {
            let ptr = match descriptor {
                Some(descriptor) => descriptor.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self, setObject:ptr atIndexedSubscript:index];
        }
    }
}

/// A reference to an Objective-C `MTLVertexDescriptor`.
pub struct MTLVertexDescriptorRef(Object);

/// An owned Objective-C `MTLVertexDescriptor`.
pub struct MTLVertexDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLVertexDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLVertexDescriptorRef {}
unsafe impl Sync for MTLVertexDescriptorRef {}

unsafe impl ForeignType for MTLVertexDescriptor {
    type CType = Object;
    type Ref = MTLVertexDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLVertexDescriptor {
        MTLVertexDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLVertexDescriptorRef> for MTLVertexDescriptor {
    fn as_ref(&self) -> &MTLVertexDescriptorRef {
        unsafe { &*(self.0.cast::<MTLVertexDescriptorRef>()) }
    }
}

unsafe impl Send for MTLVertexDescriptor {}
unsafe impl Sync for MTLVertexDescriptor {}

unsafe impl objc::Message for MTLVertexDescriptorRef {}

impl MTLVertexDescriptor {
    /// Creates a new vertex descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLVertexDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLVertexDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new vertex descriptor with default values.
    pub fn vertex_descriptor() -> Self {
        unsafe {
            let class = class!(MTLVertexDescriptor);
            let obj: *mut Object = msg_send![class, vertexDescriptor];
            MTLVertexDescriptor::from_ptr(obj)
        }
    }
}

impl MTLVertexDescriptorRef {
    /// Gets the collection of buffer layout descriptors.
    #[must_use]
    pub fn layouts(&self) -> MTLVertexBufferLayoutDescriptorArray {
        unsafe {
            let obj: *mut Object = msg_send![self, layouts];
            MTLVertexBufferLayoutDescriptorArray::from_ptr(obj)
        }
    }

    /// Gets the collection of attribute descriptors.
    #[must_use]
    pub fn attributes(&self) -> MTLVertexAttributeDescriptorArray {
        unsafe {
            let obj: *mut Object = msg_send![self, attributes];
            MTLVertexAttributeDescriptorArray::from_ptr(obj)
        }
    }

    /// Resets this descriptor to default values.
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}

impl Clone for MTLVertexDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLVertexDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLVertexDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLVertexDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLVertexDescriptor")
            .field("layouts", &"MTLVertexBufferLayoutDescriptorArray")
            .field("attributes", &"MTLVertexAttributeDescriptorArray")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // This test only runs on macOS with Metal support
    #[test]
    #[cfg(target_os = "macos")]
    #[ignore] // Only run manually since it requires a Metal environment
    fn test_vertex_descriptor_creation() {
        // Create a new vertex descriptor
        let vertex_descriptor = MTLVertexDescriptor::new();
        
        // Configure the vertex descriptor
        let layouts = vertex_descriptor.as_ref().layouts();
        let layout = layouts.as_ref().object(0).unwrap();
        layout.as_ref().set_stride(16); // 16 bytes stride
        layout.as_ref().set_step_function(MTLVertexStepFunction::PerVertex);
        layout.as_ref().set_step_rate(1);
        
        let attributes = vertex_descriptor.as_ref().attributes();
        let position_attribute = attributes.as_ref().object(0).unwrap();
        position_attribute.as_ref().set_format(MTLVertexFormat::Float3);
        position_attribute.as_ref().set_offset(0);
        position_attribute.as_ref().set_buffer_index(0);
        
        let texcoord_attribute = attributes.as_ref().object(1).unwrap();
        texcoord_attribute.as_ref().set_format(MTLVertexFormat::Float2);
        texcoord_attribute.as_ref().set_offset(12); // After 3 floats (12 bytes)
        texcoord_attribute.as_ref().set_buffer_index(0);
        
        // Verify the configuration
        assert_eq!(layout.as_ref().stride(), 16);
        assert_eq!(layout.as_ref().step_function(), MTLVertexStepFunction::PerVertex);
        assert_eq!(layout.as_ref().step_rate(), 1);
        
        assert_eq!(position_attribute.as_ref().format(), MTLVertexFormat::Float3);
        assert_eq!(position_attribute.as_ref().offset(), 0);
        assert_eq!(position_attribute.as_ref().buffer_index(), 0);
        
        assert_eq!(texcoord_attribute.as_ref().format(), MTLVertexFormat::Float2);
        assert_eq!(texcoord_attribute.as_ref().offset(), 12);
        assert_eq!(texcoord_attribute.as_ref().buffer_index(), 0);
        
        // Reset the descriptor
        vertex_descriptor.as_ref().reset();
        
        // Verify reset
        let reset_layouts = vertex_descriptor.as_ref().layouts();
        let reset_layout = reset_layouts.as_ref().object(0).unwrap();
        assert_eq!(reset_layout.as_ref().stride(), 0);
    }
}