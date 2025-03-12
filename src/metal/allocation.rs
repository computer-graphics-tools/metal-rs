// Copyright 2024 The metal-rs contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSUInteger;
use std::fmt;

/// A reference to an Objective-C `MTLAllocation`.
pub struct MTLAllocationRef(Object);

/// An owned Objective-C `MTLAllocation`.
pub struct MTLAllocation(*mut Object);

unsafe impl ForeignTypeRef for MTLAllocationRef {
    type CType = Object;
}

unsafe impl Send for MTLAllocationRef {}
unsafe impl Sync for MTLAllocationRef {}

unsafe impl ForeignType for MTLAllocation {
    type CType = Object;
    type Ref = MTLAllocationRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLAllocation {
        MTLAllocation(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLAllocationRef> for MTLAllocation {
    fn as_ref(&self) -> &MTLAllocationRef {
        unsafe { &*(self.0.cast::<MTLAllocationRef>()) }
    }
}

unsafe impl Send for MTLAllocation {}
unsafe impl Sync for MTLAllocation {}

unsafe impl objc::Message for MTLAllocationRef {}

impl MTLAllocationRef {
    /// Returns the allocated size in bytes.
    #[must_use]
    pub fn allocated_size(&self) -> NSUInteger {
        unsafe { msg_send![self, allocatedSize] }
    }
}

impl Clone for MTLAllocation {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLAllocation::from_ptr(obj)
        }
    }
}

impl Drop for MTLAllocation {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLAllocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLAllocation")
            .field("allocated_size", &self.as_ref().allocated_size())
            .finish()
    }
}