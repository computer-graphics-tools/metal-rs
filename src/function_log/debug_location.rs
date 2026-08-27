use objc2::{Message, extern_protocol, msg_send, rc::Retained};
use objc2_foundation::{NSObjectProtocol, NSString, NSURL};

extern_protocol!(
    /// Location information for a function log (from `MTLFunctionLogDebugLocation`).
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    pub unsafe trait MTLFunctionLogDebugLocation: NSObjectProtocol {
        #[unsafe(method(line))]
        #[unsafe(method_family = none)]
        fn line(&self) -> usize;

        #[unsafe(method(column))]
        #[unsafe(method_family = none)]
        fn column(&self) -> usize;
    }
);

#[allow(unused)]
pub trait MTLFunctionLogDebugLocationExt: MTLFunctionLogDebugLocation + Message {
    /// The URL string for the source location.
    fn url(&self) -> Option<String>
    where
        Self: Sized,
    {
        let url: Option<Retained<NSURL>> = unsafe { msg_send![self, URL] };
        url.and_then(|url| url.absoluteString()).map(|url| url.to_string())
    }

    /// The name of the faulting function.
    fn function_name(&self) -> Option<String>
    where
        Self: Sized,
    {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, functionName] };
        s.map(|v| v.to_string())
    }
}

impl<T: MTLFunctionLogDebugLocation + Message> MTLFunctionLogDebugLocationExt for T {}
