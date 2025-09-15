use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use super::Stages;
use crate::device::Device;

extern_protocol!(
    /// Common interface for objects that write commands into command buffers.
    #[name = "MTLCommandEncoder"]
    pub unsafe trait CommandEncoder: NSObjectProtocol {
        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn Device>>;

        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);

        /// Declare that all command generation from this encoder is complete.
        #[unsafe(method(endEncoding))]
        #[unsafe(method_family = none)]
        fn end_encoding(&self);

        /// Encodes a consumer barrier on work you commit to the same command queue.
        #[unsafe(method(barrierAfterQueueStages:beforeStages:))]
        #[unsafe(method_family = none)]
        unsafe fn barrier_after_queue_stages_before_stages(
            &self,
            after_queue_stages: Stages,
            before_stages: Stages,
        );

        /// Inserts a debug string into the command buffer.
        #[unsafe(method(insertDebugSignpost:))]
        #[unsafe(method_family = none)]
        fn insert_debug_signpost(&self, string: &NSString);

        /// Push a new named string onto a stack of string labels.
        #[unsafe(method(pushDebugGroup:))]
        #[unsafe(method_family = none)]
        fn push_debug_group(&self, string: &NSString);

        /// Pop the latest named string off of the stack.
        #[unsafe(method(popDebugGroup))]
        #[unsafe(method_family = none)]
        fn pop_debug_group(&self);
    }
);
