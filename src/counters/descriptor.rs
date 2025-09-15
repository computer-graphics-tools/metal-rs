use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::CounterSet;

extern_class!(
    /// Object to represent the counter state.
    #[unsafe(super(NSObject))]
    #[name = "MTLCounterSampleBufferDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CounterSampleBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for CounterSampleBufferDescriptor {}
);

unsafe impl CopyingHelper for CounterSampleBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CounterSampleBufferDescriptor {}
);

impl CounterSampleBufferDescriptor {
    extern_methods!(
        #[unsafe(method(counterSet))]
        #[unsafe(method_family = none)]
        pub unsafe fn counter_set(&self) -> Option<Retained<ProtocolObject<dyn CounterSet>>>;

        #[unsafe(method(setCounterSet:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_counter_set(&self, counter_set: Option<&ProtocolObject<dyn CounterSet>>);

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_label(&self, label: &NSString);

        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_count(&self) -> usize;

        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sample_count(&self, sample_count: usize);
    );
}

impl CounterSampleBufferDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
