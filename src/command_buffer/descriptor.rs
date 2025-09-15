use super::command_buffer::MTLCommandBufferErrorOption;
use crate::log_state::MTLLogState;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Properties for creating a `CommandBuffer`.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCommandBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLCommandBufferDescriptor {}
);

unsafe impl CopyingHelper for MTLCommandBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCommandBufferDescriptor {}
);

impl MTLCommandBufferDescriptor {
    extern_methods!(
        /// If true, holds strong references to objects needed for execution.
        #[unsafe(method(retainedReferences))]
        #[unsafe(method_family = none)]
        pub fn retained_references(&self) -> bool;

        #[unsafe(method(setRetainedReferences:))]
        #[unsafe(method_family = none)]
        pub fn set_retained_references(&self, retained: bool);

        /// Options configuring error reporting of the created command buffer.
        #[unsafe(method(errorOptions))]
        #[unsafe(method_family = none)]
        pub fn error_options(&self) -> MTLCommandBufferErrorOption;

        #[unsafe(method(setErrorOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_error_options(&self, opts: MTLCommandBufferErrorOption);

        /// Contains information related to shader logging.
        #[unsafe(method(logState))]
        #[unsafe(method_family = none)]
        pub fn log_state(&self) -> Option<Retained<ProtocolObject<dyn MTLLogState>>>;

        #[unsafe(method(setLogState:))]
        #[unsafe(method_family = none)]
        pub fn set_log_state(&self, log_state: Option<&ProtocolObject<dyn MTLLogState>>);
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
