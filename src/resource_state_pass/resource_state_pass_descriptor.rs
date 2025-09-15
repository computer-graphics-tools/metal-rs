use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::sample_buffer_attachment_descriptor_array::ResourceStatePassSampleBufferAttachmentDescriptorArray;

extern_class!(
    /// Resource state pass descriptor
    #[unsafe(super(NSObject))]
    #[name = "MTLResourceStatePassDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ResourceStatePassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for ResourceStatePassDescriptor {}
);

unsafe impl CopyingHelper for ResourceStatePassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for ResourceStatePassDescriptor {}
);

impl ResourceStatePassDescriptor {
    extern_methods!(
        /// Create a default resource state pass descriptor
        #[unsafe(method(resourceStatePassDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn resource_state_pass_descriptor() -> Retained<ResourceStatePassDescriptor>;

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_buffer_attachments(
            &self,
        ) -> Retained<ResourceStatePassSampleBufferAttachmentDescriptorArray>;
    );
}
