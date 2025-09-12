use objc2::{Encode, Encoding, RefEncode};

/// Texture compression type (from `MTLTextureCompressionType`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TextureCompressionType {
    Lossless = 0,
    Lossy = 1,
}

unsafe impl Encode for TextureCompressionType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for TextureCompressionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


