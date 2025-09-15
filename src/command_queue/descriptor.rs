use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSUInteger};

use crate::log_state::LogState;

extern_class!(
    /// Descriptor for creating a `CommandQueue`.
    #[unsafe(super(NSObject))]
    #[name = "MTLCommandQueueDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CommandQueueDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for CommandQueueDescriptor {}
);

unsafe impl CopyingHelper for CommandQueueDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CommandQueueDescriptor {}
);

impl CommandQueueDescriptor {
    extern_methods!(
        /// Specify upper bound on uncompleted command buffers that may be enqueued on this queue.
        #[unsafe(method(maxCommandBufferCount))]
        #[unsafe(method_family = none)]
        pub fn max_command_buffer_count(&self) -> NSUInteger;

        #[unsafe(method(setMaxCommandBufferCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_command_buffer_count(&self, count: NSUInteger);

        /// Specify the `LogState` to enable shader logging.
        #[unsafe(method(logState))]
        #[unsafe(method_family = none)]
        pub fn log_state(&self) -> Option<Retained<ProtocolObject<dyn LogState>>>;

        #[unsafe(method(setLogState:))]
        #[unsafe(method_family = none)]
        pub fn set_log_state(&self, log_state: Option<&ProtocolObject<dyn LogState>>);
    );
}

impl CommandQueueDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
