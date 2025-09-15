use objc2::{Encode, Encoding, RefEncode};

/// Function log type (from `MTLFunctionLogType`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionLogType(pub usize);

impl FunctionLogType {
    pub const Validation: Self = Self(0);
}

unsafe impl Encode for FunctionLogType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for FunctionLogType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
