use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::AccelerationStructurePassSampleBufferAttachmentDescriptorArray;

extern_class!(
    /// Pass descriptor for creating acceleration structure command encoders.
    #[unsafe(super(NSObject))]
    #[name = "MTLAccelerationStructurePassDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AccelerationStructurePassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for AccelerationStructurePassDescriptor {}
);

unsafe impl CopyingHelper for AccelerationStructurePassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for AccelerationStructurePassDescriptor {}
);

impl AccelerationStructurePassDescriptor {
    extern_methods!(
        /// Create a default acceleration structure pass descriptor.
        #[unsafe(method(accelerationStructurePassDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn acceleration_structure_pass_descriptor() -> Retained<AccelerationStructurePassDescriptor>;

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_buffer_attachments(&self) -> Retained<AccelerationStructurePassSampleBufferAttachmentDescriptorArray>;
    );
}

impl AccelerationStructurePassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}


