mod architecture;
mod argument_buffers_tier;
mod counter_sampling_point;
mod device;
mod device_error;
#[cfg(target_os = "macos")]
mod device_location;
#[cfg(target_os = "macos")]
mod device_notification;
mod feature_set;
mod gpu_family;
mod io_compression_method;
mod read_write_texture_tier;
mod size_and_align;
mod sparse_texture_region_alignment_mode;

pub use architecture::MTLArchitecture;
pub use argument_buffers_tier::MTLArgumentBuffersTier;
pub use counter_sampling_point::MTLCounterSamplingPoint;
pub use device::*;
pub use device_error::{MTLDeviceError, device_error_domain};
#[cfg(target_os = "macos")]
#[allow(deprecated)]
pub use device_location::MTLDeviceLocation;
#[cfg(target_os = "macos")]
pub use device_notification::*;
#[allow(deprecated)]
pub use feature_set::MTLFeatureSet;
pub use gpu_family::MTLGPUFamily;
pub use io_compression_method::MTLIOCompressionMethod;
pub use read_write_texture_tier::MTLReadWriteTextureTier;
pub use size_and_align::MTLSizeAndAlign;
pub use sparse_texture_region_alignment_mode::MTLSparseTextureRegionAlignmentMode;

mod process_performance_profile;
pub use process_performance_profile::{
    ProcessPerformanceProfile, has_performance_profile, is_device_certified_for,
    process_info_performance_profile_did_change_notification, process_performance_profile_default,
    process_performance_profile_sustained,
};
