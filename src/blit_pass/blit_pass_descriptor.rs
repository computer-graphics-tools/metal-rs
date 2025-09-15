use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLBlitPassSampleBufferAttachmentDescriptorArray;

extern_class!(
    /// Represents a collection of attachments used to create a blit command encoder.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLBlitPassDescriptor {}
);

unsafe impl CopyingHelper for MTLBlitPassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBlitPassDescriptor {}
);

impl MTLBlitPassDescriptor {
    extern_methods!(
        /// Create a default blit pass descriptor.
        #[unsafe(method(blitPassDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn blit_pass_descriptor() -> Retained<MTLBlitPassDescriptor>;

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub fn sample_buffer_attachments(
            &self,
        ) -> Retained<MTLBlitPassSampleBufferAttachmentDescriptorArray>;
    );
}

impl MTLBlitPassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
