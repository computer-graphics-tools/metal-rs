//! CAMetalLayer - A Rust wrapper around QuartzCore's CAMetalLayer class.
//!
//! This module provides safe Rust bindings to the CAMetalLayer class from Apple's QuartzCore framework.
//! CAMetalLayer is used to create Metal drawables that can be presented onscreen.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::MTLCreateSystemDefaultDevice;
//! use metal_rs::quartzcore::CAMetalLayer;
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a metal layer
//! let layer = CAMetalLayer::new();
//! layer.set_device(&device);
//! 
//! // Get next drawable from the layer
//! let drawable = layer.next_drawable().unwrap();
//! 
//! // Present the drawable
//! drawable.present();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::metal::{MTLDevice, MTLDeviceRef, MTLPixelFormat, MTLDrawableRef};

/// A reference to an Objective-C `CAMetalDrawable`.
pub struct CAMetalDrawableRef(Object);

/// An owned Objective-C `CAMetalDrawable`.
pub struct CAMetalDrawable(*mut Object);

unsafe impl ForeignTypeRef for CAMetalDrawableRef {
    type CType = Object;
}

unsafe impl Send for CAMetalDrawableRef {}
unsafe impl Sync for CAMetalDrawableRef {}

unsafe impl ForeignType for CAMetalDrawable {
    type CType = Object;
    type Ref = CAMetalDrawableRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> CAMetalDrawable {
        CAMetalDrawable(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<CAMetalDrawableRef> for CAMetalDrawable {
    fn as_ref(&self) -> &CAMetalDrawableRef {
        unsafe { &*(self.0.cast::<CAMetalDrawableRef>()) }
    }
}

unsafe impl Send for CAMetalDrawable {}
unsafe impl Sync for CAMetalDrawable {}

unsafe impl objc::Message for CAMetalDrawableRef {}

impl CAMetalDrawable {
    /// Get the texture associated with this drawable.
    #[must_use]
    pub fn texture(&self) -> crate::metal::MTLTexture {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref() as &CAMetalDrawableRef, texture];
            crate::metal::MTLTexture::from_ptr(ptr)
        }
    }

    /// Get the layer associated with this drawable.
    #[must_use]
    pub fn layer(&self) -> &CAMetalLayerRef {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref() as &CAMetalDrawableRef, layer];
            &*(ptr as *mut CAMetalLayerRef)
        }
    }
}

impl fmt::Debug for CAMetalDrawable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CAMetalDrawable {{ texture: {:?} }}", self.texture())
    }
}

impl Drop for CAMetalDrawable {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for CAMetalDrawable {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            CAMetalDrawable::from_ptr(obj)
        }
    }
}

impl AsRef<MTLDrawableRef> for CAMetalDrawableRef {
    fn as_ref(&self) -> &MTLDrawableRef {
        unsafe { &*(self as *const _ as *const MTLDrawableRef) }
    }
}

impl AsRef<MTLDrawableRef> for CAMetalDrawable {
    fn as_ref(&self) -> &MTLDrawableRef {
        unsafe { &*(self.0 as *mut Object as *const MTLDrawableRef) }
    }
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
    /// Creates a new CAMetalLayer.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(CAMetalLayer);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            CAMetalLayer::from_ptr(obj)
        }
    }

    /// Sets the Metal device used by this layer.
    pub fn set_device(&self, device: &MTLDevice) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDevice:device.as_ptr()];
        }
    }

    /// Gets the Metal device used by this layer.
    #[must_use]
    pub fn device(&self) -> Option<&MTLDeviceRef> {
        unsafe {
            let device: *mut Object = msg_send![self.as_ref(), device];
            if device.is_null() {
                None
            } else {
                Some(&*(device as *mut MTLDeviceRef))
            }
        }
    }

    /// Sets the pixel format for this layer.
    pub fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setPixelFormat:pixel_format];
        }
    }

    /// Gets the pixel format for this layer.
    #[must_use]
    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.as_ref(), pixelFormat]
        }
    }

    /// Gets the next drawable from this layer.
    #[must_use]
    pub fn next_drawable(&self) -> Option<CAMetalDrawable> {
        unsafe {
            let drawable: *mut Object = msg_send![self.as_ref(), nextDrawable];
            if drawable.is_null() {
                None
            } else {
                Some(CAMetalDrawable::from_ptr(drawable))
            }
        }
    }

    /// Sets the frame buffer only flag.
    pub fn set_framebuffer_only(&self, framebuffer_only: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setFramebufferOnly:framebuffer_only];
        }
    }

    /// Gets the frame buffer only flag.
    #[must_use]
    pub fn is_framebuffer_only(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), framebufferOnly]
        }
    }

    /// Sets the drawable size.
    pub fn set_drawable_size(&self, width: f64, height: f64) {
        unsafe {
            use core_graphics::geometry::CGSize;
            let size = CGSize::new(width, height);
            let _: () = msg_send![self.as_ref(), setDrawableSize:size];
        }
    }

    /// Gets the drawable size.
    #[must_use]
    pub fn drawable_size(&self) -> (f64, f64) {
        unsafe {
            use core_graphics::geometry::CGSize;
            let size: CGSize = msg_send![self.as_ref(), drawableSize];
            (size.width, size.height)
        }
    }

    /// Sets whether the contents are opaque.
    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setOpaque:opaque];
        }
    }

    /// Gets whether the contents are opaque.
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), isOpaque]
        }
    }

    /// Sets whether presentation is synchronized with vsync.
    pub fn set_presents_with_transaction(&self, presents_with_transaction: bool) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setPresentsWithTransaction:presents_with_transaction];
        }
    }

    /// Gets whether presentation is synchronized with vsync.
    #[must_use]
    pub fn presents_with_transaction(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), presentsWithTransaction]
        }
    }

    /// Sets the maximum number of drawables that can be held by the layer.
    pub fn set_maximum_drawable_count(&self, count: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setMaximumDrawableCount:count];
        }
    }

    /// Gets the maximum number of drawables that can be held by the layer.
    #[must_use]
    pub fn maximum_drawable_count(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), maximumDrawableCount]
        }
    }
}

impl fmt::Debug for CAMetalLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let device = self.device().map_or_else(|| "None".to_string(), |_| "Some(MTLDevice)".to_string());
        let (width, height) = self.drawable_size();
        
        f.debug_struct("CAMetalLayer")
            .field("device", &device)
            .field("pixel_format", &self.pixel_format())
            .field("drawable_size", &format!("({}, {})", width, height))
            .field("framebuffer_only", &self.is_framebuffer_only())
            .field("opaque", &self.is_opaque())
            .field("presents_with_transaction", &self.presents_with_transaction())
            .field("maximum_drawable_count", &self.maximum_drawable_count())
            .finish()
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