use objc2::{extern_class, extern_conformance, extern_methods, rc::{Allocated, Retained}, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSUInteger};

use crate::io::{IoCommandQueueType, IoPriority};

extern_class!(
    /// Descriptor for creating an IO command queue.
    #[unsafe(super(NSObject))]
    #[name = "MTLIOCommandQueueDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct IoCommandQueueDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for IoCommandQueueDescriptor {}
);

unsafe impl CopyingHelper for IoCommandQueueDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for IoCommandQueueDescriptor {}
);

impl IoCommandQueueDescriptor {
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
        pub unsafe fn priority(&self) -> IoPriority;

        /// Setter for [`priority`][Self::priority].
        #[unsafe(method(setPriority:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_priority(&self, priority: IoPriority);

        /// Type (serial or concurrent) of queue.
        #[unsafe(method(r#type))]
        #[unsafe(method_family = none)]
        pub unsafe fn queue_type(&self) -> IoCommandQueueType;

        /// Setter for [`queue_type`][Self::queue_type].
        #[unsafe(method(setType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_queue_type(&self, t: IoCommandQueueType);

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
impl IoCommandQueueDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}


