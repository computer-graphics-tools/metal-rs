use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLComputePassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Array of `ComputePassSampleBufferAttachmentDescriptor` objects.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptorArray {}
);

impl MTLComputePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<MTLComputePassSampleBufferAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&MTLComputePassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}

impl MTLComputePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
