use objc2::{Encode, Encoding, RefEncode};

/// Curve type (from `MTLCurveType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveType {
    /// Curve with a circular cross-section. Ideal for close-up viewing.
    Round = 0,
    /// Curve with a flat cross-section aligned with the ray direction. Ideal for distant viewing or small radii.
    Flat = 1,
}

unsafe impl Encode for MTLCurveType {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
