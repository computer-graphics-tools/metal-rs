//! NSArray - a static ordered collection of objects.

use crate::foreign_obj_type;
use crate::foundation::object::{NSObject, NSObjectRef};
use crate::foundation::types::NSUInteger;
use crate::msg_send_bool;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Index;

/// A reference to an Objective-C `NSArray`.
pub enum NSArrayRef {}

/// An owned Objective-C `NSArray`.
pub struct NSArray(foreign_types::ForeignObjectRef<NSArrayRef>);

foreign_types::foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSArray;
    pub struct NSArrayRef;
}

unsafe impl objc::Message for NSArrayRef {}

/// A reference to an Objective-C `NSMutableArray`.
pub enum NSMutableArrayRef {}

/// An owned Objective-C `NSMutableArray`.
pub struct NSMutableArray(foreign_types::ForeignObjectRef<NSMutableArrayRef>);

foreign_types::foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSMutableArray;
    pub struct NSMutableArrayRef;
}

unsafe impl objc::Message for NSMutableArrayRef {}

impl NSArray {
    /// Creates an empty array.
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSArray);
            let obj: *mut Object = msg_send![class, array];
            NSArray::from_ptr(obj)
        }
    }

    /// Creates an array with the given objects.
    pub fn from_slice<T: AsRef<NSObjectRef>>(objects: &[T]) -> Self {
        let count = objects.len();
        
        // Empty array
        if count == 0 {
            return Self::new();
        }
        
        // Create a temporary Vec of pointers
        let mut ptr_array = Vec::with_capacity(count);
        for obj in objects {
            ptr_array.push(obj.as_ref() as *const NSObjectRef);
        }
        
        unsafe {
            let class = class!(NSArray);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithObjects: ptr_array.as_ptr() count: count];
            NSArray::from_ptr(obj)
        }
    }
    
    /// Creates an NSArray from a raw pointer.
    pub fn from_ptr(ptr: *mut Object) -> Self {
        unsafe { Self::from_raw(ptr) }
    }

    /// Returns the number of objects in the array.
    pub fn count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), count]
        }
    }

    /// Returns whether the array is empty.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Returns the object at the given index.
    pub fn object_at(&self, index: NSUInteger) -> Option<&NSObjectRef> {
        if index >= self.count() {
            return None;
        }
        
        unsafe {
            let obj: *mut Object = msg_send![self.as_ref(), objectAtIndex:, index];
            if obj.is_null() {
                None
            } else {
                Some(&*(obj as *const NSObjectRef))
            }
        }
    }

    /// Returns whether the array contains the given object.
    pub fn contains_object(&self, object: &NSObjectRef) -> bool {
        unsafe {
            msg_send_bool!(self.as_ref(), containsObject:, object)
        }
    }

    /// Returns the index of the given object, or None if it's not found.
    pub fn index_of_object(&self, object: &NSObjectRef) -> Option<NSUInteger> {
        unsafe {
            let index: NSUInteger = msg_send![self.as_ref(), indexOfObject:, object];
            if index == NSUInteger::MAX {
                None
            } else {
                Some(index)
            }
        }
    }

    /// Returns the first object in the array, or None if the array is empty.
    pub fn first_object(&self) -> Option<&NSObjectRef> {
        unsafe {
            let obj: *mut Object = msg_send![self.as_ref(), firstObject];
            if obj.is_null() {
                None
            } else {
                Some(&*(obj as *const NSObjectRef))
            }
        }
    }

    /// Returns the last object in the array, or None if the array is empty.
    pub fn last_object(&self) -> Option<&NSObjectRef> {
        unsafe {
            let obj: *mut Object = msg_send![self.as_ref(), lastObject];
            if obj.is_null() {
                None
            } else {
                Some(&*(obj as *const NSObjectRef))
            }
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
        unsafe {
            let description: *mut Object = msg_send![self.as_ref(), description];
            let nsstring = super::NSString::from_ptr(description);
            write!(f, "{}", nsstring.to_string())
        }
    }
}

impl<'a> IntoIterator for &'a NSArray {
    type Item = &'a NSObjectRef;
    type IntoIter = NSArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NSArrayIterator {
            array: self,
            index: 0,
            count: self.count(),
        }
    }
}

/// Iterator over NSArray elements.
pub struct NSArrayIterator<'a> {
    array: &'a NSArray,
    index: NSUInteger,
    count: NSUInteger,
}

impl<'a> Iterator for NSArrayIterator<'a> {
    type Item = &'a NSObjectRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        
        let item = self.array.object_at(self.index);
        self.index += 1;
        item
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.count - self.index) as usize;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for NSArrayIterator<'a> {}

impl NSMutableArray {
    /// Creates a new mutable array.
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSMutableArray);
            let obj: *mut Object = msg_send![class, array];
            NSMutableArray::from_ptr(obj)
        }
    }
    
    /// Creates a mutable array with the given objects.
    pub fn from_slice<T: AsRef<NSObjectRef>>(objects: &[T]) -> Self {
        let count = objects.len();
        
        // Empty array
        if count == 0 {
            return Self::new();
        }
        
        // Create a temporary Vec of pointers
        let mut ptr_array = Vec::with_capacity(count);
        for obj in objects {
            ptr_array.push(obj.as_ref() as *const NSObjectRef);
        }
        
        unsafe {
            let class = class!(NSMutableArray);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithObjects:count:, 
                                           ptr_array.as_ptr(), 
                                           count];
            NSMutableArray::from_ptr(obj)
        }
    }
    
    /// Creates a new mutable array with a capacity.
    pub fn with_capacity(capacity: NSUInteger) -> Self {
        unsafe {
            let class = class!(NSMutableArray);
            let obj: *mut Object = msg_send![class, arrayWithCapacity:, capacity];
            NSMutableArray::from_ptr(obj)
        }
    }
    
    /// Adds an object to the end of the array.
    pub fn add_object(&self, object: &NSObjectRef) {
        unsafe {
            msg_send![self.as_ref(), addObject:, object]
        }
    }
    
    /// Inserts an object at the given index.
    pub fn insert_object_at(&self, object: &NSObjectRef, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), insertObject:atIndex:, object, index]
        }
    }
    
    /// Removes the object at the given index.
    pub fn remove_object_at(&self, index: NSUInteger) {
        unsafe {
            msg_send![self.as_ref(), removeObjectAtIndex:, index]
        }
    }
    
    /// Removes the given object from the array.
    pub fn remove_object(&self, object: &NSObjectRef) {
        unsafe {
            msg_send![self.as_ref(), removeObject:, object]
        }
    }
    
    /// Removes all objects from the array.
    pub fn remove_all_objects(&self) {
        unsafe {
            msg_send![self.as_ref(), removeAllObjects]
        }
    }
    
    /// Replaces the object at the given index with another object.
    pub fn replace_object_at(&self, index: NSUInteger, object: &NSObjectRef) {
        unsafe {
            msg_send![self.as_ref(), replaceObjectAtIndex:withObject:, index, object]
        }
    }
}

impl Default for NSMutableArray {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSMutableArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let description: *mut Object = msg_send![self.as_ref(), description];
            let nsstring = super::NSString::from_ptr(description);
            write!(f, "{}", nsstring.to_string())
        }
    }
}

impl Clone for NSArray {
    fn clone(&self) -> Self {
        unsafe {
            let copy: *mut Object = msg_send![self.as_ref(), copy];
            NSArray::from_ptr(copy)
        }
    }
}

impl<'a> IntoIterator for &'a NSMutableArray {
    type Item = &'a NSObjectRef;
    type IntoIter = NSArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // We can reuse the NSArray iterator since NSMutableArray is a subclass
        let array: &NSArray = unsafe { &*(self as *const NSMutableArray as *const NSArray) };
        NSArrayIterator {
            array,
            index: 0,
            count: array.count(),
        }
    }
}