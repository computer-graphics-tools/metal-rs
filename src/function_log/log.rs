use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use super::{MTLFunctionLogDebugLocation, MTLFunctionLogType};
use crate::library::MTLFunction;

extern_protocol!(
    /// Function log information (from `MTLFunctionLog`).
    pub unsafe trait MTLFunctionLog: NSObjectProtocol {
        #[unsafe(method(r#type))]
        #[unsafe(method_family = none)]
        unsafe fn r#type(&self) -> MTLFunctionLogType;

        #[unsafe(method(encoderLabel))]
        #[unsafe(method_family = none)]
        unsafe fn encoder_label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(function))]
        #[unsafe(method_family = none)]
        unsafe fn function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        #[unsafe(method(debugLocation))]
        #[unsafe(method_family = none)]
        unsafe fn debug_location(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionLogDebugLocation>>>;
    }
);
