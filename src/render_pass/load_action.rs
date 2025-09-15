use objc2::{Encode, Encoding, RefEncode};

/// Attachment load action (from `MTLLoadAction`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

unsafe impl Encode for LoadAction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for LoadAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
