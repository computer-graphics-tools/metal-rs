use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{AttributeDescriptorArray, BufferLayoutDescriptorArray};

extern_class!(
    /// Stage input/output descriptor
    #[unsafe(super(NSObject))]
    #[name = "MTLStageInputOutputDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct StageInputOutputDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for StageInputOutputDescriptor {}
);

unsafe impl CopyingHelper for StageInputOutputDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for StageInputOutputDescriptor {}
);

impl StageInputOutputDescriptor {
    extern_methods!(
        #[unsafe(method(stageInputOutputDescriptor))]
        #[unsafe(method_family = none)]
        pub fn stage_input_output_descriptor() -> Retained<StageInputOutputDescriptor>;

        #[unsafe(method(layouts))]
        #[unsafe(method_family = none)]
        pub fn layouts(&self) -> Retained<BufferLayoutDescriptorArray>;

        #[unsafe(method(attributes))]
        #[unsafe(method_family = none)]
        pub fn attributes(&self) -> Retained<AttributeDescriptorArray>;

        #[unsafe(method(indexBufferIndex))]
        #[unsafe(method_family = none)]
        pub fn index_buffer_index(&self) -> usize;

        /// Setter for [`index_buffer_index`][Self::index_buffer_index].
        #[unsafe(method(setIndexBufferIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_index_buffer_index(&self, index_buffer_index: usize);

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}
