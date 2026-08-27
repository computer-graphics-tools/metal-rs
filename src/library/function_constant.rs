use objc2::{extern_class, extern_conformance, extern_methods, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::MTLDataType;

extern_class!(
    /// Reflection information for a Metal function constant.
    ///
    /// Availability: macOS 10.12+, iOS 10.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionConstant;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionConstant {}
);

impl MTLFunctionConstant {
    extern_methods!(
        /// The function constant's data type.
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> MTLDataType;

        /// The function constant's index.
        #[unsafe(method(index))]
        #[unsafe(method_family = none)]
        pub fn index(&self) -> usize;

        /// Whether callers need to provide a value for this function constant.
        #[unsafe(method(required))]
        #[unsafe(method_family = none)]
        pub fn required(&self) -> bool;
    );

    /// The function constant's name.
    pub fn name(&self) -> String {
        let name: Retained<NSString> = unsafe { msg_send![self, name] };
        name.to_string()
    }
}
