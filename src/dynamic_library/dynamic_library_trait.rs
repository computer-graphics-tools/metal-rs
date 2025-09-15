use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol, NSString, NSURL};

use crate::device::Device;

extern_protocol!(
    /// A container for the binary representation of code compiled for a Device.
    ///
    /// Mirrors `MTLDynamicLibrary`.
    #[name = "MTLDynamicLibrary"]
    pub unsafe trait DynamicLibrary: NSObjectProtocol + Send + Sync {
        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);

        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn Device>>;

        /// The install name of this dynamic library. Cannot be nil.
        #[unsafe(method(installName))]
        #[unsafe(method_family = none)]
        fn install_name(&self) -> Retained<NSString>;

        /// Writes the contents of the dynamic library to a file.
        #[unsafe(method(serializeToURL:error:_))]
        #[unsafe(method_family = none)]
        fn serialize_to_url(&self, url: &NSURL) -> Result<(), Retained<NSError>>;
    }
);


