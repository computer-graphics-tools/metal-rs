use objc2::{Encode, Encoding, RefEncode};

/// Compression methods for Metal I/O handles (ported from `MTLIOCompressionMethod`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IoCompressionMethod {
    Zlib = 0,
    Lzfse = 1,
    Lz4 = 2,
    Lzma = 3,
    LzBitmap = 4,
}

unsafe impl Encode for IoCompressionMethod {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for IoCompressionMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


