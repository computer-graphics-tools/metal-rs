use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::sample_buffer_attachment_descriptor::ResourceStatePassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Array of sample buffer attachment descriptors
    #[unsafe(super(NSObject))]
    #[name = "MTLResourceStatePassSampleBufferAttachmentDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ResourceStatePassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ResourceStatePassSampleBufferAttachmentDescriptorArray {}
);

impl ResourceStatePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<ResourceStatePassSampleBufferAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&ResourceStatePassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl ResourceStatePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
