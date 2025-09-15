use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::Device;

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlfence?language=objc`
    #[name = "MTLFence"]
    pub unsafe trait Fence: NSObjectProtocol + Send + Sync {
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn Device>>;

        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label]. This is copied when set.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);
    }
);
