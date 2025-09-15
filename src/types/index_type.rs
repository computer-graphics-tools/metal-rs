use objc2::{Encode, Encoding, RefEncode};

/// Index type used for indexed drawing (from `MTLIndexType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLIndexType {
    UInt16 = 0,
    UInt32 = 1,
}

unsafe impl Encode for MTLIndexType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLIndexType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
