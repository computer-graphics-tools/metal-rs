//! MTLRenderPipeline - A Rust wrapper around Metal's render pipeline types.
//!
//! This module provides safe Rust bindings to the render pipeline classes from Apple's Metal framework,
//! including render pipeline state, color attachments, and descriptors.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLPixelFormat};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a render pipeline descriptor
//! let pipeline_descriptor = metal_rs::metal::MTLRenderPipelineDescriptor::new();
//! pipeline_descriptor.set_label("Basic Render Pipeline");
//! 
//! // Configure pipeline for a simple vertex & fragment shader
//! let library = device.new_library_with_source(SHADER_SRC, &Default::default()).unwrap();
//! let vertex_function = library.get_function("vertex_main").unwrap();
//! let fragment_function = library.get_function("fragment_main").unwrap();
//! 
//! pipeline_descriptor.set_vertex_function(&vertex_function);
//! pipeline_descriptor.set_fragment_function(&fragment_function);
//! 
//! // Configure the output pixel format to match the drawable's format
//! let color_attachment = pipeline_descriptor.color_attachments().object(0);
//! color_attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
//! 
//! // Create the pipeline state
//! let pipeline_state = device.new_render_pipeline_state(&pipeline_descriptor).unwrap();
//! ```

use std::fmt;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::types::MTLPixelFormat;
use crate::metal::library::{MTLFunction};
use crate::metal::linked_functions::{MTLLinkedFunctions, MTLLinkedFunctionsRef};
use crate::metal::vertex_descriptor::{MTLVertexDescriptor, MTLVertexDescriptorRef};
use crate::metal::pipeline::{MTLPipelineBufferDescriptorArray};

/// Options for color write operations
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLColorWriteMask {
    /// Don't write any color components
    None = 0,
    /// Write only the red component
    Red = 8,
    /// Write only the green component
    Green = 4,
    /// Write only the blue component
    Blue = 2,
    /// Write only the alpha component
    Alpha = 1,
    /// Write all color components
    All = 15,
}

/// Blend factors for color blending operations
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLBlendFactor {
    /// The blend factor is (0, 0, 0, 0)
    Zero = 0,
    /// The blend factor is (1, 1, 1, 1)
    One = 1,
    /// The blend factor is (Rs, Gs, Bs, As)
    SourceColor = 2,
    /// The blend factor is (1 - Rs, 1 - Gs, 1 - Bs, 1 - As)
    OneMinusSourceColor = 3,
    /// The blend factor is (As, As, As, As)
    SourceAlpha = 4,
    /// The blend factor is (1 - As, 1 - As, 1 - As, 1 - As)
    OneMinusSourceAlpha = 5,
    /// The blend factor is (Rd, Gd, Bd, Ad)
    DestinationColor = 6,
    /// The blend factor is (1 - Rd, 1 - Gd, 1 - Bd, 1 - Ad)
    OneMinusDestinationColor = 7,
    /// The blend factor is (Ad, Ad, Ad, Ad)
    DestinationAlpha = 8,
    /// The blend factor is (1 - Ad, 1 - Ad, 1 - Ad, 1 - Ad)
    OneMinusDestinationAlpha = 9,
    /// The blend factor is (f, f, f, 1), where f = min(As, 1 - Ad)
    SourceAlphaSaturated = 10,
    /// The blend factor is (Rc, Gc, Bc, Ac)
    BlendColor = 11,
    /// The blend factor is (1 - Rc, 1 - Gc, 1 - Bc, 1 - Ac)
    OneMinusBlendColor = 12,
    /// The blend factor is (Ac, Ac, Ac, Ac)
    BlendAlpha = 13,
    /// The blend factor is (1 - Ac, 1 - Ac, 1 - Ac, 1 - Ac)
    OneMinusBlendAlpha = 14,
    /// The blend factor is (Rs1, Gs1, Bs1, As1)
    Source1Color = 15,
    /// The blend factor is (1 - Rs1, 1 - Gs1, 1 - Bs1, 1 - As1)
    OneMinusSource1Color = 16,
    /// The blend factor is (As1, As1, As1, As1)
    Source1Alpha = 17,
    /// The blend factor is (1 - As1, 1 - As1, 1 - As1, 1 - As1)
    OneMinusSource1Alpha = 18,
}

/// Blend operations for color blending
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLBlendOperation {
    /// Add source and destination
    Add = 0,
    /// Subtract destination from source
    Subtract = 1,
    /// Subtract source from destination
    ReverseSubtract = 2,
    /// Use minimum of source and destination
    Min = 3,
    /// Use maximum of source and destination 
    Max = 4,
}

/// Primitive topology class for geometry
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPrimitiveTopologyClass {
    /// Unspecified topology
    Unspecified = 0,
    /// Point topology
    Point = 1,
    /// Line topology
    Line = 2,
    /// Triangle topology
    Triangle = 3,
}

/// Options for pipeline creation
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPipelineOption {
    /// No options
    None = 0,
    /// Include argument information
    ArgumentInfo = 1,
    /// Include buffer type information
    BufferTypeInfo = 2,
    /// Fail creation if binary archive is missing
    FailOnBinaryArchiveMiss = 4,
}

/// A reference to a color attachment descriptor in an Objective-C `MTLRenderPipelineColorAttachmentDescriptor`.
pub struct MTLRenderPipelineColorAttachmentDescriptorRef(Object);

/// An owned color attachment descriptor in an Objective-C `MTLRenderPipelineColorAttachmentDescriptor`.
pub struct MTLRenderPipelineColorAttachmentDescriptor(*mut Object);

/// A reference to a collection of color attachment descriptors in an Objective-C `MTLRenderPipelineColorAttachmentDescriptorArray`.
pub struct MTLRenderPipelineColorAttachmentDescriptorArrayRef(Object);

/// An owned collection of color attachment descriptors in an Objective-C `MTLRenderPipelineColorAttachmentDescriptorArray`.
pub struct MTLRenderPipelineColorAttachmentDescriptorArray(*mut Object);

/// A reference to an Objective-C `MTLRenderPipelineDescriptor`.
pub struct MTLRenderPipelineDescriptorRef(Object);

/// An owned Objective-C `MTLRenderPipelineDescriptor`.
pub struct MTLRenderPipelineDescriptor(*mut Object);

/// A reference to an Objective-C `MTLRenderPipelineState`.
pub struct MTLRenderPipelineStateRef(Object);

/// An owned Objective-C `MTLRenderPipelineState`.
pub struct MTLRenderPipelineState(*mut Object);

// Implementation for MTLRenderPipelineColorAttachmentDescriptor
unsafe impl ForeignTypeRef for MTLRenderPipelineColorAttachmentDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPipelineColorAttachmentDescriptorRef {}
unsafe impl Sync for MTLRenderPipelineColorAttachmentDescriptorRef {}

unsafe impl ForeignType for MTLRenderPipelineColorAttachmentDescriptor {
    type CType = Object;
    type Ref = MTLRenderPipelineColorAttachmentDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPipelineColorAttachmentDescriptor {
        MTLRenderPipelineColorAttachmentDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPipelineColorAttachmentDescriptorRef> for MTLRenderPipelineColorAttachmentDescriptor {
    fn as_ref(&self) -> &MTLRenderPipelineColorAttachmentDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRenderPipelineColorAttachmentDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRenderPipelineColorAttachmentDescriptor {}
unsafe impl Sync for MTLRenderPipelineColorAttachmentDescriptor {}

unsafe impl objc::Message for MTLRenderPipelineColorAttachmentDescriptorRef {}

// Implementation for MTLRenderPipelineColorAttachmentDescriptorArray
unsafe impl ForeignTypeRef for MTLRenderPipelineColorAttachmentDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPipelineColorAttachmentDescriptorArrayRef {}
unsafe impl Sync for MTLRenderPipelineColorAttachmentDescriptorArrayRef {}

unsafe impl ForeignType for MTLRenderPipelineColorAttachmentDescriptorArray {
    type CType = Object;
    type Ref = MTLRenderPipelineColorAttachmentDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPipelineColorAttachmentDescriptorArray {
        MTLRenderPipelineColorAttachmentDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPipelineColorAttachmentDescriptorArrayRef> for MTLRenderPipelineColorAttachmentDescriptorArray {
    fn as_ref(&self) -> &MTLRenderPipelineColorAttachmentDescriptorArrayRef {
        unsafe { &*(self.0.cast::<MTLRenderPipelineColorAttachmentDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLRenderPipelineColorAttachmentDescriptorArray {}
unsafe impl Sync for MTLRenderPipelineColorAttachmentDescriptorArray {}

unsafe impl objc::Message for MTLRenderPipelineColorAttachmentDescriptorArrayRef {}

// Implementation for MTLRenderPipelineDescriptor
unsafe impl ForeignTypeRef for MTLRenderPipelineDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPipelineDescriptorRef {}
unsafe impl Sync for MTLRenderPipelineDescriptorRef {}

unsafe impl ForeignType for MTLRenderPipelineDescriptor {
    type CType = Object;
    type Ref = MTLRenderPipelineDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPipelineDescriptor {
        MTLRenderPipelineDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPipelineDescriptorRef> for MTLRenderPipelineDescriptor {
    fn as_ref(&self) -> &MTLRenderPipelineDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRenderPipelineDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRenderPipelineDescriptor {}
unsafe impl Sync for MTLRenderPipelineDescriptor {}

unsafe impl objc::Message for MTLRenderPipelineDescriptorRef {}

// Implementation for MTLRenderPipelineState
unsafe impl ForeignTypeRef for MTLRenderPipelineStateRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPipelineStateRef {}
unsafe impl Sync for MTLRenderPipelineStateRef {}

unsafe impl ForeignType for MTLRenderPipelineState {
    type CType = Object;
    type Ref = MTLRenderPipelineStateRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPipelineState {
        MTLRenderPipelineState(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPipelineStateRef> for MTLRenderPipelineState {
    fn as_ref(&self) -> &MTLRenderPipelineStateRef {
        unsafe { &*(self.0.cast::<MTLRenderPipelineStateRef>()) }
    }
}

unsafe impl Send for MTLRenderPipelineState {}
unsafe impl Sync for MTLRenderPipelineState {}

unsafe impl objc::Message for MTLRenderPipelineStateRef {}

impl MTLRenderPipelineColorAttachmentDescriptor {
    /// Creates a new render pipeline color attachment descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLRenderPipelineColorAttachmentDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLRenderPipelineColorAttachmentDescriptor::from_ptr(obj)
        }
    }
    
    /// Sets the pixel format for the attachment.
    pub fn set_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setPixelFormat:format];
        }
    }
    
    /// Gets the pixel format for the attachment.
    #[must_use]
    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.as_ref(), pixelFormat]
        }
    }
    
    /// Sets whether blending is enabled.
    pub fn set_blending_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setBlendingEnabled:enabled];
        }
    }
    
    /// Checks if blending is enabled.
    #[must_use]
    pub fn blending_enabled(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), isBlendingEnabled]
        }
    }
    
    /// Sets the source RGB blend factor.
    pub fn set_source_rgb_blend_factor(&self, factor: MTLBlendFactor) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setSourceRGBBlendFactor:factor];
        }
    }
    
    /// Gets the source RGB blend factor.
    #[must_use]
    pub fn source_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.as_ref(), sourceRGBBlendFactor]
        }
    }
    
    /// Sets the destination RGB blend factor.
    pub fn set_destination_rgb_blend_factor(&self, factor: MTLBlendFactor) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDestinationRGBBlendFactor:factor];
        }
    }
    
    /// Gets the destination RGB blend factor.
    #[must_use]
    pub fn destination_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.as_ref(), destinationRGBBlendFactor]
        }
    }
    
    /// Sets the RGB blend operation.
    pub fn set_rgb_blend_operation(&self, operation: MTLBlendOperation) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setRgbBlendOperation:operation];
        }
    }
    
    /// Gets the RGB blend operation.
    #[must_use]
    pub fn rgb_blend_operation(&self) -> MTLBlendOperation {
        unsafe {
            msg_send![self.as_ref(), rgbBlendOperation]
        }
    }
    
    /// Sets the source alpha blend factor.
    pub fn set_source_alpha_blend_factor(&self, factor: MTLBlendFactor) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setSourceAlphaBlendFactor:factor];
        }
    }
    
    /// Gets the source alpha blend factor.
    #[must_use]
    pub fn source_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.as_ref(), sourceAlphaBlendFactor]
        }
    }
    
    /// Sets the destination alpha blend factor.
    pub fn set_destination_alpha_blend_factor(&self, factor: MTLBlendFactor) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDestinationAlphaBlendFactor:factor];
        }
    }
    
    /// Gets the destination alpha blend factor.
    #[must_use]
    pub fn destination_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.as_ref(), destinationAlphaBlendFactor]
        }
    }
    
    /// Sets the alpha blend operation.
    pub fn set_alpha_blend_operation(&self, operation: MTLBlendOperation) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setAlphaBlendOperation:operation];
        }
    }
    
    /// Gets the alpha blend operation.
    #[must_use]
    pub fn alpha_blend_operation(&self) -> MTLBlendOperation {
        unsafe {
            msg_send![self.as_ref(), alphaBlendOperation]
        }
    }
    
    /// Sets the color write mask.
    pub fn set_write_mask(&self, mask: MTLColorWriteMask) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setWriteMask:mask];
        }
    }
    
    /// Gets the color write mask.
    #[must_use]
    pub fn write_mask(&self) -> MTLColorWriteMask {
        unsafe {
            msg_send![self.as_ref(), writeMask]
        }
    }
}

impl Drop for MTLRenderPipelineColorAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPipelineColorAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLRenderPipelineColorAttachmentDescriptor::from_ptr(obj)
        }
    }
}

impl Default for MTLRenderPipelineColorAttachmentDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLRenderPipelineColorAttachmentDescriptorArray {
    /// Access the color attachment descriptor at the given index.
    #[must_use]
    pub fn object(&self, index: usize) -> MTLRenderPipelineColorAttachmentDescriptor {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), objectAtIndexedSubscript:index];
            MTLRenderPipelineColorAttachmentDescriptor::from_ptr(ptr)
        }
    }
    
    /// Set the color attachment descriptor at the given index.
    pub fn set_object(&self, attachment: &MTLRenderPipelineColorAttachmentDescriptor, index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setObject:attachment.as_ptr() atIndexedSubscript:index];
        }
    }
}

impl MTLRenderPipelineDescriptor {
    /// Creates a new render pipeline descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLRenderPipelineDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLRenderPipelineDescriptor::from_ptr(obj)
        }
    }
    
    /// Sets the label for the descriptor.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Gets the label for the descriptor.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                String::new()
            } else {
                let ns_string = NSString::from_ptr(label);
                ns_string.to_rust_string()
            }
        }
    }
    
    /// Sets the vertex function.
    pub fn set_vertex_function(&self, function: &MTLFunction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVertexFunction:function.as_ptr()];
        }
    }
    
    /// Gets the vertex function.
    #[must_use]
    pub fn vertex_function(&self) -> Option<MTLFunction> {
        unsafe {
            let function: *mut Object = msg_send![self.as_ref(), vertexFunction];
            if function.is_null() {
                None
            } else {
                Some(MTLFunction::from_ptr(function))
            }
        }
    }
    
    /// Sets the fragment function.
    pub fn set_fragment_function(&self, function: &MTLFunction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFragmentFunction:function.as_ptr()];
        }
    }
    
    /// Gets the fragment function.
    #[must_use]
    pub fn fragment_function(&self) -> Option<MTLFunction> {
        unsafe {
            let function: *mut Object = msg_send![self.as_ref(), fragmentFunction];
            if function.is_null() {
                None
            } else {
                Some(MTLFunction::from_ptr(function))
            }
        }
    }
    
    /// Gets the color attachments.
    #[must_use]
    pub fn color_attachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), colorAttachments];
            MTLRenderPipelineColorAttachmentDescriptorArray::from_ptr(ptr)
        }
    }
    
    /// Sets the depth attachment pixel format.
    pub fn set_depth_attachment_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDepthAttachmentPixelFormat:format];
        }
    }
    
    /// Gets the depth attachment pixel format.
    #[must_use]
    pub fn depth_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.as_ref(), depthAttachmentPixelFormat]
        }
    }
    
    /// Sets the stencil attachment pixel format.
    pub fn set_stencil_attachment_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStencilAttachmentPixelFormat:format];
        }
    }
    
    /// Gets the stencil attachment pixel format.
    #[must_use]
    pub fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.as_ref(), stencilAttachmentPixelFormat]
        }
    }
    
    /// Sets the sample count.
    pub fn set_sample_count(&self, count: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setSampleCount:count];
        }
    }
    
    /// Gets the sample count.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), sampleCount]
        }
    }
    
    /// Sets the input primitive topology.
    pub fn set_input_primitive_topology(&self, topology: MTLPrimitiveTopologyClass) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setInputPrimitiveTopology:topology];
        }
    }
    
    /// Gets the input primitive topology.
    #[must_use]
    pub fn input_primitive_topology(&self) -> MTLPrimitiveTopologyClass {
        unsafe {
            msg_send![self.as_ref(), inputPrimitiveTopology]
        }
    }
    
    /// Resets the descriptor to default values.
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), reset];
        }
    }
    
    /// Sets the vertex descriptor.
    pub fn set_vertex_descriptor(&self, vertex_descriptor: Option<&MTLVertexDescriptorRef>) {
        unsafe {
            let ptr = match vertex_descriptor {
                Some(descriptor) => descriptor.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self.as_ref(), setVertexDescriptor:ptr];
        }
    }
    
    /// Gets the vertex descriptor.
    #[must_use]
    pub fn vertex_descriptor(&self) -> Option<MTLVertexDescriptor> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), vertexDescriptor];
            if ptr.is_null() {
                None
            } else {
                Some(MTLVertexDescriptor::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the vertex buffer descriptor array.
    #[must_use]
    pub fn vertex_buffers(&self) -> MTLPipelineBufferDescriptorArray {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), vertexBuffers];
            MTLPipelineBufferDescriptorArray::from_ptr(ptr)
        }
    }

    /// Gets the fragment buffer descriptor array.
    #[must_use]
    pub fn fragment_buffers(&self) -> MTLPipelineBufferDescriptorArray {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), fragmentBuffers];
            MTLPipelineBufferDescriptorArray::from_ptr(ptr)
        }
    }

    /// Gets the linked functions for the render pipeline.
    #[must_use]
    pub fn linked_functions(&self) -> Option<MTLLinkedFunctions> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), linkedFunctions];
            if ptr.is_null() {
                None
            } else {
                Some(MTLLinkedFunctions::from_ptr(ptr))
            }
        }
    }
    
    /// Sets the linked functions for the render pipeline.
    pub fn set_linked_functions(&self, linked_functions: &impl AsRef<MTLLinkedFunctionsRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setLinkedFunctions:linked_functions.as_ref().as_ptr()];
        }
    }
}

impl fmt::Debug for MTLRenderPipelineDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLRenderPipelineDescriptor")
            .field("label", &self.label())
            .field("linked_functions", &self.linked_functions())
            .finish()
    }
}

impl Drop for MTLRenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLRenderPipelineDescriptor::from_ptr(obj)
        }
    }
}

impl Default for MTLRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLRenderPipelineState {
    /// Gets the label of the render pipeline state.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                String::new()
            } else {
                let ns_string = NSString::from_ptr(label);
                ns_string.to_rust_string()
            }
        }
    }
    
    /// Gets the device that created this render pipeline state.
    #[must_use]
    pub fn device(&self) -> crate::metal::MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            crate::metal::MTLDevice::from_ptr(ptr)
        }
    }
    
    /// Gets the maximum total threads per threadgroup.
    #[must_use]
    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), maxTotalThreadsPerThreadgroup]
        }
    }
}

impl fmt::Debug for MTLRenderPipelineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label();
        let max_threads = self.max_total_threads_per_threadgroup();
        write!(f, "MTLRenderPipelineState {{ label: {}, max_threads: {} }}", label, max_threads)
    }
}

impl Drop for MTLRenderPipelineState {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPipelineState {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRenderPipelineState::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLRenderPipelineReflection`.
pub struct MTLRenderPipelineReflectionRef(Object);

/// An owned Objective-C `MTLRenderPipelineReflection`.
pub struct MTLRenderPipelineReflection(*mut Object);

unsafe impl ForeignTypeRef for MTLRenderPipelineReflectionRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPipelineReflectionRef {}
unsafe impl Sync for MTLRenderPipelineReflectionRef {}

unsafe impl ForeignType for MTLRenderPipelineReflection {
    type CType = Object;
    type Ref = MTLRenderPipelineReflectionRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPipelineReflection {
        MTLRenderPipelineReflection(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPipelineReflectionRef> for MTLRenderPipelineReflection {
    fn as_ref(&self) -> &MTLRenderPipelineReflectionRef {
        unsafe { &*(self.0.cast::<MTLRenderPipelineReflectionRef>()) }
    }
}

unsafe impl Send for MTLRenderPipelineReflection {}
unsafe impl Sync for MTLRenderPipelineReflection {}

unsafe impl objc::Message for MTLRenderPipelineReflectionRef {}

impl MTLRenderPipelineReflection {
    /// Returns the vertex bindings in the reflection data.
    #[must_use]
    pub fn vertex_bindings(&self) -> Vec<crate::foundation::NSString> {
        unsafe {
            let array: *mut Object = msg_send![self.as_ref(), vertexBindings];
            if array.is_null() {
                Vec::new()
            } else {
                let count: usize = msg_send![array, count];
                let mut result = Vec::with_capacity(count);
                for i in 0..count {
                    let obj: *mut Object = msg_send![array, objectAtIndex:i];
                    result.push(crate::foundation::NSString::from_ptr(obj));
                }
                result
            }
        }
    }
    
    /// Returns the fragment bindings in the reflection data.
    #[must_use]
    pub fn fragment_bindings(&self) -> Vec<crate::foundation::NSString> {
        unsafe {
            let array: *mut Object = msg_send![self.as_ref(), fragmentBindings];
            if array.is_null() {
                Vec::new()
            } else {
                let count: usize = msg_send![array, count];
                let mut result = Vec::with_capacity(count);
                for i in 0..count {
                    let obj: *mut Object = msg_send![array, objectAtIndex:i];
                    result.push(crate::foundation::NSString::from_ptr(obj));
                }
                result
            }
        }
    }
    
    /// Returns the vertex arguments in the reflection data.
    #[must_use]
    pub fn vertex_arguments(&self) -> Vec<crate::foundation::NSString> {
        unsafe {
            let array: *mut Object = msg_send![self.as_ref(), vertexArguments];
            if array.is_null() {
                Vec::new()
            } else {
                let count: usize = msg_send![array, count];
                let mut result = Vec::with_capacity(count);
                for i in 0..count {
                    let obj: *mut Object = msg_send![array, objectAtIndex:i];
                    result.push(crate::foundation::NSString::from_ptr(obj));
                }
                result
            }
        }
    }
    
    /// Returns the fragment arguments in the reflection data.
    #[must_use]
    pub fn fragment_arguments(&self) -> Vec<crate::foundation::NSString> {
        unsafe {
            let array: *mut Object = msg_send![self.as_ref(), fragmentArguments];
            if array.is_null() {
                Vec::new()
            } else {
                let count: usize = msg_send![array, count];
                let mut result = Vec::with_capacity(count);
                for i in 0..count {
                    let obj: *mut Object = msg_send![array, objectAtIndex:i];
                    result.push(crate::foundation::NSString::from_ptr(obj));
                }
                result
            }
        }
    }
}

impl Drop for MTLRenderPipelineReflection {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPipelineReflection {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRenderPipelineReflection::from_ptr(obj)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metal::{MTLCreateSystemDefaultDevice, MTLPixelFormat, MTLVertexFormat};
    use crate::metal::vertex_descriptor::MTLVertexDescriptor;
    
    const SHADER_SRC: &str = r#"
        #include <metal_stdlib>
        using namespace metal;
        
        struct VertexIn {
            float3 position [[attribute(0)]];
            float2 texCoord [[attribute(1)]];
        };
        
        struct VertexOut {
            float4 position [[position]];
            float2 texCoord;
        };
        
        vertex VertexOut vertex_main(VertexIn in [[stage_in]]) {
            VertexOut out;
            out.position = float4(in.position, 1.0);
            out.texCoord = in.texCoord;
            return out;
        }
        
        fragment float4 fragment_main(VertexOut in [[stage_in]]) {
            return float4(in.texCoord, 0.0, 1.0);
        }
    "#;
    
    #[test]
    #[cfg(target_os = "macos")]
    #[ignore] // Only run manually since it requires a Metal environment
    fn test_render_pipeline_creation() {
        // Get the default device
        let device = MTLCreateSystemDefaultDevice();
        
        // Create a library from our shader source
        let library = device.new_library_with_source(SHADER_SRC, &Default::default()).unwrap();
        
        // Get vertex and fragment functions
        let vertex_function = library.get_function("vertex_main").unwrap();
        let fragment_function = library.get_function("fragment_main").unwrap();
        
        // Create and configure a vertex descriptor
        let vertex_descriptor = MTLVertexDescriptor::new();
        
        // Configure position attribute
        let position_attribute = vertex_descriptor.as_ref().attributes().as_ref().object(0).unwrap();
        position_attribute.as_ref().set_format(MTLVertexFormat::Float3);
        position_attribute.as_ref().set_offset(0);
        position_attribute.as_ref().set_buffer_index(0);
        
        // Configure texcoord attribute
        let texcoord_attribute = vertex_descriptor.as_ref().attributes().as_ref().object(1).unwrap();
        texcoord_attribute.as_ref().set_format(MTLVertexFormat::Float2);
        texcoord_attribute.as_ref().set_offset(12); // After 3 floats (12 bytes)
        texcoord_attribute.as_ref().set_buffer_index(0);
        
        // Configure buffer layout
        let buffer_layout = vertex_descriptor.as_ref().layouts().as_ref().object(0).unwrap();
        buffer_layout.as_ref().set_stride(20); // 3 floats for position + 2 floats for texcoord = 5 floats = 20 bytes
        
        // Create a render pipeline descriptor
        let pipeline_descriptor = MTLRenderPipelineDescriptor::new();
        pipeline_descriptor.set_label("Test Render Pipeline");
        pipeline_descriptor.set_vertex_function(&vertex_function);
        pipeline_descriptor.set_fragment_function(&fragment_function);
        pipeline_descriptor.set_vertex_descriptor(Some(&vertex_descriptor.as_ref()));
        
        // Configure color attachment
        let color_attachment = pipeline_descriptor.color_attachments().object(0);
        color_attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        
        // Create the pipeline state
        let pipeline_result = device.new_render_pipeline_state(&pipeline_descriptor);
        assert!(pipeline_result.is_ok(), "Failed to create render pipeline state: {:?}", pipeline_result.err());
        
        // Test with reflection data
        let reflection_result = device.new_render_pipeline_state_with_reflection(
            &pipeline_descriptor,
            MTLPipelineOption::ArgumentInfo as u64
        );
        assert!(reflection_result.is_ok(), "Failed to create render pipeline state with reflection: {:?}", reflection_result.err());
        
        // Get the pipeline state and reflection data
        let (pipeline_state, reflection) = reflection_result.unwrap();
        
        // Verify pipeline state
        assert!(!pipeline_state.label().is_empty());
        assert!(pipeline_state.max_total_threads_per_threadgroup() > 0);
        
        // Verify reflection data if available
        if let Some(reflection) = reflection {
            // We should have vertex arguments information due to ArgumentInfo flag
            let vertex_args = reflection.vertex_arguments();
            println!("Vertex arguments: {:?}", vertex_args.len());
        }
    }
}