//! NSAutoreleasePool - A mechanism for deferring the release of objects.

use crate::foreign_obj_type;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL};
use std::ops::{Deref, DerefMut, Drop};

/// A reference to an Objective-C `NSAutoreleasePool`.
pub enum NSAutoreleasePoolRef {}

/// An owned Objective-C `NSAutoreleasePool`.
pub struct NSAutoreleasePool(foreign_types::ForeignObjectRef<NSAutoreleasePoolRef>);

foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSAutoreleasePool;
    pub struct NSAutoreleasePoolRef;
    type ParentType = super::object::NSObjectRef;
}

unsafe impl objc::Message for NSAutoreleasePoolRef {}

impl NSAutoreleasePool {
    /// Creates a new autorelease pool.
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSAutoreleasePool);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            NSAutoreleasePool::from_ptr(obj)
        }
    }

    /// Manually drains the autorelease pool.
    pub fn drain(&mut self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), drain];
        }
    }
}

impl Default for NSAutoreleasePool {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for managing an autorelease pool within a scope.
pub struct AutoreleasePool {
    pool: NSAutoreleasePool,
}

impl AutoreleasePool {
    /// Creates a new autorelease pool.
    pub fn new() -> Self {
        AutoreleasePool {
            pool: NSAutoreleasePool::new(),
        }
    }
}

impl Default for AutoreleasePool {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for AutoreleasePool {
    type Target = NSAutoreleasePool;
    
    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for AutoreleasePool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        self.pool.drain();
    }
}