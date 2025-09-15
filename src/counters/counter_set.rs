use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use super::Counter;

extern_protocol!(
    /// A collection of counters that the device can capture in a single pass.
    #[name = "MTLCounterSet"]
    pub unsafe trait CounterSet: NSObjectProtocol + Send + Sync {
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        unsafe fn name(&self) -> Retained<NSString>;

        /// The counters array contains all the counters that will be written
        /// when a counter sample is collected. Counters that do not appear in this array
        /// will not be written to the resolved buffer when the samples are resolved.
        #[unsafe(method(counters))]
        #[unsafe(method_family = none)]
        unsafe fn counters(&self) -> Retained<NSArray<ProtocolObject<dyn Counter>>>;
    }
);
