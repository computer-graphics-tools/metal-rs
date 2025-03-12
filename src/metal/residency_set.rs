// Copyright 2024 The metal-rs contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSUInteger, NSString, NSArray};
use crate::metal::allocation::MTLAllocationRef;
use std::fmt;

/// A reference to an Objective-C `MTLResidencySetDescriptor`.
pub struct MTLResidencySetDescriptorRef(Object);

/// An owned Objective-C `MTLResidencySetDescriptor`.
pub struct MTLResidencySetDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLResidencySetDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLResidencySetDescriptorRef {}
unsafe impl Sync for MTLResidencySetDescriptorRef {}

unsafe impl ForeignType for MTLResidencySetDescriptor {
    type CType = Object;
    type Ref = MTLResidencySetDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLResidencySetDescriptor {
        MTLResidencySetDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLResidencySetDescriptorRef> for MTLResidencySetDescriptor {
    fn as_ref(&self) -> &MTLResidencySetDescriptorRef {
        unsafe { &*(self.0.cast::<MTLResidencySetDescriptorRef>()) }
    }
}

unsafe impl Send for MTLResidencySetDescriptor {}
unsafe impl Sync for MTLResidencySetDescriptor {}

unsafe impl objc::Message for MTLResidencySetDescriptorRef {}

impl MTLResidencySetDescriptor {
    /// Creates a new residency set descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLResidencySetDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLResidencySetDescriptor::from_ptr(obj)
        }
    }
}

impl Default for MTLResidencySetDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLResidencySetDescriptorRef {
    /// Gets the label of the residency set descriptor.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }

    /// Sets the label of the residency set descriptor.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self, setLabel: ns_string.as_ptr()];
        }
    }

    /// Gets the initial capacity of the residency set descriptor.
    #[must_use]
    pub fn initial_capacity(&self) -> NSUInteger {
        unsafe { msg_send![self, initialCapacity] }
    }

    /// Sets the initial capacity of the residency set descriptor.
    pub fn set_initial_capacity(&self, capacity: NSUInteger) {
        unsafe { msg_send![self, setInitialCapacity: capacity] }
    }
}

impl Clone for MTLResidencySetDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLResidencySetDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLResidencySetDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLResidencySetDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLResidencySetDescriptor")
            .field("label", &self.as_ref().label())
            .field("initial_capacity", &self.as_ref().initial_capacity())
            .finish()
    }
}

/// A reference to an Objective-C `MTLResidencySet`.
pub struct MTLResidencySetRef(Object);

/// An owned Objective-C `MTLResidencySet`.
pub struct MTLResidencySet(*mut Object);

unsafe impl ForeignTypeRef for MTLResidencySetRef {
    type CType = Object;
}

unsafe impl Send for MTLResidencySetRef {}
unsafe impl Sync for MTLResidencySetRef {}

unsafe impl ForeignType for MTLResidencySet {
    type CType = Object;
    type Ref = MTLResidencySetRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLResidencySet {
        MTLResidencySet(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLResidencySetRef> for MTLResidencySet {
    fn as_ref(&self) -> &MTLResidencySetRef {
        unsafe { &*(self.0.cast::<MTLResidencySetRef>()) }
    }
}

unsafe impl Send for MTLResidencySet {}
unsafe impl Sync for MTLResidencySet {}

unsafe impl objc::Message for MTLResidencySetRef {}

impl MTLResidencySetRef {
    /// Gets the device that created this residency set.
    #[must_use]
    pub fn device(&self) -> crate::metal::MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self, device];
            crate::metal::MTLDevice::from_ptr(ptr)
        }
    }

    /// Gets the label of the residency set.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }

    /// Gets the allocated size in bytes.
    #[must_use]
    pub fn allocated_size(&self) -> u64 {
        unsafe { msg_send![self, allocatedSize] }
    }

    /// Makes all allocations in the set resident.
    pub fn request_residency(&self) {
        unsafe { msg_send![self, requestResidency] }
    }

    /// Ends residency for all allocations in the set.
    pub fn end_residency(&self) {
        unsafe { msg_send![self, endResidency] }
    }

    /// Adds an allocation to the set.
    pub fn add_allocation(&self, allocation: &MTLAllocationRef) {
        unsafe { msg_send![self, addAllocation: allocation.as_ptr()] }
    }

    /// Adds multiple allocations to the set.
    ///
    /// # Safety
    ///
    /// The `allocations` array must contain valid MTLAllocation pointers, and
    /// `count` must be less than or equal to the length of the array.
    pub unsafe fn add_allocations(&self, allocations: &[*const MTLAllocationRef], count: NSUInteger) {
        let _: () = msg_send![self, addAllocations: allocations.as_ptr() count: count];
    }

    /// Removes an allocation from the set.
    pub fn remove_allocation(&self, allocation: &MTLAllocationRef) {
        unsafe { msg_send![self, removeAllocation: allocation.as_ptr()] }
    }

    /// Removes multiple allocations from the set.
    ///
    /// # Safety
    ///
    /// The `allocations` array must contain valid MTLAllocation pointers, and
    /// `count` must be less than or equal to the length of the array.
    pub unsafe fn remove_allocations(&self, allocations: &[*const MTLAllocationRef], count: NSUInteger) {
        let _: () = msg_send![self, removeAllocations: allocations.as_ptr() count: count];
    }

    /// Removes all allocations from the set.
    pub fn remove_all_allocations(&self) {
        unsafe { msg_send![self, removeAllAllocations] }
    }

    /// Checks if the set contains the given allocation.
    #[must_use]
    pub fn contains_allocation(&self, allocation: &MTLAllocationRef) -> bool {
        unsafe { msg_send![self, containsAllocation: allocation.as_ptr()] }
    }

    /// Gets all allocations in the set.
    #[must_use]
    pub fn all_allocations(&self) -> NSArray {
        unsafe {
            let ptr: *mut Object = msg_send![self, allAllocations];
            NSArray::from_ptr(ptr)
        }
    }

    /// Gets the number of allocations in the set.
    #[must_use]
    pub fn allocation_count(&self) -> NSUInteger {
        unsafe { msg_send![self, allocationCount] }
    }

    /// Commits changes to the residency set.
    pub fn commit(&self) {
        unsafe { msg_send![self, commit] }
    }
}

impl Clone for MTLResidencySet {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLResidencySet::from_ptr(obj)
        }
    }
}

impl Drop for MTLResidencySet {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLResidencySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLResidencySet")
            .field("label", &self.as_ref().label())
            .field("allocated_size", &self.as_ref().allocated_size())
            .field("allocation_count", &self.as_ref().allocation_count())
            .finish()
    }
}