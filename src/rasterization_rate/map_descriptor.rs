use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::{RasterizationRateLayerArray, RasterizationRateLayerDescriptor};
use crate::types::Size;

extern_class!(
    /// Describes a rasterization rate map containing layer descriptors.
    #[unsafe(super(NSObject))]
    #[name = "MTLRasterizationRateMapDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RasterizationRateMapDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for RasterizationRateMapDescriptor {}
);

unsafe impl CopyingHelper for RasterizationRateMapDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for RasterizationRateMapDescriptor {}
);

impl RasterizationRateMapDescriptor {
    extern_methods!(
        /// Convenience descriptor creation without layers.
        #[unsafe(method(rasterizationRateMapDescriptorWithScreenSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rasterization_rate_map_descriptor_with_screen_size(
            screen_size: Size,
        ) -> Retained<RasterizationRateMapDescriptor>;

        /// Convenience descriptor creation for a single layer.
        #[unsafe(method(rasterizationRateMapDescriptorWithScreenSize:layer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rasterization_rate_map_descriptor_with_screen_size_layer(
            screen_size: Size,
            layer: &RasterizationRateLayerDescriptor,
        ) -> Retained<RasterizationRateMapDescriptor>;

        /// Returns the layer descriptor for the given index, if any.
        #[unsafe(method(layerAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layer_at_index(
            &self,
            layer_index: usize,
        ) -> Option<Retained<RasterizationRateLayerDescriptor>>;

        /// Sets the layer descriptor for the given index.
        #[unsafe(method(setLayer:atIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_layer_at_index(
            &self,
            layer: Option<&RasterizationRateLayerDescriptor>,
            layer_index: usize,
        );

        /// Access the modifiable array of layers.
        #[unsafe(method(layers))]
        #[unsafe(method_family = none)]
        pub unsafe fn layers(&self) -> Retained<RasterizationRateLayerArray>;

        /// The screen size in pixels of the region where variable rasterization is applied.
        #[unsafe(method(screenSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn screen_size(&self) -> Size;

        /// Setter for [`screen_size`][Self::screen_size].
        #[unsafe(method(setScreenSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_screen_size(&self, screen_size: Size);

        /// Optional label for the descriptor.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_label(&self, label: Option<&NSString>);

        /// Number of subsequent non-nil layers starting at index 0.
        #[unsafe(method(layerCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn layer_count(&self) -> usize;
    );
}

/// Methods declared on superclass `NSObject`.
impl RasterizationRateMapDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
