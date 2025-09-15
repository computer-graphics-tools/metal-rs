use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::RenderPassColorAttachmentDescriptor;

extern_class!(
    /// Array of color attachment descriptors.
    #[unsafe(super(NSObject))]
    #[name = "MTLRenderPassColorAttachmentDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RenderPassColorAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for RenderPassColorAttachmentDescriptorArray {}
);

impl RenderPassColorAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<RenderPassColorAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&RenderPassColorAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}
