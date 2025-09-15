use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::BlitPassSampleBufferAttachmentDescriptorArray;

extern_class!(
    /// Represents a collection of attachments used to create a blit command encoder.
    #[unsafe(super(NSObject))]
    #[name = "MTLBlitPassDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BlitPassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for BlitPassDescriptor {}
);

unsafe impl CopyingHelper for BlitPassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for BlitPassDescriptor {}
);

impl BlitPassDescriptor {
    extern_methods!(
        /// Create a default blit pass descriptor.
        #[unsafe(method(blitPassDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn blit_pass_descriptor() -> Retained<BlitPassDescriptor>;

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub fn sample_buffer_attachments(
            &self,
        ) -> Retained<BlitPassSampleBufferAttachmentDescriptorArray>;
    );
}

impl BlitPassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
