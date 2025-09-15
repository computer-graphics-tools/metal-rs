use crate::drawable::MTLDrawable;
use crate::function_log::MTLLogContainer;
use objc2::{Encode, Encoding, RefEncode, extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSErrorDomain, NSObjectProtocol, NSString};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

unsafe impl Encode for MTLCommandBufferStatus {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCommandBufferStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    static MTLCommandBufferErrorDomain: &'static NSErrorDomain;
}
#[inline]
pub fn command_buffer_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLCommandBufferErrorDomain }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCommandBufferError {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    DeviceRemoved = 11,
    StackOverflow = 12,
}

unsafe impl Encode for MTLCommandBufferError {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCommandBufferError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MTLCommandBufferErrorOption: u64 { const None = 0; const EncoderExecutionStatus = 1<<0; }
}
unsafe impl Encode for MTLCommandBufferErrorOption {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCommandBufferErrorOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// Opaque command buffer type for primary Metal command queues.
    pub unsafe trait MTLCommandBuffer: NSObjectProtocol {
        #[unsafe(method(commit))]
        #[unsafe(method_family = none)]
        fn commit(&self);

        #[unsafe(method(waitUntilCompleted))]
        #[unsafe(method_family = none)]
        fn wait_until_completed(&self);

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);

        #[unsafe(method(error))]
        #[unsafe(method_family = none)]
        fn error(&self) -> Option<Retained<NSError>>;

        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        fn status(&self) -> MTLCommandBufferStatus;

        #[unsafe(method(logs))]
        #[unsafe(method_family = none)]
        unsafe fn logs(&self) -> Retained<ProtocolObject<dyn MTLLogContainer>>;

        // Completion handler API omitted in this port.

        #[unsafe(method(presentDrawable:))]
        #[unsafe(method_family = none)]
        fn present_drawable(&self, drawable: &ProtocolObject<dyn MTLDrawable>);
    }
);
