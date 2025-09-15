use objc2::{Encode, Encoding, RefEncode};

/// Result status from flushing and destroying a Metal I/O compression context
/// (ported from `MTLIOCompressionStatus`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum CompressionStatus {
    /// Compression completed successfully.
    Complete = 0,
    /// Compression encountered an error.
    Error = 1,
}

unsafe impl Encode for CompressionStatus {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for CompressionStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


