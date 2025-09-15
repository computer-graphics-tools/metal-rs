mod descriptor;
mod manager;
mod scope;
mod types;

pub use descriptor::MTLCaptureDescriptor;
pub use manager::MTLCaptureManager;
pub use scope::MTLCaptureScope;
pub use types::{MTLCaptureDestination, MTLCaptureError, capture_error_domain};
