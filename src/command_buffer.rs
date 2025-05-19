//! Wrapper for `id<MTLCommandBuffer>` – a GPU work submission container.

use crate::prelude::*;
use objc2::encode::{Encoding, RefEncode};
use objc2::Message;
use block2::StackBlock;

/// Specifies the execution status of a command buffer.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

/// Describes the type of error that a command buffer can encounter.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive] // New error codes might be added in future OS versions.
#[allow(dead_code)] // Will be used more extensively with error handling
pub enum CommandBufferError {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4, // Also known as Blacklisted (deprecated name)
    // Blacklisted = 4, // Deprecated, use AccessRevoked
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,      // macOS 10.14+, iOS 12+
    DeviceRemoved = 11,   // macOS 10.14+, iOS 12+
    StackOverflow = 12,   // macOS 11.0+, iOS 14+
    /// An unknown error code reported by the system.
    Unknown(u64),
}

impl CommandBufferError {
    #[allow(dead_code)] // Will be used more extensively with error handling
    pub fn from_u64(value: u64) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Internal,
            2 => Self::Timeout,
            3 => Self::PageFault,
            4 => Self::AccessRevoked,
            7 => Self::NotPermitted,
            8 => Self::OutOfMemory,
            9 => Self::InvalidResource,
            10 => Self::Memoryless,
            11 => Self::DeviceRemoved,
            12 => Self::StackOverflow,
            other => Self::Unknown(other),
        }
    }
}

extern_protocol!(pub unsafe trait CommandBufferProtocol: NSObjectProtocol {
});

/// Thin wrapper over `id<MTLCommandBuffer>`.
#[repr(transparent)]
pub struct CommandBuffer(ProtocolObject<dyn CommandBufferProtocol>);

impl core::ops::Deref for CommandBuffer {
    type Target = ProtocolObject<dyn CommandBufferProtocol>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl CommandBuffer {
    /// The device that created this command buffer.
    pub fn device(&self) -> Retained<crate::device::Device> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::device::DeviceProtocol> = msg_send![self, device];
            let po = Retained::retain_autoreleased(ptr).expect("commandBuffer.device returned nil");
            Retained::cast_unchecked(po)
        }
    }

    /// The command queue that this command buffer was created from.
    pub fn command_queue(&self) -> Retained<crate::command_queue::CommandQueue> {
        unsafe {
            let ptr: *mut ProtocolObject<dyn crate::command_queue::CommandQueueProtocol> = msg_send![self, commandQueue];
            let po = Retained::retain_autoreleased(ptr).expect("commandBuffer.commandQueue returned nil");
            Retained::cast_unchecked(po)
        }
    }

    /// Indicates whether this command buffer retains references to its resources.
    pub fn retained_references(&self) -> bool {
        unsafe { msg_send![self, retainedReferences] }
    }

    /// The error reporting options for this command buffer.
    pub fn error_options(&self) -> crate::command_buffer_descriptor::CommandBufferErrorOption {
        unsafe {
            let raw_opts: u64 = msg_send![self, errorOptions];
            crate::command_buffer_descriptor::CommandBufferErrorOption::from_bits_retain(raw_opts)
        }
    }

    /// A human-readable string identifying the command buffer.
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_str: *mut NSString = msg_send![self, label];
            Retained::retain_autoreleased(ns_str).map(|s| s.to_string())
        }
    }

    /// Sets a human-readable string to identify the command buffer.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_label = NSString::from_str(label);
            let _: () = msg_send![self, setLabel: &*ns_label];
        }
    }

    /// Enqueues the command buffer for execution on the command queue.
    ///
    /// After this call, the command buffer's `status` becomes `Enqueued`.
    /// You must call `enqueue` before `commit` if you are managing enqueueing manually.
    /// Typically, `commit` also handles enqueueing if the buffer is not already enqueued.
    pub fn enqueue(&self) {
        unsafe { msg_send![self, enqueue] }
    }

    /// The time, in seconds, when the command buffer's execution began on the kernel side.
    pub fn kernel_start_time(&self) -> f64 {
        unsafe { msg_send![self, kernelStartTime] }
    }

    /// The time, in seconds, when the command buffer's execution finished on the kernel side.
    pub fn kernel_end_time(&self) -> f64 {
        unsafe { msg_send![self, kernelEndTime] }
    }

    /// The time, in seconds, when the GPU started executing the command buffer.
    pub fn gpu_start_time(&self) -> f64 {
        unsafe { msg_send![self, GPUStartTime] }
    }

    /// The time, in seconds, when the GPU finished executing the command buffer.
    pub fn gpu_end_time(&self) -> f64 {
        unsafe { msg_send![self, GPUEndTime] }
    }

    /// Blocks execution of the current thread until the command buffer is scheduled.
    pub fn wait_until_scheduled(&self) {
        unsafe { msg_send![self, waitUntilScheduled] }
    }

    /// Adds a handler to be called when the command buffer is scheduled.
    ///
    /// The handler block is copied by the command buffer.
    pub fn add_scheduled_handler<F>(&self, handler: F)
    where
        F: Fn(&Self) + Send + Sync + 'static,
    {
        let handler_arc = std::sync::Arc::new(handler);
        let block = StackBlock::new(move |buffer: &Self| {
            let handler_clone = handler_arc.clone();
            handler_clone(buffer);
        }).copy();
        unsafe {
            let _: () = msg_send![self, addScheduledHandler: &*block];
        }
    }

    /// Adds a handler to be called when the command buffer has completed execution.
    ///
    /// The handler block is copied by the command buffer.
    pub fn add_completed_handler<F>(&self, handler: F)
    where
        F: Fn(&Self) + Send + Sync + 'static,
    {
        let handler_arc = std::sync::Arc::new(handler);
        let block = StackBlock::new(move |buffer: &Self| {
            let handler_clone = handler_arc.clone();
            handler_clone(buffer);
        }).copy();
        unsafe {
            let _: () = msg_send![self, addCompletedHandler: &*block];
        }
    }

    /// Enqueues the command buffer for execution.
    pub fn commit(&self) {
        unsafe { msg_send![self, commit] }
    }

    /// Blocks the current thread until GPU execution completes. Use only for
    /// testing – never in performance-critical code.
    pub fn wait_until_completed(&self) {
        unsafe { msg_send![self, waitUntilCompleted] }
    }

    /// Returns the error domain status enumeration value after GPU execution.
    pub fn status(&self) -> CommandBufferStatus {
        unsafe {
            let raw_status: u64 = msg_send![self, status];
            // SAFETY: Assuming the raw_status from Metal directly maps to CommandBufferStatus variants.
            // Consider adding a from_u64 method with validation if necessary.
            core::mem::transmute(raw_status) 
        }
    }

    /// If an error occurred during execution, this property holds an object that describes the error.
    ///
    /// Returns `None` if no error occurred or if the command buffer has not yet completed execution.
    pub fn error(&self) -> Option<Retained<objc2_foundation::NSError>> {
        unsafe {
            let err_ptr: *mut objc2_foundation::NSError = msg_send![self, error];
            // Retained::retain_autoreleased itself returns Option<Retained<T>>
            // and handles the null check internally.
            Retained::retain_autoreleased(err_ptr)
        }
    }
}

unsafe impl Message for CommandBuffer {}
unsafe impl RefEncode for CommandBuffer {
    const ENCODING_REF: Encoding = <ProtocolObject<dyn CommandBufferProtocol>>::ENCODING_REF;
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module (CommandBuffer, CommandBufferStatus, etc.)
    use crate::Device;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_command_buffer_properties_and_handlers() {
        objc2::rc::autoreleasepool(|_pool_token| {
            let device = Device::system_default().expect("Failed to get default system device");
            let queue = device.new_command_queue();
            let cmd_buf = queue.new_command_buffer();

            // Test device and commandQueue properties
            assert_eq!(Retained::as_ptr(&cmd_buf.device()), Retained::as_ptr(&device));
            assert_eq!(Retained::as_ptr(&cmd_buf.command_queue()), Retained::as_ptr(&queue));

            // Test label
            let test_label = "MyCommandBuffer";
            cmd_buf.set_label(test_label);
            assert_eq!(cmd_buf.label().as_deref(), Some(test_label));

            // Test status (initial should be NotEnqueued)
            // Note: Directly after creation, status might be 'NotEnqueued'.
            // Actual status changes require enqueueing/committing and GPU interaction.
            // For this unit test, we just check if the call works and returns a valid enum.
            let initial_status = cmd_buf.status();
            assert_eq!(initial_status, CommandBufferStatus::NotEnqueued);
            
            // Test error (should be None initially)
            assert!(cmd_buf.error().is_none());

            // Test time properties (should be 0.0 initially or some default)
            assert_eq!(cmd_buf.kernel_start_time(), 0.0);
            assert_eq!(cmd_buf.kernel_end_time(), 0.0);
            assert_eq!(cmd_buf.gpu_start_time(), 0.0);
            assert_eq!(cmd_buf.gpu_end_time(), 0.0);

            // Test retainedReferences and errorOptions (default values)
            // These depend on how the command buffer was created (descriptor vs default)
            // Default new_command_buffer usually means retainedReferences = true, errorOptions = None
            assert_eq!(cmd_buf.retained_references(), true);
            assert_eq!(cmd_buf.error_options(), crate::command_buffer_descriptor::CommandBufferErrorOption::NONE);

            // Test handlers (functional test is hard without real GPU work)
            // Just test if they can be added.
            let scheduled_called = Arc::new(Mutex::new(false));
            let scheduled_called_clone = scheduled_called.clone();
            cmd_buf.add_scheduled_handler(move |_cb| {
                *scheduled_called_clone.lock().unwrap() = true;
            });

            let completed_called = Arc::new(Mutex::new(false));
            let completed_called_clone = completed_called.clone();
            cmd_buf.add_completed_handler(move |_cb| {
                *completed_called_clone.lock().unwrap() = true;
            });

            // Enqueue and commit (these calls don't block for completion in this basic form)
            cmd_buf.enqueue();
            assert_eq!(cmd_buf.status(), CommandBufferStatus::Enqueued);
            
            cmd_buf.commit(); // This will also enqueue if not already.
            // Status after commit could be Committed or Scheduled quickly.
            // For a robust test of status changes, one would need to wait or use handlers.
        });
    }
} 