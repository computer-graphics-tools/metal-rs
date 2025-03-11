//! MTLEvent - A Rust wrapper around Metal's MTLEvent class.
//!
//! This module provides safe Rust bindings to the MTLEvent class from Apple's Metal framework.
//! MTLEvent objects are used for GPU timeline synchronization, allowing you to coordinate operations
//! between different command buffers and encoders.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCommandBufferStatus};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create an event for GPU timeline synchronization
//! let event = device.new_event();
//! 
//! // Create a command queue and command buffer
//! let queue = device.new_command_queue();
//! let producer_command_buffer = queue.new_command_buffer();
//! let consumer_command_buffer = queue.new_command_buffer();
//! 
//! // Encode some commands in the producer command buffer
//! let blit_encoder = producer_command_buffer.new_blit_command_encoder();
//! // ... encode operations ...
//! 
//! // Signal the event when these operations are done with value 1
//! blit_encoder.signal_event(&event, value: 1);
//! blit_encoder.end_encoding();
//! 
//! // Create another encoder that waits for the event
//! let compute_encoder = consumer_command_buffer.new_compute_command_encoder();
//! // Wait for the event to reach value 1 before proceeding
//! compute_encoder.wait_for_event(&event, value: 1);
//! // ... encode operations that depend on the previous operations ...
//! compute_encoder.end_encoding();
//! 
//! // Submit work
//! producer_command_buffer.commit();
//! consumer_command_buffer.commit();
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSString, NSArray};
use crate::metal::MTLDevice;

/// A reference to an Objective-C `MTLEvent`.
pub struct MTLEventRef(Object);

/// An owned Objective-C `MTLEvent`.
pub struct MTLEvent(*mut Object);

unsafe impl ForeignTypeRef for MTLEventRef {
    type CType = Object;
}

unsafe impl Send for MTLEventRef {}
unsafe impl Sync for MTLEventRef {}

unsafe impl ForeignType for MTLEvent {
    type CType = Object;
    type Ref = MTLEventRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLEvent {
        MTLEvent(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLEventRef> for MTLEvent {
    fn as_ref(&self) -> &MTLEventRef {
        unsafe { &*(self.0.cast::<MTLEventRef>()) }
    }
}

unsafe impl Send for MTLEvent {}
unsafe impl Sync for MTLEvent {}

unsafe impl objc::Message for MTLEventRef {}

impl MTLEvent {
    /// Returns the device that created this event.
    #[must_use]
    pub fn device(&self) -> MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            MTLDevice::from_ptr(ptr)
        }
    }
    
    /// Returns the label of the event.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Sets the label of the event.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
}

impl fmt::Debug for MTLEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLEvent {{ label: {} }}", label)
    }
}

impl Drop for MTLEvent {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLEvent {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLEvent::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLSharedEvent`.
pub struct MTLSharedEventRef(Object);

/// An owned Objective-C `MTLSharedEvent`.
pub struct MTLSharedEvent(*mut Object);

unsafe impl ForeignTypeRef for MTLSharedEventRef {
    type CType = Object;
}

unsafe impl Send for MTLSharedEventRef {}
unsafe impl Sync for MTLSharedEventRef {}

unsafe impl ForeignType for MTLSharedEvent {
    type CType = Object;
    type Ref = MTLSharedEventRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLSharedEvent {
        MTLSharedEvent(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLSharedEventRef> for MTLSharedEvent {
    fn as_ref(&self) -> &MTLSharedEventRef {
        unsafe { &*(self.0.cast::<MTLSharedEventRef>()) }
    }
}

impl AsRef<MTLEventRef> for MTLSharedEvent {
    fn as_ref(&self) -> &MTLEventRef {
        unsafe { &*(self.0.cast::<MTLEventRef>()) }
    }
}

unsafe impl Send for MTLSharedEvent {}
unsafe impl Sync for MTLSharedEvent {}

unsafe impl objc::Message for MTLSharedEventRef {}

impl MTLSharedEvent {
    /// Returns the device that created this shared event.
    #[must_use]
    pub fn device(&self) -> MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            MTLDevice::from_ptr(ptr)
        }
    }
    
    /// Returns the label of the shared event.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Sets the label of the shared event.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Returns the signaled value of this shared event.
    #[must_use]
    pub fn signaled_value(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), signaledValue]
        }
    }
    
    /// Sets the signaled value of this shared event.
    pub fn set_signaled_value(&self, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setSignaledValue:value];
        }
    }
    
    /// Creates a new shared event handle for this shared event.
    #[must_use]
    pub fn new_shared_event_handle(&self) -> MTLSharedEventHandle {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newSharedEventHandle];
            MTLSharedEventHandle::from_ptr(ptr)
        }
    }
    
    /// Waits until the shared event reaches the specified value with an optional timeout.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to wait for.
    /// * `timeout_ms` - The timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// `true` if the event reached the specified value before the timeout, `false` otherwise.
    #[must_use]
    pub fn wait_until_signaled_value(&self, value: u64, timeout_ms: u64) -> bool {
        unsafe {
            msg_send![self.as_ref(), waitUntilSignaledValue:value timeoutMS:timeout_ms]
        }
    }
}

impl fmt::Debug for MTLSharedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        let value = self.signaled_value();
        write!(f, "MTLSharedEvent {{ label: {}, signaled_value: {} }}", label, value)
    }
}

impl Drop for MTLSharedEvent {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLSharedEvent {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLSharedEvent::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLSharedEventHandle`.
pub struct MTLSharedEventHandleRef(Object);

/// An owned Objective-C `MTLSharedEventHandle`.
pub struct MTLSharedEventHandle(*mut Object);

unsafe impl ForeignTypeRef for MTLSharedEventHandleRef {
    type CType = Object;
}

unsafe impl Send for MTLSharedEventHandleRef {}
unsafe impl Sync for MTLSharedEventHandleRef {}

unsafe impl ForeignType for MTLSharedEventHandle {
    type CType = Object;
    type Ref = MTLSharedEventHandleRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLSharedEventHandle {
        MTLSharedEventHandle(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLSharedEventHandleRef> for MTLSharedEventHandle {
    fn as_ref(&self) -> &MTLSharedEventHandleRef {
        unsafe { &*(self.0.cast::<MTLSharedEventHandleRef>()) }
    }
}

unsafe impl Send for MTLSharedEventHandle {}
unsafe impl Sync for MTLSharedEventHandle {}

unsafe impl objc::Message for MTLSharedEventHandleRef {}

impl MTLSharedEventHandle {
    /// Creates a new shared event handle.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLSharedEventHandle);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLSharedEventHandle::from_ptr(obj)
        }
    }
    
    /// Returns the label of the shared event handle.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
}

impl fmt::Debug for MTLSharedEventHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLSharedEventHandle {{ label: {} }}", label)
    }
}

impl Drop for MTLSharedEventHandle {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLSharedEventHandle {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLSharedEventHandle::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLSharedEventListener`.
pub struct MTLSharedEventListenerRef(Object);

/// An owned Objective-C `MTLSharedEventListener`.
pub struct MTLSharedEventListener(*mut Object);

unsafe impl ForeignTypeRef for MTLSharedEventListenerRef {
    type CType = Object;
}

unsafe impl Send for MTLSharedEventListenerRef {}
unsafe impl Sync for MTLSharedEventListenerRef {}

unsafe impl ForeignType for MTLSharedEventListener {
    type CType = Object;
    type Ref = MTLSharedEventListenerRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLSharedEventListener {
        MTLSharedEventListener(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLSharedEventListenerRef> for MTLSharedEventListener {
    fn as_ref(&self) -> &MTLSharedEventListenerRef {
        unsafe { &*(self.0.cast::<MTLSharedEventListenerRef>()) }
    }
}

unsafe impl Send for MTLSharedEventListener {}
unsafe impl Sync for MTLSharedEventListener {}

unsafe impl objc::Message for MTLSharedEventListenerRef {}

impl MTLSharedEventListener {
    /// Creates a new shared event listener that uses the default queue for dispatching notifications.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLSharedEventListener);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLSharedEventListener::from_ptr(obj)
        }
    }
}

impl fmt::Debug for MTLSharedEventListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLSharedEventListener {{ }}")
    }
}

impl Drop for MTLSharedEventListener {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLSharedEventListener {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLSharedEventListener::from_ptr(obj)
        }
    }
}