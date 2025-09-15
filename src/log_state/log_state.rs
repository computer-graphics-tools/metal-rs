use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// Log state for handling GPU log messages.
    #[name = "MTLLogState"]
    pub unsafe trait LogState: NSObjectProtocol + Send + Sync {
        /// Add a function block to handle log message output.
        /// Safety: The block must be sendable.
        #[unsafe(method(addLogHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn add_log_handler(
            &self,
            block: &block2::DynBlock<
                dyn Fn(*mut NSString, *mut NSString, super::LogLevel, core::ptr::NonNull<NSString>),
            >,
        );
    }
);
