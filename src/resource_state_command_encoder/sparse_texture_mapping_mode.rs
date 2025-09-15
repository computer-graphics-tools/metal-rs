use objc2::{Encode, Encoding, RefEncode};

/// Type of mapping operation for sparse texture (from `MTLSparseTextureMappingMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum SparseTextureMappingMode {
    Map = 0,
    Unmap = 1,
}

unsafe impl Encode for SparseTextureMappingMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for SparseTextureMappingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
