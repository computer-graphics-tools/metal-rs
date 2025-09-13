mod data_type;
mod origin;
mod region;
mod resource_id;
mod sample_position;
mod size;

pub use data_type::DataType;
pub use origin::Origin;
pub use region::Region;
pub use resource_id::ResourceID;
pub use sample_position::SamplePosition;
pub use size::Size;

/// A floating point coordinate in an abstract 2D space.
pub type Coordinate2D = SamplePosition;
