use objc2::rc::Retained;
use objc2_foundation::NSError;

/// A Rust-owned error reported by Metal or its Rust bridge.
///
/// Metal reports failures as `NSError` objects. This type copies the stable
/// information callers need so Foundation types do not leak through the
/// crate's public API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetalError {
    domain: String,
    code: isize,
    description: String,
}

impl MetalError {
    pub(super) fn invalid_input(
        operation: &str,
        description: &str,
    ) -> Self {
        Self {
            domain: "mtl-rs".to_owned(),
            code: -1,
            description: format!("{operation}: {description}"),
        }
    }

    pub(super) fn from_nserror(error: Retained<NSError>) -> Self {
        Self {
            domain: error.domain().to_string(),
            code: error.code(),
            description: error.localizedDescription().to_string(),
        }
    }

    pub(super) unsafe fn from_unretained(error: *mut NSError) -> Option<Self> {
        unsafe { Retained::retain(error) }.map(Self::from_nserror)
    }

    pub(super) unsafe fn result_from_nullable<T: ?Sized>(
        value: Option<Retained<T>>,
        error: *mut NSError,
        operation: &str,
    ) -> Result<Retained<T>, Self> {
        match value {
            Some(value) => Ok(value),
            None => Err(unsafe { Self::from_unretained(error) }
                .unwrap_or_else(|| Self::invalid_input(operation, "Metal returned neither a value nor an error"))),
        }
    }

    /// The Metal error domain.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// The error code within [`Self::domain`].
    pub const fn code(&self) -> isize {
        self.code
    }

    /// A localized description supplied by Metal.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for MetalError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.description.fmt(formatter)
    }
}

impl std::error::Error for MetalError {}

#[cfg(test)]
mod tests {
    use objc2_foundation::{NSError, NSString};

    use super::MetalError;

    #[test]
    fn copies_foundation_error_into_rust_values() {
        let domain = NSString::from_str("mtl-rs.tests");
        let error = unsafe { NSError::errorWithDomain_code_userInfo(&domain, 27, None) };

        let error = MetalError::from_nserror(error);

        assert_eq!(error.domain(), "mtl-rs.tests");
        assert_eq!(error.code(), 27);
    }
}
