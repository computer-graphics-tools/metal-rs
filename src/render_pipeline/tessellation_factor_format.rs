use objc2::{Encode, Encoding, RefEncode};

/// Tessellation factor format (from `MTLTessellationFactorFormat`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TessellationFactorFormat {
    Half = 0,
}

unsafe impl Encode for TessellationFactorFormat {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for TessellationFactorFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
