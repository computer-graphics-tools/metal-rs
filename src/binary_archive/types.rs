use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

extern "C" {
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain?language=objc`
    pub static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

/// Bridged error domain symbol for `MTLBinaryArchive`.
pub const BinaryArchiveDomain: &'static NSErrorDomain = unsafe { MTLBinaryArchiveDomain };

/// Errors emitted by binary archive operations.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BinaryArchiveError(pub u64);

impl BinaryArchiveError {
    pub const None: Self = Self(0);
    pub const InvalidFile: Self = Self(1);
    pub const UnexpectedElement: Self = Self(2);
    pub const CompilationFailure: Self = Self(3);
    pub const InternalError: Self = Self(4);
}

unsafe impl Encode for BinaryArchiveError {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for BinaryArchiveError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
