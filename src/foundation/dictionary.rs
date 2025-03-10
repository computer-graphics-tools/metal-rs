//! NSDictionary - A static collection of keys and values.

use crate::foreign_obj_type;
use crate::foundation::object::{NSObject, NSObjectRef};
use crate::foundation::array::{NSArray, NSArrayRef};
use crate::foundation::types::NSUInteger;
use crate::msg_send_bool;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL};
use std::fmt;
use std::marker::PhantomData;

/// A reference to an Objective-C `NSDictionary`.
pub enum NSDictionaryRef {}

/// An owned Objective-C `NSDictionary`.
pub struct NSDictionary(foreign_types::ForeignObjectRef<NSDictionaryRef>);

foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSDictionary;
    pub struct NSDictionaryRef;
    type ParentType = super::object::NSObjectRef;
}

unsafe impl objc::Message for NSDictionaryRef {}

/// A reference to an Objective-C `NSMutableDictionary`.
pub enum NSMutableDictionaryRef {}

/// An owned Objective-C `NSMutableDictionary`.
pub struct NSMutableDictionary(foreign_types::ForeignObjectRef<NSMutableDictionaryRef>);

foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSMutableDictionary;
    pub struct NSMutableDictionaryRef;
    type ParentType = NSDictionaryRef;
}

unsafe impl objc::Message for NSMutableDictionaryRef {}

impl NSDictionary {
    /// Creates an empty dictionary.
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSDictionary);
            let obj: *mut Object = msg_send![class, dictionary];
            NSDictionary::from_ptr(obj)
        }
    }

    /// Creates a dictionary with the given key-value pairs.
    pub fn from_keys_and_objects<K: AsRef<NSObjectRef>, V: AsRef<NSObjectRef>>(
        keys: &[K],
        objects: &[V],
    ) -> Self {
        let count = keys.len().min(objects.len());
        
        // Empty dictionary
        if count == 0 {
            return Self::new();
        }
        
        // Create temporary Vecs of pointers
        let mut key_ptrs = Vec::with_capacity(count);
        let mut obj_ptrs = Vec::with_capacity(count);
        
        for i in 0..count {
            key_ptrs.push(keys[i].as_ref() as *const NSObjectRef);
            obj_ptrs.push(objects[i].as_ref() as *const NSObjectRef);
        }
        
        unsafe {
            let class = class!(NSDictionary);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithObjects:forKeys:count:, 
                                           obj_ptrs.as_ptr(), 
                                           key_ptrs.as_ptr(),
                                           count];
            NSDictionary::from_ptr(obj)
        }
    }

    /// Returns the number of entries in the dictionary.
    pub fn count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), count]
        }
    }

    /// Returns whether the dictionary is empty.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Returns the object for the given key, or None if the key is not found.
    pub fn object_for(&self, key: &NSObjectRef) -> Option<&NSObjectRef> {
        unsafe {
            let obj: *mut Object = msg_send![self.as_ref(), objectForKey:, key];
            if obj.is_null() {
                None
            } else {
                Some(&*(obj as *const NSObjectRef))
            }
        }
    }

    /// Returns all keys in the dictionary.
    pub fn all_keys(&self) -> NSArray {
        unsafe {
            let keys: *mut Object = msg_send![self.as_ref(), allKeys];
            NSArray::from_ptr(keys)
        }
    }

    /// Returns all objects in the dictionary.
    pub fn all_values(&self) -> NSArray {
        unsafe {
            let values: *mut Object = msg_send![self.as_ref(), allValues];
            NSArray::from_ptr(values)
        }
    }

    /// Returns whether the dictionary contains the given key.
    pub fn contains_key(&self, key: &NSObjectRef) -> bool {
        self.object_for(key).is_some()
    }
}

impl Default for NSDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let description: *mut Object = msg_send![self.as_ref(), description];
            let nsstring = super::NSString::from_ptr(description);
            write!(f, "{}", nsstring.to_string())
        }
    }
}

impl Clone for NSDictionary {
    fn clone(&self) -> Self {
        unsafe {
            let copy: *mut Object = msg_send![self.as_ref(), copy];
            NSDictionary::from_ptr(copy)
        }
    }
}

/// Iterator over NSDictionary key-value pairs.
pub struct NSDictionaryIterator<'a> {
    keys: NSArray,
    dict: &'a NSDictionary,
    index: NSUInteger,
    count: NSUInteger,
}

impl<'a> Iterator for NSDictionaryIterator<'a> {
    type Item = (&'a NSObjectRef, &'a NSObjectRef);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        
        let key = self.keys.object_at(self.index)?;
        let value = self.dict.object_for(key)?;
        
        self.index += 1;
        Some((key, value))
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.count - self.index) as usize;
        (remaining, Some(remaining))
    }
}

impl<'a> IntoIterator for &'a NSDictionary {
    type Item = (&'a NSObjectRef, &'a NSObjectRef);
    type IntoIter = NSDictionaryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let keys = self.all_keys();
        let count = keys.count();
        
        NSDictionaryIterator {
            keys,
            dict: self,
            index: 0,
            count,
        }
    }
}

impl NSMutableDictionary {
    /// Creates a new mutable dictionary.
    pub fn new() -> Self {
        unsafe {
            let class = class!(NSMutableDictionary);
            let obj: *mut Object = msg_send![class, dictionary];
            NSMutableDictionary::from_ptr(obj)
        }
    }
    
    /// Creates a new mutable dictionary with a capacity.
    pub fn with_capacity(capacity: NSUInteger) -> Self {
        unsafe {
            let class = class!(NSMutableDictionary);
            let obj: *mut Object = msg_send![class, dictionaryWithCapacity:, capacity];
            NSMutableDictionary::from_ptr(obj)
        }
    }
    
    /// Sets the object for the given key.
    pub fn set_object(&self, key: &NSObjectRef, value: &NSObjectRef) {
        unsafe {
            msg_send![self.as_ref(), setObject:forKey:, value, key]
        }
    }
    
    /// Removes the object for the given key.
    pub fn remove_object_for_key(&self, key: &NSObjectRef) {
        unsafe {
            msg_send![self.as_ref(), removeObjectForKey:, key]
        }
    }
    
    /// Removes all objects from the dictionary.
    pub fn remove_all_objects(&self) {
        unsafe {
            msg_send![self.as_ref(), removeAllObjects]
        }
    }
    
    /// Adds the entries from another dictionary to this dictionary.
    pub fn add_entries_from_dictionary(&self, other: &NSDictionaryRef) {
        unsafe {
            msg_send![self.as_ref(), addEntriesFromDictionary:, other]
        }
    }
}

impl Default for NSMutableDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSMutableDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let description: *mut Object = msg_send![self.as_ref(), description];
            let nsstring = super::NSString::from_ptr(description);
            write!(f, "{}", nsstring.to_string())
        }
    }
}

impl<'a> IntoIterator for &'a NSMutableDictionary {
    type Item = (&'a NSObjectRef, &'a NSObjectRef);
    type IntoIter = NSDictionaryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // We can reuse the NSDictionary iterator since NSMutableDictionary is a subclass
        let dict: &NSDictionary = unsafe { &*(self as *const NSMutableDictionary as *const NSDictionary) };
        let keys = dict.all_keys();
        let count = keys.count();
        
        NSDictionaryIterator {
            keys,
            dict,
            index: 0,
            count,
        }
    }
}