use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::device::MTLSizeAndAlign;
use crate::types::{Coordinate2D, MTLSize};
use crate::{MTLBuffer, MTLDevice};

extern_protocol!(
    /// Compiled read-only object that determines how variable rasterization rate is applied when rendering.
    pub unsafe trait MTLRasterizationRateMap: NSObjectProtocol + Send + Sync {
        /// The device on which the rasterization rate map was created
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// The dimensions, in screen space pixels, of the region where variable rasterization is applied.
        #[unsafe(method(screenSize))]
        #[unsafe(method_family = none)]
        unsafe fn screen_size(&self) -> MTLSize;

        /// The granularity, in physical pixels, at which variable rasterization rate varies.
        #[unsafe(method(physicalGranularity))]
        #[unsafe(method_family = none)]
        unsafe fn physical_granularity(&self) -> MTLSize;

        /// The number of different configured layers in the rasterization map.
        #[unsafe(method(layerCount))]
        #[unsafe(method_family = none)]
        unsafe fn layer_count(&self) -> usize;

        /// Returns the size and alignment requirements of the parameter buffer for this rate map.
        #[unsafe(method(parameterBufferSizeAndAlign))]
        #[unsafe(method_family = none)]
        unsafe fn parameter_buffer_size_and_align(&self) -> MTLSizeAndAlign;

        /// Copy the parameter data into the provided buffer at the provided offset.
        #[unsafe(method(copyParameterDataToBuffer:offset:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_parameter_data_to_buffer_offset(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
        );

        /// Computes screen to physical coordinates mapping for the given layer.
        #[unsafe(method(mapScreenToPhysicalCoordinates:forLayer:))]
        #[unsafe(method_family = none)]
        unsafe fn map_screen_to_physical_coordinates_for_layer(
            &self,
            screen_coordinates: Coordinate2D,
            layer_index: usize,
        ) -> Coordinate2D;

        /// Computes physical to screen coordinates mapping for the given layer.
        #[unsafe(method(mapPhysicalToScreenCoordinates:forLayer:))]
        #[unsafe(method_family = none)]
        unsafe fn map_physical_to_screen_coordinates_for_layer(
            &self,
            physical_coordinates: Coordinate2D,
            layer_index: usize,
        ) -> Coordinate2D;
    }
);
