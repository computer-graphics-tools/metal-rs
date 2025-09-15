use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use crate::library::Function;
use crate::{PipelineBufferDescriptorArray, StageInputOutputDescriptor};

extern_class!(
    /// Descriptor for creating a `ComputePipelineState`.
    #[unsafe(super(NSObject))]
    #[name = "MTLComputePipelineDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ComputePipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for ComputePipelineDescriptor {}
);

unsafe impl CopyingHelper for ComputePipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for ComputePipelineDescriptor {}
);

impl ComputePipelineDescriptor {
    extern_methods!(
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);

        /// The function to use with the `ComputePipelineState`.
        #[unsafe(method(computeFunction))]
        #[unsafe(method_family = none)]
        pub fn compute_function(&self) -> Option<Retained<ProtocolObject<dyn Function>>>;

        #[unsafe(method(setComputeFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_compute_function(&self, compute_function: Option<&ProtocolObject<dyn Function>>);

        /// Optional property. If not set, returns zero.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn max_total_threads_per_threadgroup(&self) -> usize;

        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_max_total_threads_per_threadgroup(&self, value: usize);

        /// An `StageInputOutputDescriptor` to fetch data from buffers.
        #[unsafe(method(stageInputDescriptor))]
        #[unsafe(method_family = none)]
        pub fn stage_input_descriptor(&self) -> Option<Retained<StageInputOutputDescriptor>>;

        /// This is copied when set.
        #[unsafe(method(setStageInputDescriptor:))]
        #[unsafe(method_family = none)]
        pub fn set_stage_input_descriptor(&self, descriptor: Option<&StageInputOutputDescriptor>);

        /// Optional properties for each buffer binding used by the compute function.
        #[unsafe(method(buffers))]
        #[unsafe(method_family = none)]
        pub fn buffers(&self) -> Retained<PipelineBufferDescriptorArray>;

        /// This flag makes this pipeline usable with indirect command buffers.
        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        pub fn support_indirect_command_buffers(&self) -> bool;

        #[unsafe(method(setSupportIndirectCommandBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_support_indirect_command_buffers(&self, enabled: bool);

        /// Restore all compute pipeline descriptor properties to their default values.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}

impl ComputePipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
