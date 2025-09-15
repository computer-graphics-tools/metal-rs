use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSData, NSObjectProtocol, NSRange, NSString};

use crate::Device;

extern_protocol!(
    /// The Counter Sample Buffer contains opaque counter samples as well
    /// as state needed to request a sample from the API.
    #[name = "MTLCounterSampleBuffer"]
    pub unsafe trait CounterSampleBuffer: NSObjectProtocol {
        /// The device this sample buffer was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn Device>>;

        /// The label set on the descriptor used to create this sample buffer.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Retained<NSString>;

        /// Number of samples in this buffer.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        unsafe fn sample_count(&self) -> usize;

        /// Resolve the counters in the given range to an NSData containing the counter values.
        #[unsafe(method(resolveCounterRange:))]
        #[unsafe(method_family = none)]
        unsafe fn resolve_counter_range(&self, range: NSRange) -> Option<Retained<NSData>>;
    }
);
