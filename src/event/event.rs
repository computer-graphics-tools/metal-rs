use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// Bridged protocol for `MTLEvent`.
    pub unsafe trait MTLEvent: NSObjectProtocol + Send + Sync {
        /// The device this event can be used with. Will be nil when the event is shared across devices.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Option<Retained<ProtocolObject<dyn crate::MTLDevice>>>;

        /// Optional label.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);
    }
);
