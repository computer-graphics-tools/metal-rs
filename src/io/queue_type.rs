use objc2::{Encode, Encoding, RefEncode};

/// IO command queue type (ported from `MTLIOCommandQueueType`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IoCommandQueueType {
    Concurrent = 0,
    Serial = 1,
}

unsafe impl Encode for IoCommandQueueType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for IoCommandQueueType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


