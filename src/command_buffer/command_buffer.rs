use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol, NSString};

extern_protocol!(
    /// Opaque command buffer type for primary Metal command queues.
    #[name = "MTLCommandBuffer"]
    pub unsafe trait CommandBuffer: NSObjectProtocol {
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
    }
);
