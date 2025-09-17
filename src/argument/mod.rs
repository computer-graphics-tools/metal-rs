mod argument;
mod argument_descriptor;
mod argument_encoder;
mod argument_type;
mod array_type;
mod binding;
mod binding_access;
mod binding_type;
mod index_type;
mod pointer_type;
mod struct_member;
mod struct_type;
mod texture_reference_type;
mod type_reflection;

pub use argument::MTLArgument;
pub use argument_descriptor::MTLArgumentDescriptor;
pub use argument_encoder::MTLArgumentEncoder;
pub use argument_type::MTLArgumentType;
pub use array_type::MTLArrayType;
pub use binding::{
    MTLBinding, MTLBufferBinding, MTLObjectPayloadBinding, MTLTensorBinding, MTLTextureBinding,
    MTLThreadgroupBinding,
};
pub use binding_access::{MTLArgumentAccess, MTLBindingAccess};
pub use binding_type::MTLBindingType;
pub use index_type::MTLIndexType;
pub use pointer_type::MTLPointerType;
pub use struct_member::MTLStructMember;
pub use struct_type::MTLStructType;
pub use texture_reference_type::MTLTextureReferenceType;
pub use type_reflection::MTLType;

/// Use this value to indicate that the attribute stride is static.
pub const ATTRIBUTE_STRIDE_STATIC: usize = usize::MAX;
