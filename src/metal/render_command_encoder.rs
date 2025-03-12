//! MTLRenderCommandEncoder - A Rust wrapper around Metal's render command encoder.
//!
//! This module provides safe Rust bindings to the render command encoder class from Apple's Metal framework,
//! which is used to encode graphics rendering commands into a command buffer.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLLoadAction, MTLStoreAction, MTLClearColor};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue and command buffer
//! let command_queue = device.new_command_queue();
//! let command_buffer = command_queue.new_command_buffer();
//! 
//! // Set up a render pass descriptor
//! let render_pass_descriptor = metal_rs::metal::MTLRenderPassDescriptor::new();
//! let color_attachment = render_pass_descriptor.color_attachments().object(0);
//! color_attachment.set_texture(&texture); // texture would be from a CAMetalDrawable or created separately
//! color_attachment.set_load_action(MTLLoadAction::Clear);
//! color_attachment.set_store_action(MTLStoreAction::Store);
//! color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
//! 
//! // Create the render command encoder
//! let render_encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
//! render_encoder.set_render_pipeline_state(&pipeline_state);
//! 
//! // Set vertex and fragment data
//! render_encoder.set_vertex_buffer(&vertex_buffer, 0, 0);
//! 
//! // Draw primitives
//! render_encoder.draw_primitives(metal_rs::metal::MTLPrimitiveType::Triangle, 0, 3, 1);
//! 
//! // End encoding and commit the command buffer
//! render_encoder.end_encoding();
//! command_buffer.commit();
//! ```

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::metal::render_pipeline::MTLRenderPipelineState;
use crate::metal::buffer::MTLBuffer;
use crate::metal::texture::MTLTexture;
use crate::metal::fence::MTLFenceRef;
use crate::foundation::NSUInteger;
use crate::metal::counters::MTLCounterSampleBufferRef;

/// Types of primitive for rendering.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPrimitiveType {
    /// Rasterize points at the vertices.
    Point = 0,
    /// Rasterize lines between consecutive vertices.
    Line = 1,
    /// Rasterize lines, with a break between each pair.
    LineStrip = 2,
    /// Rasterize triangles from each group of three vertices.
    Triangle = 3,
    /// Rasterize triangles in a strip.
    TriangleStrip = 4,
}

/// Winding order for polygon faces.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLWinding {
    /// Clockwise winding.
    Clockwise = 0,
    /// Counter-clockwise winding.
    CounterClockwise = 1,
}

/// Culling mode for faces.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLCullMode {
    /// Don't cull any faces.
    None = 0,
    /// Cull front faces.
    Front = 1,
    /// Cull back faces.
    Back = 2,
}

/// Depth clip mode for vertices.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLDepthClipMode {
    /// Clip vertices outside the depth range.
    Clip = 0,
    /// Clamp vertices to the depth range.
    Clamp = 1,
}

/// Types of indices for indexed drawing.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLIndexType {
    /// 16-bit unsigned integer indices.
    UInt16 = 0,
    /// 32-bit unsigned integer indices.
    UInt32 = 1,
}

/// Fill mode for triangles.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLTriangleFillMode {
    /// Fill the triangles.
    Fill = 0,
    /// Draw the triangle outlines.
    Lines = 1,
}

/// Viewport definition for 3D rendering.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLViewport {
    /// The x-coordinate of the viewport's origin.
    pub origin_x: f64,
    /// The y-coordinate of the viewport's origin.
    pub origin_y: f64,
    /// The width of the viewport.
    pub width: f64,
    /// The height of the viewport.
    pub height: f64,
    /// The near depth value of the viewport.
    pub znear: f64,
    /// The far depth value of the viewport.
    pub zfar: f64,
}

impl MTLViewport {
    /// Creates a new viewport.
    #[must_use]
    pub fn new(origin_x: f64, origin_y: f64, width: f64, height: f64, znear: f64, zfar: f64) -> Self {
        MTLViewport {
            origin_x,
            origin_y,
            width,
            height,
            znear,
            zfar,
        }
    }
}

/// Scissor rectangle definition for clipping.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLScissorRect {
    /// The x-coordinate of the rectangle's origin.
    pub x: usize,
    /// The y-coordinate of the rectangle's origin.
    pub y: usize,
    /// The width of the rectangle.
    pub width: usize,
    /// The height of the rectangle.
    pub height: usize,
}

impl MTLScissorRect {
    /// Creates a new scissor rectangle.
    #[must_use]
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        MTLScissorRect {
            x,
            y,
            width,
            height,
        }
    }
}

/// A reference to an Objective-C `MTLRenderPassDescriptor`.
pub struct MTLRenderPassDescriptorRef(Object);

/// An owned Objective-C `MTLRenderPassDescriptor`.
pub struct MTLRenderPassDescriptor(*mut Object);

/// A reference to an Objective-C `MTLRenderPassColorAttachmentDescriptor`.
pub struct MTLRenderPassColorAttachmentDescriptorRef(Object);

/// An owned Objective-C `MTLRenderPassColorAttachmentDescriptor`.
pub struct MTLRenderPassColorAttachmentDescriptor(*mut Object);

/// A reference to an Objective-C `MTLRenderPassColorAttachmentDescriptorArray`.
pub struct MTLRenderPassColorAttachmentDescriptorArrayRef(Object);

/// An owned Objective-C `MTLRenderPassColorAttachmentDescriptorArray`.
pub struct MTLRenderPassColorAttachmentDescriptorArray(*mut Object);

/// A reference to an Objective-C `MTLRenderCommandEncoder`.
pub struct MTLRenderCommandEncoderRef(Object);

/// An owned Objective-C `MTLRenderCommandEncoder`.
pub struct MTLRenderCommandEncoder(*mut Object);

// Implementation for MTLRenderPassDescriptor
unsafe impl ForeignTypeRef for MTLRenderPassDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPassDescriptorRef {}
unsafe impl Sync for MTLRenderPassDescriptorRef {}

unsafe impl ForeignType for MTLRenderPassDescriptor {
    type CType = Object;
    type Ref = MTLRenderPassDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPassDescriptor {
        MTLRenderPassDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPassDescriptorRef> for MTLRenderPassDescriptor {
    fn as_ref(&self) -> &MTLRenderPassDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRenderPassDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRenderPassDescriptor {}
unsafe impl Sync for MTLRenderPassDescriptor {}

unsafe impl objc::Message for MTLRenderPassDescriptorRef {}

// Implementation for MTLRenderPassColorAttachmentDescriptor
unsafe impl ForeignTypeRef for MTLRenderPassColorAttachmentDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPassColorAttachmentDescriptorRef {}
unsafe impl Sync for MTLRenderPassColorAttachmentDescriptorRef {}

unsafe impl ForeignType for MTLRenderPassColorAttachmentDescriptor {
    type CType = Object;
    type Ref = MTLRenderPassColorAttachmentDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPassColorAttachmentDescriptor {
        MTLRenderPassColorAttachmentDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPassColorAttachmentDescriptorRef> for MTLRenderPassColorAttachmentDescriptor {
    fn as_ref(&self) -> &MTLRenderPassColorAttachmentDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRenderPassColorAttachmentDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRenderPassColorAttachmentDescriptor {}
unsafe impl Sync for MTLRenderPassColorAttachmentDescriptor {}

unsafe impl objc::Message for MTLRenderPassColorAttachmentDescriptorRef {}

// Implementation for MTLRenderPassColorAttachmentDescriptorArray
unsafe impl ForeignTypeRef for MTLRenderPassColorAttachmentDescriptorArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPassColorAttachmentDescriptorArrayRef {}
unsafe impl Sync for MTLRenderPassColorAttachmentDescriptorArrayRef {}

unsafe impl ForeignType for MTLRenderPassColorAttachmentDescriptorArray {
    type CType = Object;
    type Ref = MTLRenderPassColorAttachmentDescriptorArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPassColorAttachmentDescriptorArray {
        MTLRenderPassColorAttachmentDescriptorArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPassColorAttachmentDescriptorArrayRef> for MTLRenderPassColorAttachmentDescriptorArray {
    fn as_ref(&self) -> &MTLRenderPassColorAttachmentDescriptorArrayRef {
        unsafe { &*(self.0.cast::<MTLRenderPassColorAttachmentDescriptorArrayRef>()) }
    }
}

unsafe impl Send for MTLRenderPassColorAttachmentDescriptorArray {}
unsafe impl Sync for MTLRenderPassColorAttachmentDescriptorArray {}

unsafe impl objc::Message for MTLRenderPassColorAttachmentDescriptorArrayRef {}

// Implementation for MTLRenderCommandEncoder
unsafe impl ForeignTypeRef for MTLRenderCommandEncoderRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderCommandEncoderRef {}
unsafe impl Sync for MTLRenderCommandEncoderRef {}

unsafe impl ForeignType for MTLRenderCommandEncoder {
    type CType = Object;
    type Ref = MTLRenderCommandEncoderRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderCommandEncoder {
        MTLRenderCommandEncoder(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderCommandEncoderRef> for MTLRenderCommandEncoder {
    fn as_ref(&self) -> &MTLRenderCommandEncoderRef {
        unsafe { &*(self.0.cast()) }
    }
}

// Removed the MTLCommandEncoderRef AsRef implementation to avoid ambiguity
// The MTLRenderCommandEncoder should be used with MTLRenderCommandEncoderRef exclusively

unsafe impl Send for MTLRenderCommandEncoder {}
unsafe impl Sync for MTLRenderCommandEncoder {}

unsafe impl objc::Message for MTLRenderCommandEncoderRef {}

impl MTLRenderPassDescriptor {
    /// Creates a new render pass descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = objc::class!(MTLRenderPassDescriptor);
            let obj: *mut Object = msg_send![class, renderPassDescriptor];
            MTLRenderPassDescriptor::from_ptr(obj)
        }
    }
    
    /// Gets the color attachments array.
    #[must_use]
    pub fn color_attachments(&self) -> MTLRenderPassColorAttachmentDescriptorArray {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), colorAttachments];
            MTLRenderPassColorAttachmentDescriptorArray::from_ptr(ptr)
        }
    }
}

impl Drop for MTLRenderPassDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLRenderPassDescriptor::from_ptr(obj)
        }
    }
}

impl Default for MTLRenderPassDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLRenderPassColorAttachmentDescriptorArray {
    /// Access the color attachment descriptor at the given index.
    #[must_use]
    pub fn object(&self, index: usize) -> MTLRenderPassColorAttachmentDescriptor {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), objectAtIndexedSubscript:index];
            MTLRenderPassColorAttachmentDescriptor::from_ptr(ptr)
        }
    }
}

impl MTLRenderPassColorAttachmentDescriptor {
    /// Sets the texture for this attachment.
    pub fn set_texture(&self, texture: &MTLTexture) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setTexture:texture.as_ptr()];
        }
    }
    
    /// Gets the texture for this attachment.
    #[must_use]
    pub fn texture(&self) -> Option<MTLTexture> {
        unsafe {
            let texture: *mut Object = msg_send![self.as_ref(), texture];
            if texture.is_null() {
                None
            } else {
                Some(MTLTexture::from_ptr(texture))
            }
        }
    }
    
    /// Sets the load action for this attachment.
    pub fn set_load_action(&self, action: crate::metal::MTLLoadAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setLoadAction:action];
        }
    }
    
    /// Gets the load action for this attachment.
    #[must_use]
    pub fn load_action(&self) -> crate::metal::MTLLoadAction {
        unsafe {
            msg_send![self.as_ref(), loadAction]
        }
    }
    
    /// Sets the store action for this attachment.
    pub fn set_store_action(&self, action: crate::metal::MTLStoreAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStoreAction:action];
        }
    }
    
    /// Gets the store action for this attachment.
    #[must_use]
    pub fn store_action(&self) -> crate::metal::MTLStoreAction {
        unsafe {
            msg_send![self.as_ref(), storeAction]
        }
    }
    
    /// Sets the clear color for this attachment.
    pub fn set_clear_color(&self, color: crate::metal::MTLClearColor) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setClearColor:color];
        }
    }
    
    /// Gets the clear color for this attachment.
    #[must_use]
    pub fn clear_color(&self) -> crate::metal::MTLClearColor {
        unsafe {
            msg_send![self.as_ref(), clearColor]
        }
    }
}

impl MTLRenderCommandEncoder {
    /// Sets the render pipeline state for this encoder.
    pub fn set_render_pipeline_state(&self, pipeline_state: &MTLRenderPipelineState) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setRenderPipelineState:pipeline_state.as_ptr()];
        }
    }
    
    /// Sets a vertex buffer for the given index.
    pub fn set_vertex_buffer(&self, buffer: &MTLBuffer, offset: usize, index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVertexBuffer:buffer.as_ptr() offset:offset atIndex:index];
        }
    }
    
    /// Sets a vertex buffer offset for the given index.
    pub fn set_vertex_buffer_offset(&self, offset: usize, index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVertexBufferOffset:offset atIndex:index];
        }
    }
    
    /// Sets a fragment buffer for the given index.
    pub fn set_fragment_buffer(&self, buffer: &MTLBuffer, offset: usize, index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFragmentBuffer:buffer.as_ptr() offset:offset atIndex:index];
        }
    }
    
    /// Sets vertex bytes directly for the given index.
    pub fn set_vertex_bytes(&self, bytes: &[u8], index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVertexBytes:bytes.as_ptr() 
                                                        length:bytes.len() 
                                                       atIndex:index];
        }
    }
    
    /// Sets fragment bytes directly for the given index.
    pub fn set_fragment_bytes(&self, bytes: &[u8], index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFragmentBytes:bytes.as_ptr() 
                                                          length:bytes.len() 
                                                         atIndex:index];
        }
    }
    
    /// Sets a vertex texture for the given index.
    pub fn set_vertex_texture(&self, texture: &MTLTexture, index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVertexTexture:texture.as_ptr() atIndex:index];
        }
    }
    
    /// Sets a fragment texture for the given index.
    pub fn set_fragment_texture(&self, texture: &MTLTexture, index: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFragmentTexture:texture.as_ptr() atIndex:index];
        }
    }
    
    /// Sets the viewport.
    pub fn set_viewport(&self, viewport: MTLViewport) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setViewport:viewport];
        }
    }
    
    /// Sets the front facing winding.
    pub fn set_front_facing_winding(&self, winding: MTLWinding) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFrontFacingWinding:winding];
        }
    }
    
    /// Sets the cull mode.
    pub fn set_cull_mode(&self, mode: MTLCullMode) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setCullMode:mode];
        }
    }
    
    /// Sets the depth clip mode.
    pub fn set_depth_clip_mode(&self, mode: MTLDepthClipMode) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDepthClipMode:mode];
        }
    }
    
    /// Sets the depth bias.
    pub fn set_depth_bias(&self, depth_bias: f32, slope_scale: f32, clamp: f32) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDepthBias:depth_bias slopeScale:slope_scale clamp:clamp];
        }
    }
    
    /// Sets the scissor rect.
    pub fn set_scissor_rect(&self, rect: MTLScissorRect) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setScissorRect:rect];
        }
    }
    
    /// Sets the triangle fill mode.
    pub fn set_triangle_fill_mode(&self, mode: MTLTriangleFillMode) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setTriangleFillMode:mode];
        }
    }
    
    /// Draws primitives.
    pub fn draw_primitives(&self, primitive_type: MTLPrimitiveType, vertex_start: usize, vertex_count: usize, instance_count: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), drawPrimitives:primitive_type 
                                                    vertexStart:vertex_start 
                                                    vertexCount:vertex_count 
                                                  instanceCount:instance_count];
        }
    }
    
    /// Draws primitives with a single instance.
    pub fn draw_primitives_single_instance(&self, primitive_type: MTLPrimitiveType, vertex_start: usize, vertex_count: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), drawPrimitives:primitive_type 
                                                    vertexStart:vertex_start 
                                                    vertexCount:vertex_count];
        }
    }
    
    /// Draws indexed primitives.
    pub fn draw_indexed_primitives(
        &self, 
        primitive_type: MTLPrimitiveType, 
        index_count: usize, 
        index_type: MTLIndexType, 
        index_buffer: &MTLBuffer, 
        index_buffer_offset: usize, 
        instance_count: usize
    ) {
        unsafe {
            let _: () = msg_send![self.as_ref(), drawIndexedPrimitives:primitive_type 
                                                           indexCount:index_count 
                                                            indexType:index_type 
                                                          indexBuffer:index_buffer.as_ptr() 
                                                    indexBufferOffset:index_buffer_offset 
                                                        instanceCount:instance_count];
        }
    }
    
    /// Draws indexed primitives with a single instance.
    pub fn draw_indexed_primitives_single_instance(
        &self, 
        primitive_type: MTLPrimitiveType, 
        index_count: usize, 
        index_type: MTLIndexType, 
        index_buffer: &MTLBuffer, 
        index_buffer_offset: usize
    ) {
        unsafe {
            let _: () = msg_send![self.as_ref(), drawIndexedPrimitives:primitive_type 
                                                           indexCount:index_count 
                                                            indexType:index_type 
                                                          indexBuffer:index_buffer.as_ptr() 
                                                    indexBufferOffset:index_buffer_offset];
        }
    }
    
    /// Sets the blend color.
    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setBlendColorRed:red green:green blue:blue alpha:alpha];
        }
    }
    
    /// End encoding commands into the command buffer.
    pub fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), endEncoding];
        }
    }
    
    /// Sample GPU performance counters within this encoder.
    ///
    /// # Arguments
    ///
    /// * `sample_buffer` - The counter sample buffer to store the samples in.
    /// * `sample_index` - The index into the sample buffer to store the sample at.
    /// * `barrier` - Whether to insert a barrier before taking the sample.
    pub fn sample_counters_in_buffer(&self, sample_buffer: &impl AsRef<MTLCounterSampleBufferRef>, sample_index: NSUInteger, barrier: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), sampleCountersInBuffer:sample_buffer.as_ref().as_ptr() atSampleIndex:sample_index withBarrier:barrier];
        }
    }
    
    /// Updates a fence to establish a dependency between commands in this encoder and commands in subsequent encoders.
    pub fn update_fence(&self, fence: &impl AsRef<MTLFenceRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), updateFence:fence.as_ref().as_ptr()];
        }
    }
    
    /// Makes this encoder wait for a fence from a previous encoder to complete its work.
    pub fn wait_for_fence(&self, fence: &impl AsRef<MTLFenceRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), waitForFence:fence.as_ref().as_ptr()];
        }
    }
}

impl Drop for MTLRenderCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRenderCommandEncoder::from_ptr(obj)
        }
    }
}