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
use crate::metal::render_command_encoder::MTLIndexType;

/// Specifies the data formats for attributes in a compute pipeline.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLAttributeFormat {
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
    UChar4Normalized_BGRA = 42,
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

/// Specifies how often buffer data is advanced during processing.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLStepFunction {
    /// The same data is used for every thread.
    Constant = 0,
    /// Data is advanced once per vertex.
    PerVertex = 1,
    /// Data is advanced once per instance.
    PerInstance = 2,
    /// Data is advanced once per patch.
    PerPatch = 3,
    /// Data is advanced once per patch control point.
    PerPatchControlPoint = 4,
    /// Data is advanced based on the X position of the thread in the grid.
    ThreadPositionInGridX = 5,
    /// Data is advanced based on the Y position of the thread in the grid.
    ThreadPositionInGridY = 6,
    /// Data is advanced based on the X position of the thread in the grid, using an index buffer.
    ThreadPositionInGridXIndexed = 7,
    /// Data is advanced based on the Y position of the thread in the grid, using an index buffer.
    ThreadPositionInGridYIndexed = 8,
}

/// A reference to an Objective-C `MTLBufferLayoutDescriptor`.
pub struct MTLBufferLayoutDescriptorRef(Object);

/// An owned Objective-C `MTLBufferLayoutDescriptor`.
pub struct MTLBufferLayoutDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLBufferLayoutDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLBufferLayoutDescriptorRef {}
unsafe impl Sync for MTLBufferLayoutDescriptorRef {}

unsafe impl ForeignType for MTLBufferLayoutDescriptor {
    type CType = Object;
    type Ref = MTLBufferLayoutDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBufferLayoutDescriptor {
        MTLBufferLayoutDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBufferLayoutDescriptorRef> for MTLBufferLayoutDescriptor {
    fn as_ref(&self) -> &MTLBufferLayoutDescriptorRef {
        unsafe { &*(self.0.cast::<MTLBufferLayoutDescriptorRef>()) }
    }
}

unsafe impl Send for MTLBufferLayoutDescriptor {}
unsafe impl Sync for MTLBufferLayoutDescriptor {}

unsafe impl objc::Message for MTLBufferLayoutDescriptorRef {}

impl MTLBufferLayoutDescriptor {
    /// Creates a new buffer layout descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLBufferLayoutDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLBufferLayoutDescriptor::from_ptr(obj)
        }
    }
}

impl MTLBufferLayoutDescriptorRef {
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
    pub fn step_function(&self) -> MTLStepFunction {
        unsafe { msg_send![self, stepFunction] }
    }

    /// Sets the step function of this buffer layout.
    pub fn set_step_function(&self, step_function: MTLStepFunction) {
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

impl Clone for MTLBufferLayoutDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLBufferLayoutDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLBufferLayoutDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLBufferLayoutDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLBufferLayoutDescriptor")
            .field("stride", &self.as_ref().stride())
            .field("step_function", &self.as_ref().step_function())
            .field("step_rate", &self.as_ref().step_rate())
            .finish()
    }
}

/// A reference to an Objective-C `MTLBufferLayoutDescriptorArray`.
pub struct MTLBufferLayoutDescriptorArrayRef(Object);

/// An owned Objective-C `MTLBufferLayoutDescriptorArray`.
pub struct MTLBufferLayoutDescriptorArray(*mut Object);

unsafe impl ForeignTypeRef for MTLBufferLayoutDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLBufferLayoutDescriptorArrayRef {}
unsafe impl Sync for MTLBufferLayoutDescriptorArrayRef {}

unsafe impl ForeignType for MTLBufferLayoutDescriptorArray {
    type CType = Object;
    type Ref = MTLBufferLayoutDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBufferLayoutDescriptorArray {
        MTLBufferLayoutDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBufferLayoutDescriptorArrayRef> for MTLBufferLayoutDescriptorArray {
    fn as_ref(&self) -> &MTLBufferLayoutDescriptorArrayRef {
        unsafe { &*(self.0.cast::<MTLBufferLayoutDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLBufferLayoutDescriptorArray {}
unsafe impl Sync for MTLBufferLayoutDescriptorArray {}

unsafe impl objc::Message for MTLBufferLayoutDescriptorArrayRef {}

impl Clone for MTLBufferLayoutDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLBufferLayoutDescriptorArray::from_ptr(obj)
        }
    }
}

impl Drop for MTLBufferLayoutDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl MTLBufferLayoutDescriptorArrayRef {
    /// Gets the descriptor at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> Option<MTLBufferLayoutDescriptor> {
        unsafe {
            let obj: *mut Object = msg_send![self, objectAtIndexedSubscript: index];
            if obj.is_null() {
                None
            } else {
                Some(MTLBufferLayoutDescriptor::from_ptr(obj))
            }
        }
    }

    /// Sets the descriptor at the specified index.
    pub fn set_object(&self, descriptor: Option<&MTLBufferLayoutDescriptor>, index: NSUInteger) {
        unsafe {
            let ptr = match descriptor {
                Some(descriptor) => descriptor.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self, setObject:ptr atIndexedSubscript:index];
        }
    }
}

/// A reference to an Objective-C `MTLAttributeDescriptor`.
pub struct MTLAttributeDescriptorRef(Object);

/// An owned Objective-C `MTLAttributeDescriptor`.
pub struct MTLAttributeDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLAttributeDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLAttributeDescriptorRef {}
unsafe impl Sync for MTLAttributeDescriptorRef {}

unsafe impl ForeignType for MTLAttributeDescriptor {
    type CType = Object;
    type Ref = MTLAttributeDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLAttributeDescriptor {
        MTLAttributeDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLAttributeDescriptorRef> for MTLAttributeDescriptor {
    fn as_ref(&self) -> &MTLAttributeDescriptorRef {
        unsafe { &*(self.0.cast::<MTLAttributeDescriptorRef>()) }
    }
}

unsafe impl Send for MTLAttributeDescriptor {}
unsafe impl Sync for MTLAttributeDescriptor {}

unsafe impl objc::Message for MTLAttributeDescriptorRef {}

impl MTLAttributeDescriptor {
    /// Creates a new attribute descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLAttributeDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLAttributeDescriptor::from_ptr(obj)
        }
    }
}

impl MTLAttributeDescriptorRef {
    /// Gets the format of this attribute.
    #[must_use]
    pub fn format(&self) -> MTLAttributeFormat {
        unsafe { msg_send![self, format] }
    }

    /// Sets the format of this attribute.
    pub fn set_format(&self, format: MTLAttributeFormat) {
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

impl Clone for MTLAttributeDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLAttributeDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLAttributeDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLAttributeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLAttributeDescriptor")
            .field("format", &self.as_ref().format())
            .field("offset", &self.as_ref().offset())
            .field("buffer_index", &self.as_ref().buffer_index())
            .finish()
    }
}

/// A reference to an Objective-C `MTLAttributeDescriptorArray`.
pub struct MTLAttributeDescriptorArrayRef(Object);

/// An owned Objective-C `MTLAttributeDescriptorArray`.
pub struct MTLAttributeDescriptorArray(*mut Object);

unsafe impl ForeignTypeRef for MTLAttributeDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLAttributeDescriptorArrayRef {}
unsafe impl Sync for MTLAttributeDescriptorArrayRef {}

unsafe impl ForeignType for MTLAttributeDescriptorArray {
    type CType = Object;
    type Ref = MTLAttributeDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLAttributeDescriptorArray {
        MTLAttributeDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLAttributeDescriptorArrayRef> for MTLAttributeDescriptorArray {
    fn as_ref(&self) -> &MTLAttributeDescriptorArrayRef {
        unsafe { &*(self.0.cast::<MTLAttributeDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLAttributeDescriptorArray {}
unsafe impl Sync for MTLAttributeDescriptorArray {}

unsafe impl objc::Message for MTLAttributeDescriptorArrayRef {}

impl Clone for MTLAttributeDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLAttributeDescriptorArray::from_ptr(obj)
        }
    }
}

impl Drop for MTLAttributeDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl MTLAttributeDescriptorArrayRef {
    /// Gets the descriptor at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> Option<MTLAttributeDescriptor> {
        unsafe {
            let obj: *mut Object = msg_send![self, objectAtIndexedSubscript: index];
            if obj.is_null() {
                None
            } else {
                Some(MTLAttributeDescriptor::from_ptr(obj))
            }
        }
    }

    /// Sets the descriptor at the specified index.
    pub fn set_object(&self, descriptor: Option<&MTLAttributeDescriptor>, index: NSUInteger) {
        unsafe {
            let ptr = match descriptor {
                Some(descriptor) => descriptor.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self, setObject:ptr atIndexedSubscript:index];
        }
    }
}

/// A reference to an Objective-C `MTLStageInputOutputDescriptor`.
pub struct MTLStageInputOutputDescriptorRef(Object);

/// An owned Objective-C `MTLStageInputOutputDescriptor`.
pub struct MTLStageInputOutputDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLStageInputOutputDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLStageInputOutputDescriptorRef {}
unsafe impl Sync for MTLStageInputOutputDescriptorRef {}

unsafe impl ForeignType for MTLStageInputOutputDescriptor {
    type CType = Object;
    type Ref = MTLStageInputOutputDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLStageInputOutputDescriptor {
        MTLStageInputOutputDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLStageInputOutputDescriptorRef> for MTLStageInputOutputDescriptor {
    fn as_ref(&self) -> &MTLStageInputOutputDescriptorRef {
        unsafe { &*(self.0.cast::<MTLStageInputOutputDescriptorRef>()) }
    }
}

unsafe impl Send for MTLStageInputOutputDescriptor {}
unsafe impl Sync for MTLStageInputOutputDescriptor {}

unsafe impl objc::Message for MTLStageInputOutputDescriptorRef {}

impl MTLStageInputOutputDescriptor {
    /// Creates a new stage input output descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLStageInputOutputDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLStageInputOutputDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new stage input output descriptor with default values.
    pub fn stage_input_output_descriptor() -> Self {
        unsafe {
            let class = class!(MTLStageInputOutputDescriptor);
            let obj: *mut Object = msg_send![class, stageInputOutputDescriptor];
            MTLStageInputOutputDescriptor::from_ptr(obj)
        }
    }
}

impl MTLStageInputOutputDescriptorRef {
    /// Gets the collection of buffer layout descriptors.
    #[must_use]
    pub fn layouts(&self) -> MTLBufferLayoutDescriptorArray {
        unsafe {
            let obj: *mut Object = msg_send![self, layouts];
            MTLBufferLayoutDescriptorArray::from_ptr(obj)
        }
    }

    /// Gets the collection of attribute descriptors.
    #[must_use]
    pub fn attributes(&self) -> MTLAttributeDescriptorArray {
        unsafe {
            let obj: *mut Object = msg_send![self, attributes];
            MTLAttributeDescriptorArray::from_ptr(obj)
        }
    }

    /// Gets the index type used by this descriptor.
    #[must_use]
    pub fn index_type(&self) -> MTLIndexType {
        unsafe { msg_send![self, indexType] }
    }

    /// Sets the index type used by this descriptor.
    pub fn set_index_type(&self, index_type: MTLIndexType) {
        unsafe { msg_send![self, setIndexType: index_type] }
    }

    /// Gets the index buffer index.
    #[must_use]
    pub fn index_buffer_index(&self) -> NSUInteger {
        unsafe { msg_send![self, indexBufferIndex] }
    }

    /// Sets the index buffer index.
    pub fn set_index_buffer_index(&self, index_buffer_index: NSUInteger) {
        unsafe { msg_send![self, setIndexBufferIndex: index_buffer_index] }
    }

    /// Resets this descriptor to default values.
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}

impl Clone for MTLStageInputOutputDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLStageInputOutputDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLStageInputOutputDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLStageInputOutputDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLStageInputOutputDescriptor")
            .field("layouts", &"MTLBufferLayoutDescriptorArray")
            .field("attributes", &"MTLAttributeDescriptorArray")
            .field("index_type", &self.as_ref().index_type())
            .field("index_buffer_index", &self.as_ref().index_buffer_index())
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
    fn test_stage_input_output_descriptor_creation() {
        // Create a new stage input output descriptor
        let descriptor = MTLStageInputOutputDescriptor::new();
        
        // Configure the buffer layouts
        let layouts = descriptor.as_ref().layouts();
        let layout = layouts.as_ref().object(0).unwrap();
        layout.as_ref().set_stride(16); // 16 bytes stride
        layout.as_ref().set_step_function(MTLStepFunction::PerVertex);
        layout.as_ref().set_step_rate(1);
        
        // Configure the attributes
        let attributes = descriptor.as_ref().attributes();
        let position_attribute = attributes.as_ref().object(0).unwrap();
        position_attribute.as_ref().set_format(MTLAttributeFormat::Float3);
        position_attribute.as_ref().set_offset(0);
        position_attribute.as_ref().set_buffer_index(0);
        
        // Configure the index buffer
        descriptor.as_ref().set_index_type(MTLIndexType::UInt16);
        descriptor.as_ref().set_index_buffer_index(1);
        
        // Verify the configuration
        assert_eq!(layout.as_ref().stride(), 16);
        assert_eq!(layout.as_ref().step_function(), MTLStepFunction::PerVertex);
        assert_eq!(layout.as_ref().step_rate(), 1);
        
        assert_eq!(position_attribute.as_ref().format(), MTLAttributeFormat::Float3);
        assert_eq!(position_attribute.as_ref().offset(), 0);
        assert_eq!(position_attribute.as_ref().buffer_index(), 0);
        
        assert_eq!(descriptor.as_ref().index_type(), MTLIndexType::UInt16);
        assert_eq!(descriptor.as_ref().index_buffer_index(), 1);
        
        // Reset the descriptor
        descriptor.as_ref().reset();
    }
}