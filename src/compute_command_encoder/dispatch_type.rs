use objc2::{Encode, Encoding, RefEncode};

/// Dispatch type of the compute command encoder (from `MTLDispatchType`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DispatchType(pub u64);

unsafe impl Encode for DispatchType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for DispatchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
