use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

use super::{MTLTensorDataType, MTLTensorExtents, MTLTensorPlaneType};
use crate::MTLBuffer;

extern_protocol!(
    /// The configuration and storage of an auxiliary plane in a multi-plane tensor.
    ///
    /// Availability: macOS 27.0+, iOS 27.0+
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLTensorAuxiliaryPlane` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLTensorAuxiliaryPlane: NSObjectProtocol {
        /// The data format of all elements in the plane.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        fn data_type(&self) -> MTLTensorDataType;

        /// The number of data-plane elements that correspond to one element in this plane.
        #[unsafe(method(blockFactors))]
        #[unsafe(method_family = none)]
        fn block_factors(&self) -> Retained<MTLTensorExtents>;

        /// The buffer that provides this plane's storage, or `None` if the tensor did not use a buffer.
        #[unsafe(method(buffer))]
        #[unsafe(method_family = none)]
        fn buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// The byte offset into the buffer, or zero if the tensor did not use a buffer.
        #[unsafe(method(bufferOffset))]
        #[unsafe(method_family = none)]
        fn buffer_offset(&self) -> usize;

        /// The type of information this plane stores.
        #[unsafe(method(planeType))]
        #[unsafe(method_family = none)]
        fn plane_type(&self) -> MTLTensorPlaneType;
    }
);
