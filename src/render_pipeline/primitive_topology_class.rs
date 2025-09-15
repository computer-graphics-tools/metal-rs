use objc2::{Encode, Encoding, RefEncode};

/// Primitive topology class (from `MTLPrimitiveTopologyClass`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum PrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

unsafe impl Encode for PrimitiveTopologyClass {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for PrimitiveTopologyClass {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
