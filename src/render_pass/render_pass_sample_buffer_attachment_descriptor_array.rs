use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::RenderPassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Array of sample buffer attachment descriptors.
    #[unsafe(super(NSObject))]
    #[name = "MTLRenderPassSampleBufferAttachmentDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RenderPassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for RenderPassSampleBufferAttachmentDescriptorArray {}
);

impl RenderPassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<RenderPassSampleBufferAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&RenderPassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}
