use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLDevice, MTLStages};

extern_protocol!(
    /// MTLCommandEncoder is the common interface for objects that write commands into MTLCommandBuffers.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLCommandEncoder` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLCommandEncoder: NSObjectProtocol {
        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Declare that all command generation from this encoder is complete, and detach from the MTLCommandBuffer.
        #[unsafe(method(endEncoding))]
        #[unsafe(method_family = none)]
        fn end_encoding(&self);

        /// Encodes a consumer barrier on work you commit to the same command queue.
        ///
        /// Encode a barrier that guarantees that any subsequent work you encode in the current command encoder that corresponds
        /// to the `beforeStages` stages doesn't proceed until Metal completes all work prior to the current command encoder
        /// corresponding to the `afterQueueStages` stages, completes.
        ///
        /// Metal can reorder the exact point where it applies the barrier, so use this method for synchronizing between different passes.
        /// If you need to synchronize work within a pass that you encode with an instance of a subclass of `MTLCommandEncoder`,
        /// use memory barriers instead. For subclasses of `MTL4CommandEncoder`, use encoder barriers.
        ///
        /// You can specify `afterQueueStages` and `beforeStages` that contain `MTLStages` unrelated to the current command encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(barrierAfterQueueStages:beforeStages:))]
        #[unsafe(method_family = none)]
        fn barrier_after_queue_stages_before_stages(
            &self,
            after_queue_stages: MTLStages,
            before_stages: MTLStages,
        );
    }
);

/// Convenience wrappers for the Objective-C string properties and debug methods.
pub trait MTLCommandEncoderExt: MTLCommandEncoder + Message {
    /// The device this resource was created against.
    fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>
    where
        Self: Sized,
    {
        unsafe { msg_send![self, device] }
    }

    /// A string to help identify this object.
    fn label(&self) -> Option<String>
    where
        Self: Sized,
    {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    /// Sets a string to help identify this object.
    fn set_label(
        &self,
        label: Option<&str>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    /// Inserts a debug string into the command buffer. This does not change any API behavior, but can be useful when debugging.
    fn insert_debug_signpost(
        &self,
        string: &str,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![self, insertDebugSignpost: &*NSString::from_str(string)];
        }
    }

    /// Push a new named string onto a stack of string labels.
    fn push_debug_group(
        &self,
        string: &str,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![self, pushDebugGroup: &*NSString::from_str(string)];
        }
    }

    /// Pop the latest named string off of the stack.
    fn pop_debug_group(&self)
    where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![self, popDebugGroup];
        }
    }
}

impl<T: MTLCommandEncoder + Message> MTLCommandEncoderExt for T {}
