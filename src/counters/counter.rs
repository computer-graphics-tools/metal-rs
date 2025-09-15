use objc2::{extern_protocol, rc::Retained};
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// A descriptor for a single counter.
    #[name = "MTLCounter"]
    pub unsafe trait Counter: NSObjectProtocol + Send + Sync {
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        unsafe fn name(&self) -> Retained<NSString>;
    }
);
