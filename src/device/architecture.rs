use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

extern_class!(
    /// Contains information about the device's architecture
    ///
    /// Apple's docs: `https://developer.apple.com/documentation/metal/mtlarchitecture?language=objc`
    #[unsafe(super(NSObject))]
    #[name = "MTLArchitecture"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Architecture;
);

extern_conformance!(
    unsafe impl NSCopying for Architecture {}
);

unsafe impl CopyingHelper for Architecture {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for Architecture {}
);

impl Architecture {
    extern_methods!(
        /// The device's architecture name.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;
    );
}

/// Methods declared on superclass `NSObject`.
impl Architecture {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
