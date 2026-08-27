use objc2::{extern_class, extern_methods, rc::Retained, runtime::NSObject};

use crate::{MTLTensorDataType, MTLTensorExtents, MTLTensorPlaneType};

extern_class!(
    /// An auxiliary plane that a shader's tensor argument requires.
    ///
    /// Availability: macOS 27.0+, iOS 27.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorAuxiliaryPlaneType;
);

impl MTLTensorAuxiliaryPlaneType {
    extern_methods!(
        /// The data format of all elements in the plane.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> MTLTensorDataType;

        /// The number of data-plane elements that correspond to one element in this plane.
        #[unsafe(method(blockFactors))]
        #[unsafe(method_family = none)]
        pub fn block_factors(&self) -> Retained<MTLTensorExtents>;

        /// The type of information this plane stores.
        #[unsafe(method(planeType))]
        #[unsafe(method_family = none)]
        pub fn plane_type(&self) -> MTLTensorPlaneType;
    );
}
