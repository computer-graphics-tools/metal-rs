use objc2::{extern_protocol, rc::Retained};
use objc2_foundation::{NSObjectProtocol, NSString, NSUInteger, NSURL};

extern_protocol!(
    /// Location information for a function log (from `MTLFunctionLogDebugLocation`).
    pub unsafe trait MTLFunctionLogDebugLocation: NSObjectProtocol {
        #[unsafe(method(functionName))]
        #[unsafe(method_family = none)]
        unsafe fn function_name(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(URL))]
        #[unsafe(method_family = none)]
        unsafe fn url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(line))]
        #[unsafe(method_family = none)]
        unsafe fn line(&self) -> NSUInteger;

        #[unsafe(method(column))]
        #[unsafe(method_family = none)]
        unsafe fn column(&self) -> NSUInteger;
    }
);
