use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{NSObjectProtocol, NSUInteger};

use super::MTLTileRenderPipelineColorAttachmentDescriptor;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTileRenderPipelineColorAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTileRenderPipelineColorAttachmentDescriptorArray {}
);

impl MTLTileRenderPipelineColorAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Retained<MTLTileRenderPipelineColorAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: &MTLTileRenderPipelineColorAttachmentDescriptor,
            attachment_index: NSUInteger,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTileRenderPipelineColorAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
