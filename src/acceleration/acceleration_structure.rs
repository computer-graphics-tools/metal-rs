use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

use crate::Resource;

extern_protocol!(
    /// Minimal wrapper for `MTLAccelerationStructure`.
    #[name = "MTLAccelerationStructure"]
    pub unsafe trait AccelerationStructure: Resource + NSObjectProtocol {}
);
