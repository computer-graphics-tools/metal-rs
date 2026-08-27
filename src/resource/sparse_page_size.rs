use objc2::{Encode, Encoding, RefEncode};

/// Physical size of sparse resource page in KBs (from `MTLSparsePageSize`).
///
/// Availability: macOS 13.0+, iOS 16.0+
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSparsePageSize {
    /// 16 KB sparse page size.
    KB16 = 101,
    /// 64 KB sparse page size.
    KB64 = 102,
    /// 256 KB sparse page size.
    KB256 = 103,
}

unsafe impl Encode for MTLSparsePageSize {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLSparsePageSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
