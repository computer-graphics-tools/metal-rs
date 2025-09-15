use objc2::{Encode, Encoding, RefEncode};

/// Options for selecting and filtering between mipmap levels (from `MTLSamplerMipFilter`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

unsafe impl Encode for MipFilter {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MipFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
