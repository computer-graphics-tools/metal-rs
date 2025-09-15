use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::Mutability;

extern_class!(
    /// Apple's docs: `https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor?language=objc`
    #[unsafe(super(NSObject))]
    #[name = "MTLPipelineBufferDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PipelineBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for PipelineBufferDescriptor {}
);

unsafe impl CopyingHelper for PipelineBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for PipelineBufferDescriptor {}
);

impl PipelineBufferDescriptor {
    extern_methods!(
        /// Buffer mutability. Defaults to Mutability::Default: mutable for standard buffers, immutable for argument buffers
        #[unsafe(method(mutability))]
        #[unsafe(method_family = none)]
        pub fn mutability(&self) -> Mutability;

        /// Setter for [`mutability`][Self::mutability].
        #[unsafe(method(setMutability:))]
        #[unsafe(method_family = none)]
        pub fn set_mutability(&self, mutability: Mutability);
    );
}

/// Methods declared on superclass `NSObject`.
impl PipelineBufferDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
