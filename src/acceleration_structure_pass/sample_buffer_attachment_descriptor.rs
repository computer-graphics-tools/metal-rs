use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::CounterSampleBuffer;

extern_class!(
    /// Sample buffer attachment descriptor for acceleration structure passes.
    #[unsafe(super(NSObject))]
    #[name = "MTLAccelerationStructurePassSampleBufferAttachmentDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AccelerationStructurePassSampleBufferAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for AccelerationStructurePassSampleBufferAttachmentDescriptor {}
);

unsafe impl CopyingHelper for AccelerationStructurePassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for AccelerationStructurePassSampleBufferAttachmentDescriptor {}
);

impl AccelerationStructurePassSampleBufferAttachmentDescriptor {
    extern_methods!(
        /// The sample buffer to store samples for the encoder-defined samples.
        #[unsafe(method(sampleBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_buffer(&self) -> Option<Retained<ProtocolObject<dyn CounterSampleBuffer>>>;

        /// Setter for `sample_buffer`.
        #[unsafe(method(setSampleBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sample_buffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn CounterSampleBuffer>>,
        );

        /// Sample index for start-of-encoder sample.
        #[unsafe(method(startOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn start_of_encoder_sample_index(&self) -> usize;

        #[unsafe(method(setStartOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_start_of_encoder_sample_index(&self, value: usize);

        /// Sample index for end-of-encoder sample.
        #[unsafe(method(endOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn end_of_encoder_sample_index(&self) -> usize;

        #[unsafe(method(setEndOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_end_of_encoder_sample_index(&self, value: usize);
    );
}

impl AccelerationStructurePassSampleBufferAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}


