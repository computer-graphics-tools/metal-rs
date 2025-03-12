//! MTLIOCommandBuffer - A Rust wrapper around Metal's I/O command buffer API.
//!
//! This module provides Rust bindings to the MTLIOCommandBuffer class from Apple's Metal framework.
//! MTLIOCommandBuffer is used for asynchronous file I/O operations, such as loading data from files
//! to buffers or textures.

use std::fmt;
use std::os::raw::c_void;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::{Object, Class, Protocol};
use foreign_types::{ForeignType, ForeignTypeRef};
use block::Block;
use crate::foundation::{NSInteger, NSUInteger, NSString};
use crate::metal::{MTLBuffer, MTLBufferRef, MTLTexture, MTLTextureRef, MTLEvent, MTLSharedEvent};

/// Status of an I/O command buffer.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLIOStatus {
    /// The command buffer is pending execution.
    Pending = 0,
    /// The command buffer was cancelled.
    Cancelled = 1,
    /// The command buffer encountered an error.
    Error = 2,
    /// The command buffer has completed execution.
    Complete = 3,
}

/// Errors that can occur with I/O operations.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLIOError {
    /// The URL is invalid.
    URLInvalid = 1,
    /// An internal error occurred.
    Internal = 2,
}

/// A reference to an Objective-C `MTLIOFileHandle`.
pub struct MTLIOFileHandleRef(Object);

/// An owned Objective-C `MTLIOFileHandle`.
pub struct MTLIOFileHandle(*mut Object);

// Implementation for MTLIOFileHandle
unsafe impl ForeignTypeRef for MTLIOFileHandleRef {
    type CType = Object;
}

unsafe impl Send for MTLIOFileHandleRef {}
unsafe impl Sync for MTLIOFileHandleRef {}

unsafe impl ForeignType for MTLIOFileHandle {
    type CType = Object;
    type Ref = MTLIOFileHandleRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIOFileHandle {
        MTLIOFileHandle(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIOFileHandleRef> for MTLIOFileHandle {
    fn as_ref(&self) -> &MTLIOFileHandleRef {
        unsafe { &*(self.0.cast::<MTLIOFileHandleRef>()) }
    }
}

unsafe impl Send for MTLIOFileHandle {}
unsafe impl Sync for MTLIOFileHandle {}

unsafe impl objc::Message for MTLIOFileHandleRef {}

impl MTLIOFileHandle {
    /// Returns the label of the file handle.
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
    
    /// Sets the label of the file handle.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let nsstring = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:nsstring.as_ptr()];
        }
    }
}

impl fmt::Debug for MTLIOFileHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIOFileHandle")
            .field("label", &self.label())
            .finish()
    }
}

impl Drop for MTLIOFileHandle {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIOFileHandle {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIOFileHandle::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLIOScratchBuffer`.
pub struct MTLIOScratchBufferRef(Object);

/// An owned Objective-C `MTLIOScratchBuffer`.
pub struct MTLIOScratchBuffer(*mut Object);

// Implementation for MTLIOScratchBuffer
unsafe impl ForeignTypeRef for MTLIOScratchBufferRef {
    type CType = Object;
}

unsafe impl Send for MTLIOScratchBufferRef {}
unsafe impl Sync for MTLIOScratchBufferRef {}

unsafe impl ForeignType for MTLIOScratchBuffer {
    type CType = Object;
    type Ref = MTLIOScratchBufferRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIOScratchBuffer {
        MTLIOScratchBuffer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIOScratchBufferRef> for MTLIOScratchBuffer {
    fn as_ref(&self) -> &MTLIOScratchBufferRef {
        unsafe { &*(self.0.cast::<MTLIOScratchBufferRef>()) }
    }
}

unsafe impl Send for MTLIOScratchBuffer {}
unsafe impl Sync for MTLIOScratchBuffer {}

unsafe impl objc::Message for MTLIOScratchBufferRef {}

impl MTLIOScratchBuffer {
    /// Returns the buffer of the scratch buffer.
    #[must_use]
    pub fn buffer(&self) -> MTLBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), buffer];
            MTLBuffer::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLIOScratchBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIOScratchBuffer")
            .field("buffer", &self.buffer())
            .finish()
    }
}

impl Drop for MTLIOScratchBuffer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIOScratchBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIOScratchBuffer::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLIOScratchBufferAllocator`.
pub struct MTLIOScratchBufferAllocatorRef(Object);

/// An owned Objective-C `MTLIOScratchBufferAllocator`.
pub struct MTLIOScratchBufferAllocator(*mut Object);

// Implementation for MTLIOScratchBufferAllocator
unsafe impl ForeignTypeRef for MTLIOScratchBufferAllocatorRef {
    type CType = Object;
}

unsafe impl Send for MTLIOScratchBufferAllocatorRef {}
unsafe impl Sync for MTLIOScratchBufferAllocatorRef {}

unsafe impl ForeignType for MTLIOScratchBufferAllocator {
    type CType = Object;
    type Ref = MTLIOScratchBufferAllocatorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIOScratchBufferAllocator {
        MTLIOScratchBufferAllocator(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIOScratchBufferAllocatorRef> for MTLIOScratchBufferAllocator {
    fn as_ref(&self) -> &MTLIOScratchBufferAllocatorRef {
        unsafe { &*(self.0.cast::<MTLIOScratchBufferAllocatorRef>()) }
    }
}

unsafe impl Send for MTLIOScratchBufferAllocator {}
unsafe impl Sync for MTLIOScratchBufferAllocator {}

unsafe impl objc::Message for MTLIOScratchBufferAllocatorRef {}

impl MTLIOScratchBufferAllocator {
    /// Creates a new scratch buffer.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the buffer in bytes.
    ///
    /// # Returns
    ///
    /// A new scratch buffer.
    #[must_use]
    pub fn new_scratch_buffer(&self, size: NSUInteger) -> MTLIOScratchBuffer {
        unsafe {
            let ptr: *mut Object = msg_send![self.as_ref(), newScratchBufferWithMinimumSize:size];
            MTLIOScratchBuffer::from_ptr(ptr)
        }
    }
}

impl fmt::Debug for MTLIOScratchBufferAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIOScratchBufferAllocator").finish()
    }
}

impl Drop for MTLIOScratchBufferAllocator {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIOScratchBufferAllocator {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIOScratchBufferAllocator::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLIOCommandBuffer`.
pub struct MTLIOCommandBufferRef(Object);

/// An owned Objective-C `MTLIOCommandBuffer`.
pub struct MTLIOCommandBuffer(*mut Object);

// Implementation for MTLIOCommandBuffer
unsafe impl ForeignTypeRef for MTLIOCommandBufferRef {
    type CType = Object;
}

unsafe impl Send for MTLIOCommandBufferRef {}
unsafe impl Sync for MTLIOCommandBufferRef {}

unsafe impl ForeignType for MTLIOCommandBuffer {
    type CType = Object;
    type Ref = MTLIOCommandBufferRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLIOCommandBuffer {
        MTLIOCommandBuffer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLIOCommandBufferRef> for MTLIOCommandBuffer {
    fn as_ref(&self) -> &MTLIOCommandBufferRef {
        unsafe { &*(self.0.cast::<MTLIOCommandBufferRef>()) }
    }
}

unsafe impl Send for MTLIOCommandBuffer {}
unsafe impl Sync for MTLIOCommandBuffer {}

unsafe impl objc::Message for MTLIOCommandBufferRef {}

impl MTLIOCommandBuffer {
    /// Returns the label of the command buffer.
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
    
    /// Sets the label of the command buffer.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let nsstring = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:nsstring.as_ptr()];
        }
    }
    
    /// Returns the status of the command buffer.
    #[must_use]
    pub fn status(&self) -> MTLIOStatus {
        unsafe {
            let status: u64 = msg_send![self.as_ref(), status];
            std::mem::transmute(status)
        }
    }
    
    /// Returns the error that occurred during execution, if any.
    #[must_use]
    pub fn error(&self) -> Option<(MTLIOError, String)> {
        unsafe {
            let error: *mut Object = msg_send![self.as_ref(), error];
            if error.is_null() {
                None
            } else {
                let code: NSInteger = msg_send![error, code];
                let description: *mut Object = msg_send![error, localizedDescription];
                let description = NSString::from_ptr(description).to_rust_string();
                Some((std::mem::transmute(code), description))
            }
        }
    }
    
    /// Adds a completion handler that will be called when the command buffer completes.
    pub fn add_completed_handler<F>(&self, handler: F)
    where
        F: Fn(MTLIOStatus) + Send + 'static,
    {
        let handler_block = block::ConcreteBlock::new(move |status: u64| {
            let status: MTLIOStatus = unsafe { std::mem::transmute(status) };
            handler(status);
        }).copy();
        
        unsafe {
            let _: () = msg_send![self.as_ref(), addCompletedHandler:handler_block];
        }
    }
    
    /// Loads bytes from a file handle to a memory location.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The destination memory buffer.
    /// * `file_handle` - The source file handle.
    /// * `offset` - The offset into the file in bytes.
    /// * `size` - The number of bytes to read.
    pub fn load_bytes(&self, buffer: *mut c_void, file_handle: &MTLIOFileHandle, offset: u64, size: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), loadBytes:buffer
                                                  sourceHandle:file_handle.as_ptr()
                                                  sourceHandleOffset:offset
                                                  size:size];
        }
    }
    
    /// Loads data from a file handle to a buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The destination buffer.
    /// * `buffer_offset` - The offset into the buffer in bytes.
    /// * `file_handle` - The source file handle.
    /// * `file_offset` - The offset into the file in bytes.
    /// * `size` - The number of bytes to read.
    pub fn load_buffer(&self, buffer: &MTLBuffer, buffer_offset: NSUInteger, file_handle: &MTLIOFileHandle, file_offset: u64, size: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), loadBuffer:buffer.as_ptr()
                                                  offset:buffer_offset
                                                  sourceHandle:file_handle.as_ptr()
                                                  sourceHandleOffset:file_offset
                                                  size:size];
        }
    }
    
    /// Loads data from a file handle to a texture.
    ///
    /// # Arguments
    ///
    /// * `texture` - The destination texture.
    /// * `slice` - The slice of the texture.
    /// * `level` - The mipmap level of the texture.
    /// * `size` - The size of the region.
    /// * `origin` - The origin of the region.
    /// * `file_handle` - The source file handle.
    /// * `file_offset` - The offset into the file in bytes.
    pub fn load_texture(&self, texture: &MTLTexture, slice: NSUInteger, level: NSUInteger, size: crate::metal::MTLSize, origin: crate::metal::MTLOrigin, file_handle: &MTLIOFileHandle, file_offset: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), loadTexture:texture.as_ptr()
                                                  slice:slice
                                                  level:level
                                                  size:size
                                                  origin:origin
                                                  sourceHandle:file_handle.as_ptr()
                                                  sourceHandleOffset:file_offset];
        }
    }
    
    /// Copies the status of the command buffer to a buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The destination buffer.
    /// * `offset` - The offset into the buffer in bytes.
    pub fn copy_status_to_buffer(&self, buffer: &MTLBuffer, offset: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), copyStatusToBuffer:buffer.as_ptr()
                                                  offset:offset];
        }
    }
    
    /// Commits the command buffer for execution.
    pub fn commit(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), commit];
        }
    }
    
    /// Waits for the command buffer to complete.
    pub fn wait_until_completed(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), waitUntilCompleted];
        }
    }
    
    /// Attempts to cancel the command buffer.
    pub fn try_cancel(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), tryCancel];
        }
    }
    
    /// Adds a synchronization barrier.
    pub fn add_barrier(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), addBarrier];
        }
    }
    
    /// Pushes a debug group.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the debug group.
    pub fn push_debug_group(&self, name: &str) {
        unsafe {
            let nsstring = NSString::from_rust_str(name);
            let _: () = msg_send![self.as_ref(), pushDebugGroup:nsstring.as_ptr()];
        }
    }
    
    /// Pops a debug group.
    pub fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), popDebugGroup];
        }
    }
    
    /// Enqueues the command buffer.
    pub fn enqueue(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), enqueue];
        }
    }
    
    /// Waits for a shared event to reach the specified value.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to wait for.
    /// * `value` - The value to wait for.
    pub fn wait(&self, event: &MTLSharedEvent, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), waitForEvent:event.as_ptr()
                                                  value:value];
        }
    }
    
    /// Signals a shared event.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to signal.
    /// * `value` - The value to signal with.
    pub fn signal_event(&self, event: &MTLSharedEvent, value: u64) {
        unsafe {
            let _: () = msg_send![self.as_ref(), signalEvent:event.as_ptr()
                                                  value:value];
        }
    }
}

impl fmt::Debug for MTLIOCommandBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIOCommandBuffer")
            .field("label", &self.label())
            .field("status", &self.status())
            .field("error", &self.error())
            .finish()
    }
}

impl Drop for MTLIOCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLIOCommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLIOCommandBuffer::from_ptr(obj)
        }
    }
}