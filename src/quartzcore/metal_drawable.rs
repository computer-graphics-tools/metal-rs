//! CAMetalDrawable - A Rust wrapper around QuartzCore's CAMetalDrawable class.
//!
//! This module provides safe Rust bindings to the CAMetalDrawable class from Apple's QuartzCore framework.
//! CAMetalDrawable represents a Metal texture that can be rendered to and presented on screen.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::quartzcore::CAMetalLayer;
//!
//! // Create a metal layer
//! let layer = CAMetalLayer::new();
//! 
//! // In a real app, this would return a drawable
//! if let Some(drawable) = layer.next_drawable() {
//!     // Get the texture to render to (this would require a proper Metal device)
//!     let texture = drawable.texture();
//!     
//!     // After rendering, present the drawable
//!     drawable.present();
//! }
//! ```

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {
    // This is empty, but links with the QuartzCore framework
}

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
// For placeholder metal types
use crate::quartzcore::metal_layer::CAMetalLayer;

/// A reference to an Objective-C `CAMetalDrawable`.
pub struct CAMetalDrawableRef(Object);

/// An owned Objective-C `CAMetalDrawable`.
pub struct CAMetalDrawable(*mut Object);

// Dummy types for MTL - these will be properly implemented later
pub struct MTLTextureRef(Object);
pub struct MTLTexture(*mut Object);

// Implementation for MTLTexture placeholders
unsafe impl ForeignTypeRef for MTLTextureRef {
    type CType = Object;
}

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
    /// Returns the metal layer that created this drawable.
    #[must_use]
    pub fn layer(&self) -> CAMetalLayer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), layer];
            CAMetalLayer::from_ptr(ptr)
        }
    }

    /// Returns the metal texture that can be rendered to.
    #[must_use]
    pub fn texture(&self) -> MTLTexture {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), texture];
            MTLTexture::from_ptr(ptr)
        }
    }
    
    /// Presents the drawable as soon as possible.
    pub fn present(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), present];
        }
    }
    
    /// Presents the drawable at the specified time.
    pub fn present_at_time(&self, present_time: f64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), presentAtTime:present_time];
        }
    }
}

impl fmt::Debug for CAMetalDrawable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CAMetalDrawable")
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