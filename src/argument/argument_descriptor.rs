use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_conformance, extern_methods};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{BindingAccess, DataType, TextureType};

extern_class!(
    /// Represents a member of an argument buffer.
    ///
    /// See also Apple's documentation for MTLArgumentDescriptor.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[name = "MTLArgumentDescriptor"]
    pub struct ArgumentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for ArgumentDescriptor {}
);

unsafe impl CopyingHelper for ArgumentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for ArgumentDescriptor {}
);

impl ArgumentDescriptor {
    extern_methods!(
        /// Create an autoreleased default argument descriptor.
        #[unsafe(method(argumentDescriptor))]
        pub fn new() -> Retained<Self>;

        /// For constants, the data type. Otherwise, MTLDataTypeTexture, MTLDataTypeSampler, or MTLDataTypePointer.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> DataType;

        /// Sets the data type of the argument.
        #[unsafe(method(setDataType:))]
        pub fn set_data_type(&self, value: DataType);

        /// The binding point index of the argument.
        #[unsafe(method(index))]
        #[unsafe(method_family = none)]
        pub fn index(&self) -> usize;

        /// Sets the binding point index of the argument.
        #[unsafe(method(setIndex:))]
        pub fn set_index(&self, value: usize);

        /// The length of an array of constants, textures, or samplers, or 0 for non-array arguments.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        pub fn array_length(&self) -> usize;

        /// Sets the array length of the argument.
        #[unsafe(method(setArrayLength:))]
        pub fn set_array_length(&self, value: usize);

        /// Access flags for the argument.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        pub fn access(&self) -> BindingAccess;

        /// Sets the access flags for the argument.
        #[unsafe(method(setAccess:))]
        pub fn set_access(&self, value: BindingAccess);

        /// For texture arguments, the texture type.
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        pub fn texture_type(&self) -> TextureType;

        /// Sets the texture type for the argument.
        #[unsafe(method(setTextureType:))]
        pub fn set_texture_type(&self, value: TextureType);

        /// If set, forces the constant block to be aligned to the given alignment.
        /// Should only be set on the first constant of the block and is only valid if a corresponding
        /// explicit "alignas" is applied to the constant in the metal shader language.
        #[unsafe(method(constantBlockAlignment))]
        #[unsafe(method_family = none)]
        pub fn constant_block_alignment(&self) -> usize;

        /// Sets the constant block alignment.
        #[unsafe(method(setConstantBlockAlignment:))]
        pub fn set_constant_block_alignment(&self, value: usize);
    );
}


