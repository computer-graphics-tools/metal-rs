use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{AnyObject, NSObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSURL};

use crate::capture::types::CaptureDestination;

extern_class!(
    /// Parameters for starting a capture.
    #[unsafe(super(NSObject))]
    #[name = "MTLCaptureDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CaptureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for CaptureDescriptor {}
);

unsafe impl CopyingHelper for CaptureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CaptureDescriptor {}
);

impl CaptureDescriptor {
    extern_methods!(
        /// The object that is captured (Device/CommandQueue/CaptureScope).
        #[unsafe(method(captureObject))]
        #[unsafe(method_family = none)]
        pub fn capture_object(&self) -> Option<Retained<AnyObject>>;

        /// Safety: `capture_object` should be of the correct type.
        #[unsafe(method(setCaptureObject:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_capture_object(&self, capture_object: Option<&AnyObject>);

        /// The destination where to capture the GPU trace.
        #[unsafe(method(destination))]
        #[unsafe(method_family = none)]
        pub fn destination(&self) -> CaptureDestination;

        #[unsafe(method(setDestination:))]
        #[unsafe(method_family = none)]
        pub fn set_destination(&self, destination: CaptureDestination);

        /// Output URL if using `GPUTraceDocument`.
        #[unsafe(method(outputURL))]
        #[unsafe(method_family = none)]
        pub fn output_url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(setOutputURL:))]
        #[unsafe(method_family = none)]
        pub fn set_output_url(&self, output_url: Option<&NSURL>);
    );
}

impl CaptureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
