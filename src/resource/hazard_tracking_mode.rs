use objc2::{Encode, Encoding, RefEncode};

/// Describes how hazard tracking is performed (from `MTLHazardTrackingMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum HazardTrackingMode {
    /// The default hazard tracking mode for the context. Refer to the usage of the field for semantics.
    Default = 0,
    /// Do not perform hazard tracking.
    Untracked = 1,
    /// Perform hazard tracking.
    Tracked = 2,
}

unsafe impl Encode for HazardTrackingMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for HazardTrackingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


