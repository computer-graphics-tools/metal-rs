//! Wrapper for `id<MTLCommandQueue>`.
//!
//! A [`CommandQueue`] organises the submission of [`CommandBuffer`] objects
//! for execution on the GPU.

use crate::prelude::*;
use objc2::encode::{Encoding, RefEncode};
use objc2::Message;

extern_protocol!(pub unsafe trait CommandQueueProtocol: NSObjectProtocol {
});
/// Thin wrapper over `id<MTLCommandQueue>` implemented as transparent newtype
#[repr(transparent)]
pub struct CommandQueue(ProtocolObject<dyn CommandQueueProtocol>);

impl core::ops::Deref for CommandQueue {
    type Target = ProtocolObject<dyn CommandQueueProtocol>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

// SAFETY: Transparent wrapper over Objective-C pointer
unsafe impl Message for CommandQueue {}
unsafe impl RefEncode for CommandQueue {
    const ENCODING_REF: Encoding = <ProtocolObject<dyn CommandQueueProtocol>>::ENCODING_REF;
}

impl CommandQueue {
    /// Returns / sets the debug label visible in Xcode's GPU frame-debugger.
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_str: *mut NSString = msg_send![self, label];
            Retained::retain_autoreleased(ns_str).map(|s| s.to_string())
        }
    }

    /// Set a new debug label.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_label = NSString::from_str(label);
            let _: () = msg_send![self, setLabel: &*ns_label];
        }
    }

    /// Returns the device that created this command queue.
    pub fn device(&self) -> Retained<crate::device::Device> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::device::DeviceProtocol> = msg_send![self, device];
            // The device property should always return a valid, retained object.
            let po = Retained::retain_autoreleased(ptr).expect("device returned nil");
            Retained::cast_unchecked(po)
        }
    }

    /// Creates a fresh, empty [`CommandBuffer`].
    pub fn new_command_buffer(&self) -> Retained<crate::command_buffer::CommandBuffer> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::command_buffer::CommandBufferProtocol> = msg_send![self, commandBuffer];
            let po_ret: Retained<ProtocolObject<dyn crate::command_buffer::CommandBufferProtocol>> = Retained::retain_autoreleased(ptr).expect("commandBuffer returned nil");
            Retained::cast_unchecked::<crate::command_buffer::CommandBuffer>(po_ret)
        }
    }

    /// Creates a fresh [`CommandBuffer`] using the provided descriptor.
    pub fn new_command_buffer_with_descriptor(&self, descriptor: &crate::command_buffer_descriptor::CommandBufferDescriptor) -> Retained<crate::command_buffer::CommandBuffer> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::command_buffer::CommandBufferProtocol> = msg_send![self, commandBufferWithDescriptor: descriptor];
            let po_ret = Retained::retain_autoreleased(ptr).expect("commandBufferWithDescriptor returned nil");
            Retained::cast_unchecked(po_ret)
        }
    }

    /// Creates a fresh, empty [`CommandBuffer`] with unretained references.
    /// 
    /// Command buffers created this way do not hold strong references to any of the
    /// resources used within them. This can be useful to break retain cycles.
    pub fn new_command_buffer_with_unretained_references(&self) -> Retained<crate::command_buffer::CommandBuffer> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::command_buffer::CommandBufferProtocol> = msg_send![self, commandBufferWithUnretainedReferences];
            let po_ret = Retained::retain_autoreleased(ptr).expect("commandBufferWithUnretainedReferences returned nil");
            Retained::cast_unchecked(po_ret)
        }
    }

    /// Inserts a debug capture boundary into the command queue.
    ///
    /// This can be used with GPU frame capture tools (like Xcode's) to delineate sections of work.
    pub fn insert_debug_capture_boundary(&self) {
        unsafe {
            let _: () = msg_send![self, insertDebugCaptureBoundary];
        }
    }

    /// Create from retained protocol object (internal helper)
    #[allow(dead_code)]
    pub(crate) unsafe fn from_retained(po: Retained<ProtocolObject<dyn CommandQueueProtocol>>) -> Retained<Self> {
        // SAFETY: `po` came from Objective-C and is known to implement MTLCommandQueue.
        unsafe { Retained::cast_unchecked::<Self>(po) }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Device;
    use crate::CommandQueue;
    use crate::command_buffer_descriptor::CommandBufferDescriptor; // Import for descriptor

    #[test]
    fn test_command_queue_methods() { // Renamed test function for clarity
        objc2::rc::autoreleasepool(|_pool_token| { // Corrected autoreleasepool closure signature
            let device = Device::system_default().expect("Failed to get default system device");
            let queue: Retained<CommandQueue> = device.new_command_queue();
            // Getting a Retained<CommandQueue> implies it's not null due to expect() in new_command_queue.

            let queue_device: Retained<Device> = queue.device();
            // Compare the raw pointers for equality
            assert_eq!(Retained::as_ptr(&device), Retained::as_ptr(&queue_device), "Queue's device should match the creating device");

            // Test label
            let test_label = "MyTestQueue";
            queue.set_label(test_label);
            assert_eq!(queue.label().as_deref(), Some(test_label));

            // Test new_command_buffer (existence check by getting it)
            let _cmd_buf = queue.new_command_buffer();
            // Getting Retained<CommandBuffer> is enough of an assertion here.

            // Test new_command_buffer_with_unretained_references
            let _cmd_buf_unretained = queue.new_command_buffer_with_unretained_references();

            // Test insert_debug_capture_boundary (just call it)
            queue.insert_debug_capture_boundary();

            // Test new_command_buffer_with_descriptor
            let descriptor = CommandBufferDescriptor::new();
            let _cmd_buf_desc = queue.new_command_buffer_with_descriptor(&descriptor);
        });
    }
} 