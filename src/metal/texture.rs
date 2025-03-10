//! MTLTexture - A Rust wrapper around Metal's MTLTexture class.
//!
//! This module provides safe Rust bindings to the MTLTexture class from Apple's Metal framework.
//! MTLTexture represents a texture resource that can be used for rendering, compute, or storage.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLTextureDescriptor, MTLPixelFormat, MTLTextureUsage};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a texture descriptor
//! let texture_descriptor = MTLTextureDescriptor::new();
//! texture_descriptor.set_width(1024);
//! texture_descriptor.set_height(1024);
//! texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
//! texture_descriptor.set_usage(MTLTextureUsage::RenderTarget);
//! 
//! // Create the texture
//! let texture = device.new_texture(&texture_descriptor);
//! texture.set_label("My Texture");
//! ```

use std::fmt;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::types::MTLPixelFormat;
use crate::metal::MTLResourceRef;

/// Describes the dimensionality of a texture.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLTextureType {
    /// A 1D texture.
    Type1D = 0,
    
    /// A 1D texture with mipmaps.
    Type1DArray = 1,
    
    /// A 2D texture.
    Type2D = 2,
    
    /// A 2D texture with mipmaps.
    Type2DArray = 3,
    
    /// A 2D texture with multisampling.
    Type2DMultisample = 4,
    
    /// A cube texture.
    TypeCube = 5,
    
    /// A cube texture with mipmaps.
    TypeCubeArray = 6,
    
    /// A 3D texture.
    Type3D = 7,
    
    /// A 2D texture with multisampling and mipmaps.
    Type2DMultisampleArray = 8,
    
    /// A texture buffer.
    TypeTextureBuffer = 9,
}

/// Describes how a texture will be used.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLTextureUsage {
    /// The texture will be used with a shader that reads from the texture.
    ShaderRead = 1 << 0,
    
    /// The texture will be used with a shader that writes to the texture.
    ShaderWrite = 1 << 1,
    
    /// The texture will be used as a render target.
    RenderTarget = 1 << 2,
    
    /// The texture will be used as a pixel format conversion target.
    PixelFormatView = 1 << 4,
}

impl std::ops::BitOr for MTLTextureUsage {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self as u64 | rhs as u64) }
    }
}

/// A reference to an Objective-C `MTLTexture`.
pub struct MTLTextureRef(Object);

/// An owned Objective-C `MTLTexture`.
pub struct MTLTexture(*mut Object);

unsafe impl ForeignTypeRef for MTLTextureRef {
    type CType = Object;
}

unsafe impl Send for MTLTextureRef {}
unsafe impl Sync for MTLTextureRef {}

unsafe impl ForeignType for MTLTexture {
    type CType = Object;
    type Ref = MTLTextureRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLTexture {
        MTLTexture(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLTextureRef> for MTLTexture {
    fn as_ref(&self) -> &MTLTextureRef {
        unsafe { &*(self.0.cast::<MTLTextureRef>()) }
    }
}

impl AsRef<MTLResourceRef> for MTLTextureRef {
    fn as_ref(&self) -> &MTLResourceRef {
        unsafe { &*(self as *const MTLTextureRef as *const MTLResourceRef) }
    }
}

impl AsRef<MTLResourceRef> for MTLTexture {
    fn as_ref(&self) -> &MTLResourceRef {
        let texture_ref: &MTLTextureRef = AsRef::<MTLTextureRef>::as_ref(self);
        AsRef::<MTLResourceRef>::as_ref(texture_ref)
    }
}

unsafe impl Send for MTLTexture {}
unsafe impl Sync for MTLTexture {}

unsafe impl objc::Message for MTLTextureRef {}

impl MTLTexture {
    /// Returns the label of the texture.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            let label: *mut Object = msg_send![texture_ref, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Sets the label of the texture.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let texture_ref: &MTLTextureRef = self.as_ref();
            let _: () = msg_send![texture_ref, setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Returns the width of the texture.
    #[must_use]
    pub fn width(&self) -> usize {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, width]
        }
    }
    
    /// Returns the height of the texture.
    #[must_use]
    pub fn height(&self) -> usize {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, height]
        }
    }
    
    /// Returns the depth of the texture.
    #[must_use]
    pub fn depth(&self) -> usize {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, depth]
        }
    }
    
    /// Returns the pixel format of the texture.
    #[must_use]
    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, pixelFormat]
        }
    }
    
    /// Returns the type of the texture.
    #[must_use]
    pub fn texture_type(&self) -> MTLTextureType {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, textureType]
        }
    }
    
    /// Returns the number of mipmap levels in the texture.
    #[must_use]
    pub fn mipmap_level_count(&self) -> usize {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, mipmapLevelCount]
        }
    }
    
    /// Returns the number of samples per pixel in the texture.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, sampleCount]
        }
    }
    
    /// Returns the usage of the texture.
    #[must_use]
    pub fn usage(&self) -> MTLTextureUsage {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            msg_send![texture_ref, usage]
        }
    }
    
    /// Replaces a region of the texture with data from memory.
    pub fn replace_region(
        &self,
        region: MTLRegion,
        mipmap_level: usize,
        data: &[u8],
        bytes_per_row: usize
    ) {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            let _: () = msg_send![texture_ref, replaceRegion:region
                                                mipmapLevel:mipmap_level
                                                  withBytes:data.as_ptr()
                                                bytesPerRow:bytes_per_row];
        }
    }
    
    /// Gets a region of the texture and copies it into memory.
    pub fn get_bytes(
        &self,
        dest: &mut [u8],
        bytes_per_row: usize,
        bytes_per_image: usize,
        region: MTLRegion,
        mipmap_level: usize,
        slice: usize
    ) {
        unsafe {
            let texture_ref: &MTLTextureRef = self.as_ref();
            let _: () = msg_send![texture_ref, getBytes:dest.as_mut_ptr()
                                               bytesPerRow:bytes_per_row
                                             bytesPerImage:bytes_per_image
                                                fromRegion:region
                                               mipmapLevel:mipmap_level
                                                     slice:slice];
        }
    }
    
    // MTLResource protocol methods
    
    /// Returns the CPU cache mode of the texture.
    #[must_use]
    pub fn cpu_cache_mode(&self) -> crate::metal::MTLCPUCacheMode {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            msg_send![resource_ref, cpuCacheMode]
        }
    }
    
    /// Returns the storage mode of the texture.
    #[must_use]
    pub fn storage_mode(&self) -> crate::metal::MTLStorageMode {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            msg_send![resource_ref, storageMode]
        }
    }
    
    /// Returns the hazard tracking mode of the texture.
    #[must_use]
    pub fn hazard_tracking_mode(&self) -> crate::metal::MTLHazardTrackingMode {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            msg_send![resource_ref, hazardTrackingMode]
        }
    }
    
    /// Returns the resource options of the texture.
    #[must_use]
    pub fn resource_options(&self) -> crate::metal::MTLResourceOptions {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            msg_send![resource_ref, resourceOptions]
        }
    }
    
    /// Returns the allocated size of the texture in bytes.
    #[must_use]
    pub fn allocated_size(&self) -> usize {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            msg_send![resource_ref, allocatedSize]
        }
    }
    
    /// Sets the purgeable state of the texture.
    ///
    /// Returns the previous purgeable state.
    pub fn set_purgeable_state(&self, state: crate::metal::MTLPurgeableState) -> crate::metal::MTLPurgeableState {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            msg_send![resource_ref, setPurgeableState:state]
        }
    }
    
    /// Makes the texture aliasable.
    pub fn make_aliasable(&self) {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            let _: () = msg_send![resource_ref, makeAliasable];
        }
    }
    
    /// Returns whether the texture is aliasable.
    #[must_use]
    pub fn is_aliasable(&self) -> bool {
        let resource_ref: &crate::metal::MTLResourceRef = self.as_ref();
        unsafe {
            msg_send![resource_ref, isAliasable]
        }
    }
}

/// A structure that defines a three-dimensional region in the texture.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRegion {
    /// The origin of the region.
    pub origin: MTLOrigin,
    /// The size of the region.
    pub size: MTLSize,
}

impl MTLRegion {
    /// Creates a new region with the given origin and size.
    #[must_use]
    pub fn new(origin: MTLOrigin, size: MTLSize) -> Self {
        MTLRegion { origin, size }
    }
    
    /// Creates a new region with origin (0,0,0) and the given size.
    #[must_use]
    pub fn from_size(width: usize, height: usize, depth: usize) -> Self {
        MTLRegion {
            origin: MTLOrigin::new(0, 0, 0),
            size: MTLSize::new(width, height, depth),
        }
    }
    
    /// Creates a new 2D region with origin (0,0) and the given size.
    #[must_use]
    pub fn from_2d_size(width: usize, height: usize) -> Self {
        MTLRegion {
            origin: MTLOrigin::new(0, 0, 0),
            size: MTLSize::new(width, height, 1),
        }
    }
}

/// A structure that defines a three-dimensional point in the texture.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLOrigin {
    /// The x-coordinate of the origin.
    pub x: usize,
    /// The y-coordinate of the origin.
    pub y: usize,
    /// The z-coordinate of the origin.
    pub z: usize,
}

impl MTLOrigin {
    /// Creates a new origin with the given coordinates.
    #[must_use]
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        MTLOrigin { x, y, z }
    }
}

/// A structure that defines a three-dimensional size.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSize {
    /// The width of the size.
    pub width: usize,
    /// The height of the size.
    pub height: usize,
    /// The depth of the size.
    pub depth: usize,
}

impl MTLSize {
    /// Creates a new size with the given dimensions.
    #[must_use]
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        MTLSize { width, height, depth }
    }
}

impl fmt::Debug for MTLTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLTexture {{ label: {}, width: {}, height: {}, format: {:?} }}", 
               label, self.width(), self.height(), self.pixel_format())
    }
}

impl Drop for MTLTexture {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLTexture {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLTexture::from_ptr(obj)
        }
    }
}

/// A descriptor that specifies properties for creating a texture.
pub struct MTLTextureDescriptorRef(Object);

/// An owned descriptor that specifies properties for creating a texture.
pub struct MTLTextureDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLTextureDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLTextureDescriptorRef {}
unsafe impl Sync for MTLTextureDescriptorRef {}

unsafe impl ForeignType for MTLTextureDescriptor {
    type CType = Object;
    type Ref = MTLTextureDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLTextureDescriptor {
        MTLTextureDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLTextureDescriptorRef> for MTLTextureDescriptor {
    fn as_ref(&self) -> &MTLTextureDescriptorRef {
        unsafe { &*(self.0.cast::<MTLTextureDescriptorRef>()) }
    }
}

unsafe impl Send for MTLTextureDescriptor {}
unsafe impl Sync for MTLTextureDescriptor {}

unsafe impl objc::Message for MTLTextureDescriptorRef {}

impl MTLTextureDescriptor {
    /// Creates a new texture descriptor with default values.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLTextureDescriptor);
            let obj: *mut Object = msg_send![class, new];
            MTLTextureDescriptor::from_ptr(obj)
        }
    }
    
    /// Sets the width of the texture.
    pub fn set_width(&self, width: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setWidth:width];
        }
    }
    
    /// Sets the height of the texture.
    pub fn set_height(&self, height: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setHeight:height];
        }
    }
    
    /// Sets the depth of the texture.
    pub fn set_depth(&self, depth: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDepth:depth];
        }
    }
    
    /// Sets the pixel format of the texture.
    pub fn set_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setPixelFormat:format];
        }
    }
    
    /// Sets the type of the texture.
    pub fn set_texture_type(&self, texture_type: MTLTextureType) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setTextureType:texture_type];
        }
    }
    
    /// Sets the usage of the texture.
    pub fn set_usage(&self, usage: MTLTextureUsage) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setUsage:usage];
        }
    }
    
    /// Sets the resource options of the texture.
    pub fn set_resource_options(&self, options: crate::metal::MTLResourceOptions) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setResourceOptions:options];
        }
    }
    
    /// Sets the sample count of the texture.
    pub fn set_sample_count(&self, count: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setSampleCount:count];
        }
    }
    
    /// Sets the mipmap level count of the texture.
    pub fn set_mipmap_level_count(&self, count: usize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMipmapLevelCount:count];
        }
    }
    
    /// Sets the storage mode of the texture.
    pub fn set_storage_mode(&self, mode: crate::metal::MTLStorageMode) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStorageMode:mode];
        }
    }
    
    /// Sets the CPU cache mode of the texture.
    pub fn set_cpu_cache_mode(&self, mode: crate::metal::MTLCPUCacheMode) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setCpuCacheMode:mode];
        }
    }
    
    /// Sets the hazard tracking mode of the texture.
    pub fn set_hazard_tracking_mode(&self, mode: crate::metal::MTLHazardTrackingMode) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setHazardTrackingMode:mode];
        }
    }
}

impl Default for MTLTextureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for MTLTextureDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLTextureDescriptor")
    }
}

impl Drop for MTLTextureDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLTextureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLTextureDescriptor::from_ptr(obj)
        }
    }
}