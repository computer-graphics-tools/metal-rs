use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLCommandQueue, MTLDevice};

extern_protocol!(
    /// Capture scope to bracket GPU work for debugging captures.
    pub unsafe trait MTLCaptureScope: NSObjectProtocol {
        #[unsafe(method(beginScope))]
        #[unsafe(method_family = none)]
        fn begin_scope(&self);

        #[unsafe(method(endScope))]
        #[unsafe(method_family = none)]
        fn end_scope(&self);

        /// Scope label.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        unsafe fn set_label(&self, label: Option<&NSString>);

        /// Associated device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// If set, only capture commands from the associated command queue.
        #[unsafe(method(commandQueue))]
        #[unsafe(method_family = none)]
        unsafe fn command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;
    }
);
