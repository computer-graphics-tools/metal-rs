use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSUInteger};

use crate::io::{MTLIOCommandQueueType, MTLIOPriority};

extern_class!(
    /// Descriptor for creating an IO command queue.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIOCommandQueueDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLIOCommandQueueDescriptor {}
);

unsafe impl CopyingHelper for MTLIOCommandQueueDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLIOCommandQueueDescriptor {}
);

impl MTLIOCommandQueueDescriptor {
    extern_methods!(
        /// Maximum number of command buffers in flight.
        #[unsafe(method(maxCommandBufferCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_command_buffer_count(&self) -> NSUInteger;

        /// Setter for [`max_command_buffer_count`][Self::max_command_buffer_count].
        #[unsafe(method(setMaxCommandBufferCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_command_buffer_count(&self, count: NSUInteger);

        /// Priority of this queue.
        #[unsafe(method(priority))]
        #[unsafe(method_family = none)]
        pub unsafe fn priority(&self) -> MTLIOPriority;

        /// Setter for [`priority`][Self::priority].
        #[unsafe(method(setPriority:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_priority(&self, priority: MTLIOPriority);

        /// Type (serial or concurrent) of queue.
        #[unsafe(method(r#type))]
        #[unsafe(method_family = none)]
        pub unsafe fn queue_type(&self) -> MTLIOCommandQueueType;

        /// Setter for [`queue_type`][Self::queue_type].
        #[unsafe(method(setType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_queue_type(&self, t: MTLIOCommandQueueType);

        /// Maximum number of IO commands in flight.
        #[unsafe(method(maxCommandsInFlight))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_commands_in_flight(&self) -> NSUInteger;

        /// Setter for [`max_commands_in_flight`][Self::max_commands_in_flight].
        #[unsafe(method(setMaxCommandsInFlight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_commands_in_flight(&self, count: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLIOCommandQueueDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
