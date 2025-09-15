use objc2::{Encode, Encoding, RefEncode};

/// Attachment store action (from `MTLStoreAction`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
    StoreAndMultisampleResolve = 3,
    Unknown = 4,
    CustomSampleDepthStore = 5,
}

unsafe impl Encode for StoreAction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for StoreAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
