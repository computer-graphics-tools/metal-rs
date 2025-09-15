mod argument_descriptor;
mod argument_encoder;
mod binding_access;
mod binding_type;

pub use argument_descriptor::MTLArgumentDescriptor;
pub use argument_encoder::MTLArgumentEncoder;
pub use binding_access::MTLBindingAccess;
pub use binding_type::MTLBindingType;

/// Use this value to indicate that the attribute stride is static.
pub const ATTRIBUTE_STRIDE_STATIC: usize = usize::MAX;
