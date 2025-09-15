use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::RenderPipelineColorAttachmentDescriptor;

extern_class!(
    /// Array wrapper for color attachment descriptors.
    #[unsafe(super(NSObject))]
    #[name = "MTLRenderPipelineColorAttachmentDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RenderPipelineColorAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for RenderPipelineColorAttachmentDescriptorArray {}
);

impl RenderPipelineColorAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<RenderPipelineColorAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&RenderPipelineColorAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}
