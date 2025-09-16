use objc2::{Encode, Encoding, RefEncode};

/// Curve end caps (from `MTLCurveEndCaps`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveEndCaps {
    /// No end caps.
    None = 0,
    /// Disk end caps.
    Disk = 1,
    /// Spherical end caps.
    Sphere = 2,
}

unsafe impl Encode for MTLCurveEndCaps {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveEndCaps {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
