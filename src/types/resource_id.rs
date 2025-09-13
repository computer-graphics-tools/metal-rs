use objc2::{Encode, Encoding, RefEncode};

/// Handle of the GPU resource used for binding resources to argument tables and resource view pools.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ResourceID {
    pub _impl: u64,
}

unsafe impl Encode for ResourceID {
    const ENCODING: Encoding = Encoding::Struct("{MTLResourceID=Q}", &[u64::ENCODING]);
}

unsafe impl RefEncode for ResourceID {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
