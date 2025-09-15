use objc2::{Encode, Encoding, RefEncode};

/// IO priority for queues (ported from `MTLIOPriority`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IoPriority {
    High = 0,
    Normal = 1,
    Low = 2,
}

unsafe impl Encode for IoPriority {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for IoPriority {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
