use objc2::{Encode, Encoding, RefEncode};

/// Overall kind of entry point (from `MTLFunctionType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
    Visible = 5,
    Intersection = 6,
    Mesh = 7,
    Object = 8,
}

unsafe impl Encode for FunctionType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for FunctionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
