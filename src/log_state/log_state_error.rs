use objc2::{Encode, Encoding, RefEncode};

/// Errors when creating a log state (from `MTLLogStateError`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogStateError {
    InvalidSize = 1,
    Invalid = 2,
}

unsafe impl Encode for LogStateError {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for LogStateError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
