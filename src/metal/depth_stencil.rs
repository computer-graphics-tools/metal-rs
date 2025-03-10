//! MTLDepthStencil - A Rust wrapper around Metal's MTLDepthStencil class.
//!
//! This module provides safe Rust bindings to the MTLDepthStencil, MTLDepthStencilDescriptor,
//! and MTLStencilDescriptor classes from Apple's Metal framework.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCompareFunction, MTLStencilOperation};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a depth stencil descriptor
//! let depth_stencil_desc = MTLDepthStencilDescriptor::new();
//! depth_stencil_desc.set_depth_compare_function(MTLCompareFunction::Less);
//! depth_stencil_desc.set_depth_write_enabled(true);
//! 
//! // Configure stencil if needed
//! let front_stencil_desc = MTLStencilDescriptor::new();
//! front_stencil_desc.set_stencil_compare_function(MTLCompareFunction::Always);
//! front_stencil_desc.set_stencil_failure_operation(MTLStencilOperation::Keep);
//! front_stencil_desc.set_depth_failure_operation(MTLStencilOperation::Keep);
//! front_stencil_desc.set_depth_stencil_pass_operation(MTLStencilOperation::Keep);
//! front_stencil_desc.set_read_mask(0xFF);
//! front_stencil_desc.set_write_mask(0xFF);
//! 
//! // Attach stencil descriptor
//! depth_stencil_desc.set_front_face_stencil(&front_stencil_desc);
//! 
//! // Create the depth stencil state
//! let depth_stencil_state = device.new_depth_stencil_state(&depth_stencil_desc);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::NSString;
use crate::metal::device::MTLDevice;

/// MTLCompareFunction - Enum representing depth and stencil comparison functions.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLCompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

/// MTLStencilOperation - Enum representing stencil operations.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLStencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementClamp = 3,
    DecrementClamp = 4,
    Invert = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

/// A reference to an Objective-C `MTLStencilDescriptor`.
pub struct MTLStencilDescriptorRef(Object);

/// An owned Objective-C `MTLStencilDescriptor`.
pub struct MTLStencilDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLStencilDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLStencilDescriptorRef {}
unsafe impl Sync for MTLStencilDescriptorRef {}

unsafe impl ForeignType for MTLStencilDescriptor {
    type CType = Object;
    type Ref = MTLStencilDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLStencilDescriptor {
        MTLStencilDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLStencilDescriptorRef> for MTLStencilDescriptor {
    fn as_ref(&self) -> &MTLStencilDescriptorRef {
        unsafe { &*(self.0.cast::<MTLStencilDescriptorRef>()) }
    }
}

unsafe impl Send for MTLStencilDescriptor {}
unsafe impl Sync for MTLStencilDescriptor {}

unsafe impl objc::Message for MTLStencilDescriptorRef {}

impl MTLStencilDescriptor {
    /// Creates a new stencil descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLStencilDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLStencilDescriptor::from_ptr(obj)
        }
    }
    
    /// Sets the comparison function.
    pub fn set_stencil_compare_function(&self, function: MTLCompareFunction) {
        unsafe {
            msg_send![self.as_ref(), setStencilCompareFunction:function]
        }
    }
    
    /// Gets the comparison function.
    #[must_use]
    pub fn stencil_compare_function(&self) -> MTLCompareFunction {
        unsafe {
            msg_send![self.as_ref(), stencilCompareFunction]
        }
    }
    
    /// Sets the stencil failure operation.
    pub fn set_stencil_failure_operation(&self, operation: MTLStencilOperation) {
        unsafe {
            msg_send![self.as_ref(), setStencilFailureOperation:operation]
        }
    }
    
    /// Gets the stencil failure operation.
    #[must_use]
    pub fn stencil_failure_operation(&self) -> MTLStencilOperation {
        unsafe {
            msg_send![self.as_ref(), stencilFailureOperation]
        }
    }
    
    /// Sets the depth failure operation.
    pub fn set_depth_failure_operation(&self, operation: MTLStencilOperation) {
        unsafe {
            msg_send![self.as_ref(), setDepthFailureOperation:operation]
        }
    }
    
    /// Gets the depth failure operation.
    #[must_use]
    pub fn depth_failure_operation(&self) -> MTLStencilOperation {
        unsafe {
            msg_send![self.as_ref(), depthFailureOperation]
        }
    }
    
    /// Sets the depth stencil pass operation.
    pub fn set_depth_stencil_pass_operation(&self, operation: MTLStencilOperation) {
        unsafe {
            msg_send![self.as_ref(), setDepthStencilPassOperation:operation]
        }
    }
    
    /// Gets the depth stencil pass operation.
    #[must_use]
    pub fn depth_stencil_pass_operation(&self) -> MTLStencilOperation {
        unsafe {
            msg_send![self.as_ref(), depthStencilPassOperation]
        }
    }
    
    /// Sets the read mask.
    pub fn set_read_mask(&self, mask: u32) {
        unsafe {
            msg_send![self.as_ref(), setReadMask:mask]
        }
    }
    
    /// Gets the read mask.
    #[must_use]
    pub fn read_mask(&self) -> u32 {
        unsafe {
            msg_send![self.as_ref(), readMask]
        }
    }
    
    /// Sets the write mask.
    pub fn set_write_mask(&self, mask: u32) {
        unsafe {
            msg_send![self.as_ref(), setWriteMask:mask]
        }
    }
    
    /// Gets the write mask.
    #[must_use]
    pub fn write_mask(&self) -> u32 {
        unsafe {
            msg_send![self.as_ref(), writeMask]
        }
    }
}

impl fmt::Debug for MTLStencilDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLStencilDescriptor")
            .field("compare_function", &self.stencil_compare_function())
            .field("stencil_failure_operation", &self.stencil_failure_operation())
            .field("depth_failure_operation", &self.depth_failure_operation())
            .field("depth_stencil_pass_operation", &self.depth_stencil_pass_operation())
            .field("read_mask", &self.read_mask())
            .field("write_mask", &self.write_mask())
            .finish()
    }
}

impl Drop for MTLStencilDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLStencilDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLStencilDescriptor::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLDepthStencilDescriptor`.
pub struct MTLDepthStencilDescriptorRef(Object);

/// An owned Objective-C `MTLDepthStencilDescriptor`.
pub struct MTLDepthStencilDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLDepthStencilDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLDepthStencilDescriptorRef {}
unsafe impl Sync for MTLDepthStencilDescriptorRef {}

unsafe impl ForeignType for MTLDepthStencilDescriptor {
    type CType = Object;
    type Ref = MTLDepthStencilDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLDepthStencilDescriptor {
        MTLDepthStencilDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLDepthStencilDescriptorRef> for MTLDepthStencilDescriptor {
    fn as_ref(&self) -> &MTLDepthStencilDescriptorRef {
        unsafe { &*(self.0.cast::<MTLDepthStencilDescriptorRef>()) }
    }
}

unsafe impl Send for MTLDepthStencilDescriptor {}
unsafe impl Sync for MTLDepthStencilDescriptor {}

unsafe impl objc::Message for MTLDepthStencilDescriptorRef {}

impl MTLDepthStencilDescriptor {
    /// Creates a new depth stencil descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLDepthStencilDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLDepthStencilDescriptor::from_ptr(obj)
        }
    }
    
    /// Sets the depth comparison function.
    pub fn set_depth_compare_function(&self, function: MTLCompareFunction) {
        unsafe {
            msg_send![self.as_ref(), setDepthCompareFunction:function]
        }
    }
    
    /// Gets the depth comparison function.
    #[must_use]
    pub fn depth_compare_function(&self) -> MTLCompareFunction {
        unsafe {
            msg_send![self.as_ref(), depthCompareFunction]
        }
    }
    
    /// Sets whether depth writing is enabled.
    pub fn set_depth_write_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self.as_ref(), setDepthWriteEnabled:enabled]
        }
    }
    
    /// Gets whether depth writing is enabled.
    #[must_use]
    pub fn depth_write_enabled(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), isDepthWriteEnabled]
        }
    }
    
    /// Sets the front face stencil descriptor.
    pub fn set_front_face_stencil(&self, stencil: &MTLStencilDescriptor) {
        unsafe {
            msg_send![self.as_ref(), setFrontFaceStencil:stencil.as_ptr()]
        }
    }
    
    /// Gets the front face stencil descriptor.
    #[must_use]
    pub fn front_face_stencil(&self) -> Option<MTLStencilDescriptor> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), frontFaceStencil];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStencilDescriptor::from_ptr(msg_send![ptr, retain]))
            }
        }
    }
    
    /// Sets the back face stencil descriptor.
    pub fn set_back_face_stencil(&self, stencil: &MTLStencilDescriptor) {
        unsafe {
            msg_send![self.as_ref(), setBackFaceStencil:stencil.as_ptr()]
        }
    }
    
    /// Gets the back face stencil descriptor.
    #[must_use]
    pub fn back_face_stencil(&self) -> Option<MTLStencilDescriptor> {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), backFaceStencil];
            if ptr.is_null() {
                None
            } else {
                Some(MTLStencilDescriptor::from_ptr(msg_send![ptr, retain]))
            }
        }
    }
    
    /// Sets the label for the descriptor.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            msg_send![self.as_ref(), setLabel:ns_string.as_ptr()]
        }
    }
    
    /// Gets the label for the descriptor.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), label];
            if ns_string.is_null() {
                String::new()
            } else {
                NSString::from_ptr(ns_string).to_rust_string()
            }
        }
    }
}

impl fmt::Debug for MTLDepthStencilDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLDepthStencilDescriptor")
            .field("depth_compare_function", &self.depth_compare_function())
            .field("depth_write_enabled", &self.depth_write_enabled())
            .field("front_face_stencil", &self.front_face_stencil())
            .field("back_face_stencil", &self.back_face_stencil())
            .field("label", &self.label())
            .finish()
    }
}

impl Drop for MTLDepthStencilDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLDepthStencilDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLDepthStencilDescriptor::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLDepthStencilState`.
pub struct MTLDepthStencilStateRef(Object);

/// An owned Objective-C `MTLDepthStencilState`.
pub struct MTLDepthStencilState(*mut Object);

unsafe impl ForeignTypeRef for MTLDepthStencilStateRef {
    type CType = Object;
}

unsafe impl Send for MTLDepthStencilStateRef {}
unsafe impl Sync for MTLDepthStencilStateRef {}

unsafe impl ForeignType for MTLDepthStencilState {
    type CType = Object;
    type Ref = MTLDepthStencilStateRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLDepthStencilState {
        MTLDepthStencilState(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLDepthStencilStateRef> for MTLDepthStencilState {
    fn as_ref(&self) -> &MTLDepthStencilStateRef {
        unsafe { &*(self.0.cast::<MTLDepthStencilStateRef>()) }
    }
}

unsafe impl Send for MTLDepthStencilState {}
unsafe impl Sync for MTLDepthStencilState {}

unsafe impl objc::Message for MTLDepthStencilStateRef {}

impl MTLDepthStencilState {
    /// Gets the label for the depth stencil state.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), label];
            if ns_string.is_null() {
                String::new()
            } else {
                NSString::from_ptr(ns_string).to_rust_string()
            }
        }
    }

    /// Gets the device that created this depth stencil state.
    #[must_use]
    pub fn device(&self) -> MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), device];
            MTLDevice::from_ptr(msg_send![ptr, retain])
        }
    }
}

impl fmt::Debug for MTLDepthStencilState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLDepthStencilState")
            .field("label", &self.label())
            .finish()
    }
}

impl Drop for MTLDepthStencilState {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLDepthStencilState {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLDepthStencilState::from_ptr(obj)
        }
    }
}