use objc2::{Encode, Encoding, RefEncode};

/// Specifies whether a buffer will be modified between binding and pipeline execution (from `MTLMutability`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Mutability {
    Default = 0,
    Mutable = 1,
    Immutable = 2,
}

unsafe impl Encode for Mutability {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for Mutability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
