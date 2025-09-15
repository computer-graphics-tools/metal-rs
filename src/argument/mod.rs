mod argument_descriptor;
mod binding_access;
mod argument_encoder;

pub use argument_descriptor::ArgumentDescriptor;
pub use binding_access::BindingAccess;
pub use argument_encoder::ArgumentEncoder;

/// Use this value to indicate that the attribute stride is static.
pub const ATTRIBUTE_STRIDE_STATIC: usize = usize::MAX;


