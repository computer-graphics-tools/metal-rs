//! MTLCaptureManager - A Rust wrapper around Metal's capture manager API.
//!
//! This module provides Rust bindings to the MTLCaptureManager class from Apple's Metal framework.
//! MTLCaptureManager is used for capturing Metal API calls and GPU activity for debugging
//! and profiling purposes.

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::{Object, Class};
use objc::rc::StrongPtr;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSError, NSErrorOptionRef, NSString, NSURL};
use crate::metal::{MTLDevice, MTLDeviceRef, MTLCommandQueue, MTLCommandQueueRef};

/// An error that can occur during capture operations.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLCaptureError {
    /// The capture operation is not supported.
    NotSupported = 1,
    /// A capture is already in progress.
    AlreadyCapturing = 2,
    /// The capture descriptor is invalid.
    InvalidDescriptor = 3,
}

/// The destination for a capture.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLCaptureDestination {
    /// Capture to developer tools (like Xcode or Instruments).
    DeveloperTools = 1,
    /// Capture to a GPU trace document.
    GPUTraceDocument = 2,
}

/// A reference to an Objective-C `MTLCaptureDescriptor`.
pub struct MTLCaptureDescriptorRef(Object);

/// An owned Objective-C `MTLCaptureDescriptor`.
pub struct MTLCaptureDescriptor(*mut Object);

// Implementation for MTLCaptureDescriptor
unsafe impl ForeignTypeRef for MTLCaptureDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLCaptureDescriptorRef {}
unsafe impl Sync for MTLCaptureDescriptorRef {}

unsafe impl ForeignType for MTLCaptureDescriptor {
    type CType = Object;
    type Ref = MTLCaptureDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCaptureDescriptor {
        MTLCaptureDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCaptureDescriptorRef> for MTLCaptureDescriptor {
    fn as_ref(&self) -> &MTLCaptureDescriptorRef {
        unsafe { &*(self.0.cast::<MTLCaptureDescriptorRef>()) }
    }
}

unsafe impl Send for MTLCaptureDescriptor {}
unsafe impl Sync for MTLCaptureDescriptor {}

unsafe impl objc::Message for MTLCaptureDescriptorRef {}

impl MTLCaptureDescriptor {
    /// Creates a new `MTLCaptureDescriptor`.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLCaptureDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLCaptureDescriptor::from_ptr(obj)
        }
    }
    
    /// Sets the object to capture.
    ///
    /// # Safety
    ///
    /// The object must be a valid Metal object like a device, command queue, or capture scope.
    pub unsafe fn set_capture_object(&self, obj: *mut Object) {
        let _: () = msg_send![self.as_ref(), setCaptureObject:obj];
    }
    
    /// Sets the destination for the capture.
    pub fn set_destination(&self, destination: MTLCaptureDestination) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setDestination:destination];
        }
    }
    
    /// Sets the output URL for the capture if the destination is `GPUTraceDocument`.
    pub fn set_output_url(&self, url: &NSURL) {
        unsafe {
            // Get the raw pointer to the NSURL
            let url_ptr = std::mem::transmute::<&NSURL, *mut Object>(url);
            let _: () = msg_send![self.as_ref(), setOutputURL:url_ptr];
        }
    }
    
    /// Gets the destination for the capture.
    #[must_use]
    pub fn destination(&self) -> MTLCaptureDestination {
        unsafe {
            let result: u64 = msg_send![self.as_ref(), destination];
            std::mem::transmute(result)
        }
    }
}

impl fmt::Debug for MTLCaptureDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLCaptureDescriptor")
            .field("destination", &self.destination())
            .finish()
    }
}

impl Drop for MTLCaptureDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCaptureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCaptureDescriptor::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLCaptureScope`.
pub struct MTLCaptureScopeRef(Object);

/// An owned Objective-C `MTLCaptureScope`.
pub struct MTLCaptureScope(*mut Object);

// Implementation for MTLCaptureScope
unsafe impl ForeignTypeRef for MTLCaptureScopeRef {
    type CType = Object;
}

unsafe impl Send for MTLCaptureScopeRef {}
unsafe impl Sync for MTLCaptureScopeRef {}

unsafe impl ForeignType for MTLCaptureScope {
    type CType = Object;
    type Ref = MTLCaptureScopeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCaptureScope {
        MTLCaptureScope(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCaptureScopeRef> for MTLCaptureScope {
    fn as_ref(&self) -> &MTLCaptureScopeRef {
        unsafe { &*(self.0.cast::<MTLCaptureScopeRef>()) }
    }
}

unsafe impl Send for MTLCaptureScope {}
unsafe impl Sync for MTLCaptureScope {}

unsafe impl objc::Message for MTLCaptureScopeRef {}

impl MTLCaptureScope {
    /// Begins the capture scope.
    pub fn begin_scope(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), beginScope];
        }
    }
    
    /// Ends the capture scope.
    pub fn end_scope(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), endScope];
        }
    }
    
    /// Gets the device associated with this capture scope.
    #[must_use]
    pub fn device(&self) -> Option<MTLDevice> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            if ptr.is_null() {
                None
            } else {
                Some(MTLDevice::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the command queue associated with this capture scope.
    #[must_use]
    pub fn command_queue(&self) -> Option<MTLCommandQueue> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), commandQueue];
            if ptr.is_null() {
                None
            } else {
                Some(MTLCommandQueue::from_ptr(ptr))
            }
        }
    }
    
    /// Gets the label for this capture scope.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            if label.is_null() {
                String::new()
            } else {
                let nsstring = NSString::from_ptr(label);
                nsstring.to_rust_string()
            }
        }
    }
    
    /// Sets the label for this capture scope.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let nsstring = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:nsstring.as_ptr()];
        }
    }
}

impl fmt::Debug for MTLCaptureScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLCaptureScope")
            .field("label", &self.label())
            .field("device", &self.device().is_some())
            .field("command_queue", &self.command_queue().is_some())
            .finish()
    }
}

impl Drop for MTLCaptureScope {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCaptureScope {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCaptureScope::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLCaptureManager`.
pub struct MTLCaptureManagerRef(Object);

/// An owned Objective-C `MTLCaptureManager`.
pub struct MTLCaptureManager(*mut Object);

// Implementation for MTLCaptureManager
unsafe impl ForeignTypeRef for MTLCaptureManagerRef {
    type CType = Object;
}

unsafe impl Send for MTLCaptureManagerRef {}
unsafe impl Sync for MTLCaptureManagerRef {}

unsafe impl ForeignType for MTLCaptureManager {
    type CType = Object;
    type Ref = MTLCaptureManagerRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCaptureManager {
        MTLCaptureManager(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCaptureManagerRef> for MTLCaptureManager {
    fn as_ref(&self) -> &MTLCaptureManagerRef {
        unsafe { &*(self.0.cast::<MTLCaptureManagerRef>()) }
    }
}

unsafe impl Send for MTLCaptureManager {}
unsafe impl Sync for MTLCaptureManager {}

unsafe impl objc::Message for MTLCaptureManagerRef {}

impl MTLCaptureManager {
    /// Returns the shared capture manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::MTLCaptureManager;
    ///
    /// let manager = MTLCaptureManager::shared();
    /// ```
    #[must_use]
    pub fn shared() -> Self {
        unsafe {
            let class = class!(MTLCaptureManager);
            let obj: *mut Object = msg_send![class, sharedCaptureManager];
            MTLCaptureManager::from_ptr(obj)
        }
    }
    
    /// Creates a new capture scope for the given device.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCreateSystemDefaultDevice};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let manager = MTLCaptureManager::shared();
    /// let scope = manager.new_capture_scope_with_device(&device);
    /// ```
    #[must_use]
    pub fn new_capture_scope_with_device(&self, device: &MTLDevice) -> MTLCaptureScope {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newCaptureScopeWithDevice:device.as_ptr()];
            MTLCaptureScope::from_ptr(ptr)
        }
    }
    
    /// Creates a new capture scope for the given command queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCreateSystemDefaultDevice};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let queue = device.new_command_queue();
    /// let manager = MTLCaptureManager::shared();
    /// let scope = manager.new_capture_scope_with_command_queue(&queue);
    /// ```
    #[must_use]
    pub fn new_capture_scope_with_command_queue(&self, queue: &MTLCommandQueue) -> MTLCaptureScope {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newCaptureScopeWithCommandQueue:queue.as_ptr()];
            MTLCaptureScope::from_ptr(ptr)
        }
    }
    
    /// Returns whether the given destination is supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCaptureDestination};
    ///
    /// let manager = MTLCaptureManager::shared();
    /// let supported = manager.supports_destination(MTLCaptureDestination::DeveloperTools);
    /// ```
    #[must_use]
    pub fn supports_destination(&self, destination: MTLCaptureDestination) -> bool {
        unsafe {
            let result: bool = msg_send![self.as_ref(), supportsDestination:destination];
            result
        }
    }
    
    /// Starts a capture using the given descriptor. Returns an error if the capture could not be started.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCaptureDescriptor, MTLCaptureDestination, MTLCreateSystemDefaultDevice};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let manager = MTLCaptureManager::shared();
    /// let descriptor = MTLCaptureDescriptor::new();
    /// unsafe {
    ///     descriptor.set_capture_object(device.as_ptr());
    /// }
    /// descriptor.set_destination(MTLCaptureDestination::DeveloperTools);
    /// let result = manager.start_capture_with_descriptor(&descriptor);
    /// ```
    pub fn start_capture_with_descriptor(&self, descriptor: &MTLCaptureDescriptor) -> Result<(), (MTLCaptureError, String)> {
        unsafe {
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let result: bool = msg_send![self.as_ref(), startCaptureWithDescriptor:descriptor.as_ptr() error:&mut error_ptr];
            if result {
                Ok(())
            } else if !error_ptr.is_null() {
                // Create NSError from the raw pointer
                let error_ref = NSErrorOptionRef::from_ptr(error_ptr);
                let code = error_ref.code();
                let desc = error_ref.localized_description();
                Err((std::mem::transmute(code), desc))
            } else {
                Err((MTLCaptureError::NotSupported, String::from("Unknown error occurred")))
            }
        }
    }
    
    /// Starts a capture for the given device.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCreateSystemDefaultDevice};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let manager = MTLCaptureManager::shared();
    /// manager.start_capture_with_device(&device);
    /// ```
    pub fn start_capture_with_device(&self, device: &MTLDevice) {
        unsafe {
            let _: () = msg_send![self.as_ref(), startCaptureWithDevice:device.as_ptr()];
        }
    }
    
    /// Starts a capture for the given command queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCreateSystemDefaultDevice};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let queue = device.new_command_queue();
    /// let manager = MTLCaptureManager::shared();
    /// manager.start_capture_with_command_queue(&queue);
    /// ```
    pub fn start_capture_with_command_queue(&self, queue: &MTLCommandQueue) {
        unsafe {
            let _: () = msg_send![self.as_ref(), startCaptureWithCommandQueue:queue.as_ptr()];
        }
    }
    
    /// Starts a capture for the given capture scope.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCreateSystemDefaultDevice};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let manager = MTLCaptureManager::shared();
    /// let scope = manager.new_capture_scope_with_device(&device);
    /// manager.start_capture_with_scope(&scope);
    /// ```
    pub fn start_capture_with_scope(&self, scope: &MTLCaptureScope) {
        unsafe {
            let _: () = msg_send![self.as_ref(), startCaptureWithScope:scope.as_ptr()];
        }
    }
    
    /// Stops the current capture.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{MTLCaptureManager, MTLCreateSystemDefaultDevice};
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// let manager = MTLCaptureManager::shared();
    /// manager.start_capture_with_device(&device);
    /// // ... Do work ...
    /// manager.stop_capture();
    /// ```
    pub fn stop_capture(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), stopCapture];
        }
    }
    
    /// Returns the default capture scope.
    #[must_use]
    pub fn default_capture_scope(&self) -> Option<MTLCaptureScope> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), defaultCaptureScope];
            if ptr.is_null() {
                None
            } else {
                Some(MTLCaptureScope::from_ptr(ptr))
            }
        }
    }
    
    /// Sets the default capture scope.
    pub fn set_default_capture_scope(&self, scope: Option<&MTLCaptureScope>) {
        unsafe {
            let ptr = match scope {
                Some(s) => s.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self.as_ref(), setDefaultCaptureScope:ptr];
        }
    }
    
    /// Returns whether a capture is currently in progress.
    #[must_use]
    pub fn is_capturing(&self) -> bool {
        unsafe {
            let result: bool = msg_send![self.as_ref(), isCapturing];
            result
        }
    }
}

impl fmt::Debug for MTLCaptureManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLCaptureManager")
            .field("is_capturing", &self.is_capturing())
            .field("default_capture_scope", &self.default_capture_scope().is_some())
            .finish()
    }
}

impl Drop for MTLCaptureManager {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCaptureManager {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCaptureManager::from_ptr(obj)
        }
    }
}