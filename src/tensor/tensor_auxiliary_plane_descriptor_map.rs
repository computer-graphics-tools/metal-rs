use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLTensorAuxiliaryPlaneDescriptor, MTLTensorPlaneType};

extern_class!(
    /// A map of auxiliary-plane descriptors keyed by plane type.
    ///
    /// Availability: macOS 27.0+, iOS 27.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorAuxiliaryPlaneDescriptorMap;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTensorAuxiliaryPlaneDescriptorMap {}
);

unsafe impl CopyingHelper for MTLTensorAuxiliaryPlaneDescriptorMap {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTensorAuxiliaryPlaneDescriptorMap {}
);

impl MTLTensorAuxiliaryPlaneDescriptorMap {
    extern_methods!(
        /// Associates an auxiliary-plane descriptor with a plane type.
        ///
        /// [`MTLTensorPlaneType::Data`] is not valid because every tensor configures its data
        /// plane directly on [`crate::MTLTensorDescriptor`].
        #[unsafe(method(setDescriptor:forPlane:))]
        #[unsafe(method_family = none)]
        pub fn set_descriptor_for_plane(
            &self,
            descriptor: &MTLTensorAuxiliaryPlaneDescriptor,
            plane: MTLTensorPlaneType,
        );

        /// Returns the descriptor for a plane, or `None` when no descriptor is associated with it.
        #[unsafe(method(descriptorForPlane:))]
        #[unsafe(method_family = none)]
        pub fn descriptor_for_plane(
            &self,
            plane: MTLTensorPlaneType,
        ) -> Option<Retained<MTLTensorAuxiliaryPlaneDescriptor>>;

        /// Removes every descriptor from the map.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}

impl MTLTensorAuxiliaryPlaneDescriptorMap {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
