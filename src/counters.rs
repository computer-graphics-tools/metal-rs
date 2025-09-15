use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Bridged protocol for `MTLCounterSampleBuffer`.
    #[name = "MTLCounterSampleBuffer"]
    pub unsafe trait CounterSampleBuffer: NSObjectProtocol {}
);
