//! MTLDrawable - A Rust wrapper around Metal's MTLDrawable protocol.
//!
//! This module provides safe Rust bindings to the MTLDrawable protocol from Apple's Metal framework.
//! MTLDrawable represents an object that can be rendered to and presented onscreen.
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
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use core_foundation::date::CFTimeInterval;
use crate::foundation::NSUInteger;

/// A reference to an Objective-C `MTLDrawable`.
pub struct MTLDrawableRef(Object);

/// An owned Objective-C `MTLDrawable`.
pub struct MTLDrawable(*mut Object);

unsafe impl ForeignTypeRef for MTLDrawableRef {
    type CType = Object;
}

unsafe impl Send for MTLDrawableRef {}
unsafe impl Sync for MTLDrawableRef {}

unsafe impl ForeignType for MTLDrawable {
    type CType = Object;
    type Ref = MTLDrawableRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLDrawable {
        MTLDrawable(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLDrawableRef> for MTLDrawable {
    fn as_ref(&self) -> &MTLDrawableRef {
        unsafe { &*(self.0.cast::<MTLDrawableRef>()) }
    }
}

unsafe impl Send for MTLDrawable {}
unsafe impl Sync for MTLDrawable {}

unsafe impl objc::Message for MTLDrawableRef {}

impl MTLDrawable {
    /// Present this drawable.
    pub fn present(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), present];
        }
    }

    /// Present this drawable at the specified time.
    ///
    /// # Parameters
    ///
    /// * `presentation_time` - The host time at which to display the drawable.
    pub fn present_at_time(&self, presentation_time: CFTimeInterval) {
        unsafe {
            let _: () = msg_send![self.as_ref(), presentAtTime:presentation_time];
        }
    }

    /// Present this drawable after the specified duration.
    ///
    /// # Parameters
    ///
    /// * `duration` - The minimum duration in seconds from now to display the drawable.
    pub fn present_after_minimum_duration(&self, duration: CFTimeInterval) {
        unsafe {
            let _: () = msg_send![self.as_ref(), presentAfterMinimumDuration:duration];
        }
    }

    /// The time when the drawable was presented.
    ///
    /// Returns 0 if the drawable has not been presented.
    #[must_use]
    pub fn presented_time(&self) -> CFTimeInterval {
        unsafe {
            msg_send![self.as_ref(), presentedTime]
        }
    }

    /// The unique identifier for this drawable.
    #[must_use]
    pub fn drawable_id(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), drawableID]
        }
    }
}

impl fmt::Debug for MTLDrawable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLDrawable {{ drawable_id: {}, presented_time: {} }}", 
               self.drawable_id(), self.presented_time())
    }
}

impl Drop for MTLDrawable {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLDrawable {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLDrawable::from_ptr(obj)
        }
    }
}