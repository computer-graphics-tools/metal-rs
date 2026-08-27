use objc2_foundation::NSErrorDomain;

unsafe extern "C" {
    /// Error domain constant for `MTLBinaryArchive` operations.
    ///
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain`.
    static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

/// Returns the error domain for `MTLBinaryArchive` operations.
pub fn binary_archive_domain() -> String {
    unsafe { MTLBinaryArchiveDomain }.to_string()
}
