use objc2::{Encode, Encoding, RefEncode};

/// The type of a resource binding (from `MTLBindingType`).
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BindingType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,
    ImageblockData = 16,
    Imageblock = 17,
    VisibleFunctionTable = 24,
    PrimitiveAccelerationStructure = 25,
    InstanceAccelerationStructure = 26,
    IntersectionFunctionTable = 27,
    ObjectPayload = 34,
    Tensor = 37,
}

unsafe impl Encode for BindingType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for BindingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
