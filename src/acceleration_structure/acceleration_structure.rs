use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

use crate::MTLResource;

extern_protocol!(
    /// Minimal wrapper for `MTLAccelerationStructure`.
    pub unsafe trait MTLAccelerationStructure: MTLResource + NSObjectProtocol {}
);
