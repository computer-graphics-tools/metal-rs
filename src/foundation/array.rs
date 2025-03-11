//! NSArray - A Rust wrapper around Foundation's NSArray class.
//!
//! This module provides safe Rust bindings to the NSArray class from Apple's Foundation framework.
//! NSArray is an ordered collection of objects.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::foundation::NSArray;
//!
//! // Create an array from a slice
//! let strings = vec!["Hello", "World"];
//! let ns_strings: Vec<_> = strings.iter().map(|s| metal_rs::foundation::NSString::from_rust_str(s)).collect();
//! let array = NSArray::from_refs_slice(&ns_strings);
//! 
//! // Get the count
//! let count = array.count();
//! 
//! // Get an object at index
//! if count > 0 {
//!     let obj = array.object_at(0);
//! }
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSUInteger;

/// A reference to an Objective-C `NSArray`.
pub struct NSArrayRef(Object);

/// An owned Objective-C `NSArray`.
pub struct NSArray(*mut Object);

unsafe impl ForeignTypeRef for NSArrayRef {
    type CType = Object;
}

unsafe impl Send for NSArrayRef {}
unsafe impl Sync for NSArrayRef {}

unsafe impl ForeignType for NSArray {
    type CType = Object;
    type Ref = NSArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> NSArray {
        NSArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<NSArrayRef> for NSArray {
    fn as_ref(&self) -> &NSArrayRef {
        unsafe { &*(self.0.cast::<NSArrayRef>()) }
    }
}

unsafe impl Send for NSArray {}
unsafe impl Sync for NSArray {}

unsafe impl objc::Message for NSArrayRef {}

impl NSArray {
    /// Creates a new empty array.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSArray);
            let obj: *mut Object = msg_send![class, array];
            NSArray::from_ptr(obj)
        }
    }

    /// Creates a new array from a slice of objects.
    #[must_use]
    pub fn from_slice<T: ForeignTypeRef>(objects: &[&T]) -> Self 
    where T::CType: objc::Message {
        unsafe {
            let count = objects.len();
            let class = class!(NSArray);
            let ptrs: Vec<*const Object> = objects.iter().map(|obj| obj.as_ptr() as *const Object).collect();
            let obj: *mut Object = msg_send![class, arrayWithObjects:ptrs.as_ptr() count:count];
            NSArray::from_ptr(obj)
        }
    }

    /// Creates a new array from a slice of object references that implement AsRef<T>.
    #[must_use]
    pub fn from_refs_slice<T, U>(objects: &[T]) -> Self 
    where 
        T: AsRef<U>,
        U: ForeignTypeRef,
        U::CType: objc::Message 
    {
        unsafe {
            let count = objects.len();
            let class = class!(NSArray);
            let ptrs: Vec<*const Object> = objects.iter().map(|obj| obj.as_ref().as_ptr() as *const Object).collect();
            let obj: *mut Object = msg_send![class, arrayWithObjects:ptrs.as_ptr() count:count];
            NSArray::from_ptr(obj)
        }
    }

    /// Gets the number of objects in the array.
    #[must_use]
    pub fn count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), count]
        }
    }

    /// Gets an object at the specified index.
    #[must_use]
    pub fn object_at(&self, index: NSUInteger) -> *mut Object {
        unsafe {
            msg_send![self.as_ref(), objectAtIndex:index]
        }
    }

    /// Gets an object at the specified index as a specific type.
    #[must_use]
    pub fn object_at_as<T: ForeignType>(&self, index: NSUInteger) -> T 
    where T::CType: objc::Message {
        unsafe {
            let obj: *mut Object = msg_send![self.as_ref(), objectAtIndex:index];
            T::from_ptr(obj as *mut T::CType)
        }
    }

    /// Returns whether the array contains the given object.
    #[must_use]
    pub fn contains_object<T>(&self, object: &T) -> bool 
    where 
        T: ForeignTypeRef,
        T::CType: objc::Message 
    {
        unsafe {
            let result: bool = msg_send![self.as_ref(), containsObject:object.as_ptr()];
            result
        }
    }

    /// Returns the first index of the given object.
    #[must_use]
    pub fn index_of_object<T>(&self, object: &T) -> NSUInteger 
    where 
        T: ForeignTypeRef,
        T::CType: objc::Message 
    {
        unsafe {
            msg_send![self.as_ref(), indexOfObject:object.as_ptr()]
        }
    }
}

impl Default for NSArray {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSArray")
            .field("count", &self.count())
            .finish()
    }
}

impl Drop for NSArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for NSArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            NSArray::from_ptr(obj)
        }
    }
}