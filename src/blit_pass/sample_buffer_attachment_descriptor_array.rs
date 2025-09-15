use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLBlitPassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Array of `BlitPassSampleBufferAttachmentDescriptor` objects.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBlitPassSampleBufferAttachmentDescriptorArray {}
);

impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<MTLBlitPassSampleBufferAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&MTLBlitPassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}

impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
