use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::PipelineBufferDescriptor;

extern_class!(
    /// Apple's docs: `https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray?language=objc`
    #[unsafe(super(NSObject))]
    #[name = "MTLPipelineBufferDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PipelineBufferDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PipelineBufferDescriptorArray {}
);

impl PipelineBufferDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            buffer_index: usize,
        ) -> Retained<PipelineBufferDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            buffer: Option<&PipelineBufferDescriptor>,
            buffer_index: usize,
        );
    );
}
