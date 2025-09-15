use objc2::{Encode, Encoding, RefEncode};

/// Blend operation (from `MTLBlendOperation`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum BlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
    /// Defers assigning the blend operation.
    Unspecialized = 5,
}

unsafe impl Encode for BlendOperation {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for BlendOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
