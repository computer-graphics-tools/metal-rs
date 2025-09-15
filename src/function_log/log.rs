use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use super::{FunctionLogDebugLocation, FunctionLogType};
use crate::library::Function;

extern_protocol!(
    /// Function log information (from `MTLFunctionLog`).
    #[name = "MTLFunctionLog"]
    pub unsafe trait FunctionLog: NSObjectProtocol {
        #[unsafe(method(r#type))]
        #[unsafe(method_family = none)]
        unsafe fn r#type(&self) -> FunctionLogType;

        #[unsafe(method(encoderLabel))]
        #[unsafe(method_family = none)]
        unsafe fn encoder_label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(function))]
        #[unsafe(method_family = none)]
        unsafe fn function(&self) -> Option<Retained<ProtocolObject<dyn Function>>>;

        #[unsafe(method(debugLocation))]
        #[unsafe(method_family = none)]
        unsafe fn debug_location(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn FunctionLogDebugLocation>>>;
    }
);
