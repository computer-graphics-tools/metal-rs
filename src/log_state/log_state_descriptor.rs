use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSInteger, NSObjectProtocol};

use super::LogLevel;

extern_class!(
    /// Descriptor for creating a `LogState`.
    #[unsafe(super(NSObject))]
    #[name = "MTLLogStateDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LogStateDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for LogStateDescriptor {}
);

unsafe impl CopyingHelper for LogStateDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for LogStateDescriptor {}
);

impl LogStateDescriptor {
    extern_methods!(
        /// Minimum level of logs that will be printed.
        #[unsafe(method(level))]
        #[unsafe(method_family = none)]
        pub fn level(&self) -> LogLevel;

        #[unsafe(method(setLevel:))]
        #[unsafe(method_family = none)]
        pub fn set_level(&self, level: LogLevel);

        /// Size of the GPU buffer for shader logging (minimum 1KB).
        #[unsafe(method(bufferSize))]
        #[unsafe(method_family = none)]
        pub fn buffer_size(&self) -> isize;

        #[unsafe(method(setBufferSize:))]
        #[unsafe(method_family = none)]
        pub fn set_buffer_size(&self, buffer_size: isize);
    );
}

impl LogStateDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
