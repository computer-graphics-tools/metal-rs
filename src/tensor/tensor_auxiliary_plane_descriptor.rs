use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLTensorDataType, MTLTensorExtents};

extern_class!(
    /// A configuration for an auxiliary plane in a multi-plane tensor.
    ///
    /// Availability: macOS 27.0+, iOS 27.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorAuxiliaryPlaneDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTensorAuxiliaryPlaneDescriptor {}
);

unsafe impl CopyingHelper for MTLTensorAuxiliaryPlaneDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTensorAuxiliaryPlaneDescriptor {}
);

impl MTLTensorAuxiliaryPlaneDescriptor {
    extern_methods!(
        /// The data format of all elements in the plane.
        ///
        /// The default is [`MTLTensorDataType::MetalFloat8UE8M0`].
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> MTLTensorDataType;

        /// Setter for [`data_type`][Self::data_type].
        #[unsafe(method(setDataType:))]
        #[unsafe(method_family = none)]
        pub fn set_data_type(
            &self,
            data_type: MTLTensorDataType,
        );

        /// The number of data-plane elements that correspond to one element in this plane.
        #[unsafe(method(blockFactors))]
        #[unsafe(method_family = none)]
        pub fn block_factors(&self) -> Retained<MTLTensorExtents>;

        /// Setter for [`block_factors`][Self::block_factors].
        #[unsafe(method(setBlockFactors:))]
        #[unsafe(method_family = none)]
        pub fn set_block_factors(
            &self,
            block_factors: &MTLTensorExtents,
        );
    );
}

impl MTLTensorAuxiliaryPlaneDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
