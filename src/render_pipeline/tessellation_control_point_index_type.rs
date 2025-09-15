use objc2::{Encode, Encoding, RefEncode};

/// Tessellation control point index type (from `MTLTessellationControlPointIndexType`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TessellationControlPointIndexType {
    None = 0,
    UInt16 = 1,
    UInt32 = 2,
}

unsafe impl Encode for TessellationControlPointIndexType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for TessellationControlPointIndexType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
