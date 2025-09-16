use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

unsafe extern "C" {
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain?language=objc`
    pub static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

/// Bridged error domain symbol for `MTLBinaryArchive`.
pub unsafe fn binary_archive_domain() -> &'static NSErrorDomain { unsafe {
    MTLBinaryArchiveDomain
}}

/// Errors emitted by binary archive operations.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBinaryArchiveError(pub u64);

impl MTLBinaryArchiveError {
    pub const NONE: Self = Self(0);
    pub const INVALID_FILE: Self = Self(1);
    pub const UNEXPECTED_ELEMENT: Self = Self(2);
    pub const COMPILATION_FAILURE: Self = Self(3);
    pub const INTERNAL_ERROR: Self = Self(4);
}

unsafe impl Encode for MTLBinaryArchiveError {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBinaryArchiveError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
