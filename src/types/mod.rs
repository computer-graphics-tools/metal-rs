mod data_type;
mod gpu_address;
mod index_type;
mod origin;
mod region;
mod resource_id;
mod sample_position;
mod size;

pub use data_type::MTLDataType;
pub use gpu_address::MTLGPUAddress;
pub use index_type::MTLIndexType;
pub use origin::MTLOrigin;
pub use region::MTLRegion;
pub use resource_id::MTLResourceID;
pub use sample_position::MTLSamplePosition;
pub use size::MTLSize;

/// A floating point coordinate in an abstract 2D space.
pub type Coordinate2D = MTLSamplePosition;
