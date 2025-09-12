mod data_type;
mod origin;
mod size;
mod region;
mod sample_position;
mod resource_id;

pub use data_type::DataType;
pub use origin::Origin;
pub use size::Size;
pub use region::Region;
pub use sample_position::SamplePosition;
pub use resource_id::ResourceId;

/// A floating point coordinate in an abstract 2D space.
pub type Coordinate2D = SamplePosition;


