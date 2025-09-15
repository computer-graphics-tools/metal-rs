use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::TileRenderPipelineColorAttachmentDescriptor;

extern_class!(
    /// Array wrapper for tile color attachment descriptors.
    #[unsafe(super(NSObject))]
    #[name = "MTLTileRenderPipelineColorAttachmentDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TileRenderPipelineColorAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for TileRenderPipelineColorAttachmentDescriptorArray {}
);

impl TileRenderPipelineColorAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<TileRenderPipelineColorAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&TileRenderPipelineColorAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}


