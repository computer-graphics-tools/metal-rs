use objc2::extern_protocol;
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// Log state for handling GPU log messages.
    pub unsafe trait MTLLogState: NSObjectProtocol + Send + Sync {
        /// Add a function block to handle log message output.
        /// Safety: The block must be sendable.
        #[unsafe(method(addLogHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn add_log_handler(
            &self,
            block: &block2::DynBlock<
                dyn Fn(
                    *mut NSString,
                    *mut NSString,
                    super::MTLLogLevel,
                    core::ptr::NonNull<NSString>,
                ),
            >,
        );
    }
);
