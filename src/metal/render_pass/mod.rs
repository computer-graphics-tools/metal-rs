//! MTLRenderPass - Metal render pass interface and descriptors
//! 
//! This module provides safe Rust bindings to Metal's render pass interface and descriptors,
//! which are used to configure rendering operations, including:
//! - Color attachments
//! - Depth attachments
//! - Stencil attachments
//! - Sample buffer attachments
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
//! 
//! // Configure color attachment
//! let color_attachment = render_pass_descriptor.color_attachments().object(0);
//! color_attachment.set_texture(&texture); // texture would be from a CAMetalDrawable or created separately
//! color_attachment.set_load_action(MTLLoadAction::Clear);
//! color_attachment.set_store_action(MTLStoreAction::Store);
//! color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
//! 
//! // Configure depth attachment
//! let depth_attachment = render_pass_descriptor.depth_attachment();
//! depth_attachment.set_texture(&depth_texture);
//! depth_attachment.set_load_action(MTLLoadAction::Clear);
//! depth_attachment.set_store_action(MTLStoreAction::Store);
//! depth_attachment.set_clear_depth(1.0);
//! 
//! // Create the render command encoder
//! let render_encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
//! render_encoder.end_encoding();
//! command_buffer.commit();
//! ```

mod descriptor;

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::metal::texture::MTLTexture;

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

/// A reference to an Objective-C `MTLRenderPassDepthAttachmentDescriptor`.
pub struct MTLRenderPassDepthAttachmentDescriptorRef(Object);

/// An owned Objective-C `MTLRenderPassDepthAttachmentDescriptor`.
pub struct MTLRenderPassDepthAttachmentDescriptor(*mut Object);

/// A reference to an Objective-C `MTLRenderPassStencilAttachmentDescriptor`.
pub struct MTLRenderPassStencilAttachmentDescriptorRef(Object);

/// An owned Objective-C `MTLRenderPassStencilAttachmentDescriptor`.
pub struct MTLRenderPassStencilAttachmentDescriptor(*mut Object);

/// Load action options for attachments.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLLoadAction {
    /// Don't care about the initial contents. Contents may be undefined.
    DontCare = 0,
    /// Load the attachment contents from the texture.
    Load = 1,
    /// Clear the attachment to the clear value.
    Clear = 2,
}

/// Store action options for attachments.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLStoreAction {
    /// Don't care about the final contents. Contents may be discarded.
    DontCare = 0,
    /// Store the attachment contents to the texture.
    Store = 1,
    /// Resolve multisampled contents and store to the resolve texture.
    MultisampleResolve = 2,
    /// Store the attachment contents and resolve multisampled contents.
    StoreAndMultisampleResolve = 3,
    /// Don't store the attachment contents but do keep the resource in the same state.
    Unknown = 4,
    /// Custom resolve operations.
    CustomSampleDepthResolve = 5,
}

/// Store action options for attachments.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLStoreActionOptions {
    /// No special options.
    None = 0,
    /// Store content in a way that can be used for a custom sample resolve operation.
    CustomSamplePositions = 1,
}

/// Multisample depth resolve filter.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLMultisampleDepthResolveFilter {
    /// Sampling filter not defined.
    None = 0,
    /// Sample closest to the center of the pixel.
    Sample0 = 1,
    /// Take the minimum depth value from the samples.
    Min = 2,
    /// Take the maximum depth value from the samples.
    Max = 3,
}

/// Multisample stencil resolve filter.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLMultisampleStencilResolveFilter {
    /// Sampling filter not defined.
    None = 0,
    /// Sample closest to the center of the pixel.
    Sample0 = 1,
    /// Bitwise OR of all samples.
    DepthResolvedSample = 2,
}

/// Clear color value.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLClearColor {
    /// Red component
    pub red: f64,
    /// Green component
    pub green: f64,
    /// Blue component
    pub blue: f64,
    /// Alpha component
    pub alpha: f64,
}

impl MTLClearColor {
    /// Creates a new clear color.
    #[must_use]
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        MTLClearColor {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Default for MTLClearColor {
    fn default() -> Self {
        MTLClearColor::new(0.0, 0.0, 0.0, 1.0)
    }
}

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

// Implementation for MTLRenderPassDepthAttachmentDescriptor
unsafe impl ForeignTypeRef for MTLRenderPassDepthAttachmentDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPassDepthAttachmentDescriptorRef {}
unsafe impl Sync for MTLRenderPassDepthAttachmentDescriptorRef {}

unsafe impl ForeignType for MTLRenderPassDepthAttachmentDescriptor {
    type CType = Object;
    type Ref = MTLRenderPassDepthAttachmentDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPassDepthAttachmentDescriptor {
        MTLRenderPassDepthAttachmentDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPassDepthAttachmentDescriptorRef> for MTLRenderPassDepthAttachmentDescriptor {
    fn as_ref(&self) -> &MTLRenderPassDepthAttachmentDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRenderPassDepthAttachmentDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRenderPassDepthAttachmentDescriptor {}
unsafe impl Sync for MTLRenderPassDepthAttachmentDescriptor {}

unsafe impl objc::Message for MTLRenderPassDepthAttachmentDescriptorRef {}

// Implementation for MTLRenderPassStencilAttachmentDescriptor
unsafe impl ForeignTypeRef for MTLRenderPassStencilAttachmentDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRenderPassStencilAttachmentDescriptorRef {}
unsafe impl Sync for MTLRenderPassStencilAttachmentDescriptorRef {}

unsafe impl ForeignType for MTLRenderPassStencilAttachmentDescriptor {
    type CType = Object;
    type Ref = MTLRenderPassStencilAttachmentDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRenderPassStencilAttachmentDescriptor {
        MTLRenderPassStencilAttachmentDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRenderPassStencilAttachmentDescriptorRef> for MTLRenderPassStencilAttachmentDescriptor {
    fn as_ref(&self) -> &MTLRenderPassStencilAttachmentDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRenderPassStencilAttachmentDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRenderPassStencilAttachmentDescriptor {}
unsafe impl Sync for MTLRenderPassStencilAttachmentDescriptor {}

unsafe impl objc::Message for MTLRenderPassStencilAttachmentDescriptorRef {}

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

impl fmt::Debug for MTLRenderPassDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLRenderPassDescriptor")
            .finish()
    }
}

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

    /// Gets the depth attachment.
    #[must_use]
    pub fn depth_attachment(&self) -> MTLRenderPassDepthAttachmentDescriptor {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), depthAttachment];
            MTLRenderPassDepthAttachmentDescriptor::from_ptr(ptr)
        }
    }

    /// Gets the stencil attachment.
    #[must_use]
    pub fn stencil_attachment(&self) -> MTLRenderPassStencilAttachmentDescriptor {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), stencilAttachment];
            MTLRenderPassStencilAttachmentDescriptor::from_ptr(ptr)
        }
    }

    /// Sets the visibility result buffer.
    pub fn set_visibility_result_buffer(&self, buffer: &crate::metal::buffer::MTLBuffer) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setVisibilityResultBuffer:buffer.as_ptr()];
        }
    }

    /// Sets the render target array length.
    pub fn set_render_target_array_length(&self, length: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setRenderTargetArrayLength:length];
        }
    }

    /// Sets whether the render pass descriptor is using the default sample positions.
    pub fn set_default_raster_sample_positions(&self, default: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDefaultRasterSamplePositions:default];
        }
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
    pub fn set_load_action(&self, action: MTLLoadAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setLoadAction:action];
        }
    }
    
    /// Gets the load action for this attachment.
    #[must_use]
    pub fn load_action(&self) -> MTLLoadAction {
        unsafe {
            msg_send![self.as_ref(), loadAction]
        }
    }
    
    /// Sets the store action for this attachment.
    pub fn set_store_action(&self, action: MTLStoreAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStoreAction:action];
        }
    }
    
    /// Gets the store action for this attachment.
    #[must_use]
    pub fn store_action(&self) -> MTLStoreAction {
        unsafe {
            msg_send![self.as_ref(), storeAction]
        }
    }
    
    /// Sets the clear color for this attachment.
    pub fn set_clear_color(&self, color: MTLClearColor) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setClearColor:color];
        }
    }
    
    /// Gets the clear color for this attachment.
    #[must_use]
    pub fn clear_color(&self) -> MTLClearColor {
        unsafe {
            msg_send![self.as_ref(), clearColor]
        }
    }

    /// Sets the resolve texture for this attachment.
    pub fn set_resolve_texture(&self, texture: &MTLTexture) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setResolveTexture:texture.as_ptr()];
        }
    }
    
    /// Gets the resolve texture for this attachment.
    #[must_use]
    pub fn resolve_texture(&self) -> Option<MTLTexture> {
        unsafe {
            let texture: *mut Object = msg_send![self.as_ref(), resolveTexture];
            if texture.is_null() {
                None
            } else {
                Some(MTLTexture::from_ptr(texture))
            }
        }
    }

    /// Sets the resolve slice for this attachment.
    pub fn set_resolve_slice(&self, slice: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setResolveSlice:slice];
        }
    }

    /// Gets the resolve slice for this attachment.
    #[must_use]
    pub fn resolve_slice(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), resolveSlice]
        }
    }

    /// Sets the resolve level for this attachment.
    pub fn set_resolve_level(&self, level: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setResolveLevel:level];
        }
    }

    /// Gets the resolve level for this attachment.
    #[must_use]
    pub fn resolve_level(&self) -> usize {
        unsafe {
            msg_send![self.as_ref(), resolveLevel]
        }
    }
}

impl MTLRenderPassDepthAttachmentDescriptor {
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
    pub fn set_load_action(&self, action: MTLLoadAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setLoadAction:action];
        }
    }
    
    /// Gets the load action for this attachment.
    #[must_use]
    pub fn load_action(&self) -> MTLLoadAction {
        unsafe {
            msg_send![self.as_ref(), loadAction]
        }
    }
    
    /// Sets the store action for this attachment.
    pub fn set_store_action(&self, action: MTLStoreAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStoreAction:action];
        }
    }
    
    /// Gets the store action for this attachment.
    #[must_use]
    pub fn store_action(&self) -> MTLStoreAction {
        unsafe {
            msg_send![self.as_ref(), storeAction]
        }
    }
    
    /// Sets the clear depth value for this attachment.
    pub fn set_clear_depth(&self, depth: f64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setClearDepth:depth];
        }
    }
    
    /// Gets the clear depth value for this attachment.
    #[must_use]
    pub fn clear_depth(&self) -> f64 {
        unsafe {
            msg_send![self.as_ref(), clearDepth]
        }
    }

    /// Sets the resolve texture for this attachment.
    pub fn set_resolve_texture(&self, texture: &MTLTexture) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setResolveTexture:texture.as_ptr()];
        }
    }
    
    /// Gets the resolve texture for this attachment.
    #[must_use]
    pub fn resolve_texture(&self) -> Option<MTLTexture> {
        unsafe {
            let texture: *mut Object = msg_send![self.as_ref(), resolveTexture];
            if texture.is_null() {
                None
            } else {
                Some(MTLTexture::from_ptr(texture))
            }
        }
    }

    /// Sets the depth resolve filter.
    pub fn set_depth_resolve_filter(&self, filter: MTLMultisampleDepthResolveFilter) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDepthResolveFilter:filter];
        }
    }

    /// Gets the depth resolve filter.
    #[must_use]
    pub fn depth_resolve_filter(&self) -> MTLMultisampleDepthResolveFilter {
        unsafe {
            msg_send![self.as_ref(), depthResolveFilter]
        }
    }
}

impl MTLRenderPassStencilAttachmentDescriptor {
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
    pub fn set_load_action(&self, action: MTLLoadAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setLoadAction:action];
        }
    }
    
    /// Gets the load action for this attachment.
    #[must_use]
    pub fn load_action(&self) -> MTLLoadAction {
        unsafe {
            msg_send![self.as_ref(), loadAction]
        }
    }
    
    /// Sets the store action for this attachment.
    pub fn set_store_action(&self, action: MTLStoreAction) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStoreAction:action];
        }
    }
    
    /// Gets the store action for this attachment.
    #[must_use]
    pub fn store_action(&self) -> MTLStoreAction {
        unsafe {
            msg_send![self.as_ref(), storeAction]
        }
    }
    
    /// Sets the clear stencil value for this attachment.
    pub fn set_clear_stencil(&self, stencil: u32) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setClearStencil:stencil];
        }
    }
    
    /// Gets the clear stencil value for this attachment.
    #[must_use]
    pub fn clear_stencil(&self) -> u32 {
        unsafe {
            msg_send![self.as_ref(), clearStencil]
        }
    }

    /// Sets the resolve texture for this attachment.
    pub fn set_resolve_texture(&self, texture: &MTLTexture) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setResolveTexture:texture.as_ptr()];
        }
    }
    
    /// Gets the resolve texture for this attachment.
    #[must_use]
    pub fn resolve_texture(&self) -> Option<MTLTexture> {
        unsafe {
            let texture: *mut Object = msg_send![self.as_ref(), resolveTexture];
            if texture.is_null() {
                None
            } else {
                Some(MTLTexture::from_ptr(texture))
            }
        }
    }

    /// Sets the stencil resolve filter.
    pub fn set_stencil_resolve_filter(&self, filter: MTLMultisampleStencilResolveFilter) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStencilResolveFilter:filter];
        }
    }

    /// Gets the stencil resolve filter.
    #[must_use]
    pub fn stencil_resolve_filter(&self) -> MTLMultisampleStencilResolveFilter {
        unsafe {
            msg_send![self.as_ref(), stencilResolveFilter]
        }
    }
}