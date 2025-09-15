mod archive_trait;
mod descriptor;
mod types;

pub use archive_trait::BinaryArchive;
pub use descriptor::BinaryArchiveDescriptor;
pub use types::{BinaryArchiveError, binary_archive_domain};
