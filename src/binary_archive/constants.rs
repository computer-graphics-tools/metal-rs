use objc2_foundation::NSErrorDomain;

unsafe extern "C" {
    /// Error domain constant for `MTLBinaryArchive` operations.
    ///
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain`.
    pub static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

/// Bridged error domain symbol for `MTLBinaryArchive`.
pub unsafe fn binary_archive_domain() -> &'static NSErrorDomain {
    unsafe { MTLBinaryArchiveDomain }
}
