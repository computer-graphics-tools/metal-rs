//! CAMetalLayer - A Rust wrapper around QuartzCore's CAMetalLayer class.
//!
//! This module provides safe Rust bindings to the CAMetalLayer class from Apple's QuartzCore framework.
//! CAMetalLayer is used to create a Metal-compatible rendering layer that can be added to a view hierarchy.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::quartzcore::{CAMetalLayer, CGSize, MTLPixelFormat};
//!
//! // Create a metal layer
//! let layer = CAMetalLayer::new();
//! 
//! // Configure the layer
//! layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
//! layer.set_framebuffer_only(true);
//! 
//! // Set the drawable size
//! let size = CGSize::new(800.0, 600.0);
//! layer.set_drawable_size(size);
//! ```

use std::fmt;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::quartzcore::metal_drawable::CAMetalDrawable;

/// CGSize from CoreGraphics, we'll define it here for simplicity
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGSize {
    pub width: f64,
    pub height: f64,
}

impl CGSize {
    #[must_use]
    pub fn new(width: f64, height: f64) -> Self {
        CGSize { width, height }
    }
}

// Dummy types for MTL - these will be replaced with actual imports later
pub struct MTLDeviceRef(Object);
pub struct MTLDevice(*mut Object);

// Implementation of placeholder MTLDevice
unsafe impl ForeignTypeRef for MTLDeviceRef {
    type CType = Object;
}

unsafe impl ForeignType for MTLDevice {
    type CType = Object;
    type Ref = MTLDeviceRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLDevice {
        MTLDevice(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

// MTLPixelFormat - using an enum with number representation to match Metal's API
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPixelFormat {
    Invalid = 0,
    RGBA8Unorm = 70,
    BGRA8Unorm = 80,
}

/// A reference to an Objective-C `CAMetalLayer`.
pub struct CAMetalLayerRef(Object);

/// An owned Objective-C `CAMetalLayer`.
pub struct CAMetalLayer(*mut Object);

unsafe impl ForeignTypeRef for CAMetalLayerRef {
    type CType = Object;
}

unsafe impl Send for CAMetalLayerRef {}
unsafe impl Sync for CAMetalLayerRef {}

unsafe impl ForeignType for CAMetalLayer {
    type CType = Object;
    type Ref = CAMetalLayerRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> CAMetalLayer {
        CAMetalLayer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<CAMetalLayerRef> for CAMetalLayer {
    fn as_ref(&self) -> &CAMetalLayerRef {
        unsafe { &*(self.0.cast::<CAMetalLayerRef>()) }
    }
}

unsafe impl Send for CAMetalLayer {}
unsafe impl Sync for CAMetalLayer {}

unsafe impl objc::Message for CAMetalLayerRef {}

impl CAMetalLayer {
    /// Creates a new `CAMetalLayer` instance.
    #[must_use]
    pub fn new() -> Self {
        Self::layer()
    }

    /// Creates a new `CAMetalLayer`.
    #[must_use]
    pub fn layer() -> Self {
        unsafe {
            let cls = class!(CAMetalLayer);
            let obj: *mut Object = msg_send![cls, layer];
            CAMetalLayer::from_ptr(obj)
        }
    }

    /// Returns the Metal device used by the layer.
    pub fn device(&self) -> MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            MTLDevice::from_ptr(ptr)
        }
    }

    /// Sets the Metal device to be used by the layer.
    pub fn set_device(&self, device: &MTLDevice) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDevice:device.as_ptr()];
        }
    }

    /// Returns the pixel format of textures created by the layer.
    #[must_use]
    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            let format: MTLPixelFormat = msg_send![self.as_ref(), pixelFormat];
            format
        }
    }

    /// Sets the pixel format of textures created by the layer.
    pub fn set_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setPixelFormat:format];
        }
    }

    /// Returns whether the layer's textures are only for framebuffer use.
    #[must_use]
    pub fn framebuffer_only(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), framebufferOnly]
        }
    }

    /// Sets whether the layer's textures are only for framebuffer use.
    pub fn set_framebuffer_only(&self, framebuffer_only: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFramebufferOnly:framebuffer_only];
        }
    }

    /// Returns the drawable size of the layer.
    #[must_use]
    pub fn drawable_size(&self) -> CGSize {
        unsafe {
            msg_send![self.as_ref(), drawableSize]
        }
    }

    /// Sets the drawable size of the layer.
    pub fn set_drawable_size(&self, size: CGSize) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDrawableSize:size];
        }
    }

    /// Returns the next drawable for the layer.
    #[must_use]
    pub fn next_drawable(&self) -> Option<CAMetalDrawable> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), nextDrawable];
            if ptr.is_null() {
                None
            } else {
                Some(CAMetalDrawable::from_ptr(ptr))
            }
        }
    }
}

impl fmt::Debug for CAMetalLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.drawable_size();
        write!(f, "CAMetalLayer {{ size: {}x{} }}", size.width, size.height)
    }
}

impl Drop for CAMetalLayer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for CAMetalLayer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            CAMetalLayer::from_ptr(obj)
        }
    }
}

impl Default for CAMetalLayer {
    fn default() -> Self {
        Self::new()
    }
}