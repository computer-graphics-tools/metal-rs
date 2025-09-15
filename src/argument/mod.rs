mod argument_descriptor;
mod argument_encoder;
mod binding_access;
mod binding_type;

pub use argument_descriptor::ArgumentDescriptor;
pub use argument_encoder::ArgumentEncoder;
pub use binding_access::BindingAccess;
pub use binding_type::BindingType;

/// Use this value to indicate that the attribute stride is static.
pub const ATTRIBUTE_STRIDE_STATIC: usize = usize::MAX;
