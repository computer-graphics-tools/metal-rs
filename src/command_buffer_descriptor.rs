use bitflags::bitflags;
use crate::prelude::*;
use objc2_foundation::NSObject;
use objc2::{extern_class, msg_send, msg_send_id};

bitflags! {
    /// Options for configuring error reporting on a command buffer.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct CommandBufferErrorOption: u64 {
        /// No options.
        const NONE = 0;
        /// Report errors from the encoder execution status.
        const ENCODER_EXECUTION_STATUS = 1;
    }
}

extern_class!(
    /// Describes the configuration of a new command buffer.
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[unsafe(super = NSObject)]
    #[name = "MTLCommandBufferDescriptor"]
    pub struct CommandBufferDescriptor;
);

impl CommandBufferDescriptor {
    /// Creates a new, default command buffer descriptor.
    pub fn new() -> Retained<Self> {
        unsafe { msg_send_id![Self::class(), new] }
    }

    /// Indicates whether the command buffer should retain references to resources.
    pub fn retained_references(&self) -> bool {
        unsafe { msg_send![self, retainedReferences] }
    }

    /// Sets whether the command buffer should retain references to resources.
    pub fn set_retained_references(&self, retained_references: bool) {
        unsafe { msg_send![self, setRetainedReferences: retained_references] }
    }

    /// The error reporting options for the command buffer.
    pub fn error_options(&self) -> CommandBufferErrorOption {
        unsafe {
            let raw_opts: u64 = msg_send![self, errorOptions];
            CommandBufferErrorOption::from_bits_retain(raw_opts)
        }
    }

    /// Sets the error reporting options for the command buffer.
    pub fn set_error_options(&self, error_options: CommandBufferErrorOption) {
        unsafe { msg_send![self, setErrorOptions: error_options.bits()] }
    }

    // TODO: Add logState and setLogState methods once LogState is available.
    // pub fn log_state(&self) -> Option<Retained<crate::log_state::LogState>> { ... }
} 