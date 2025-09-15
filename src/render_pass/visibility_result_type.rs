use objc2::{Encode, Encoding, RefEncode};

/// Controls whether visibility results accumulate between encoders (from `MTLVisibilityResultType`).
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisibilityResultType {
    Reset = 0,
    Accumulate = 1,
}

unsafe impl Encode for VisibilityResultType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for VisibilityResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
