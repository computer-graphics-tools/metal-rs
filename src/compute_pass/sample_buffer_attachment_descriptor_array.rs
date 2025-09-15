use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::ComputePassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Array of `ComputePassSampleBufferAttachmentDescriptor` objects.
    #[unsafe(super(NSObject))]
    #[name = "MTLComputePassSampleBufferAttachmentDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ComputePassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ComputePassSampleBufferAttachmentDescriptorArray {}
);

impl ComputePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<ComputePassSampleBufferAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&ComputePassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}

impl ComputePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
