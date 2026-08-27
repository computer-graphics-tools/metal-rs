mod layer_array;
mod layer_descriptor;
mod map;
mod map_descriptor;
mod sample_array;

pub use layer_array::MTLRasterizationRateLayerArray;
pub use layer_descriptor::MTLRasterizationRateLayerDescriptor;
pub use map::{MTLRasterizationRateMap, MTLRasterizationRateMapExt};
pub use map_descriptor::MTLRasterizationRateMapDescriptor;
pub use sample_array::MTLRasterizationRateSampleArray;

#[cfg(test)]
mod tests {
    use core::{ffi::c_float, ptr::NonNull};

    use objc2::rc::{Allocated, Retained};
    use objc2_foundation::{NSCopying, NSObjectProtocol};

    use super::*;
    use crate::MTLSize;

    fn assert_copying<T: NSCopying + ?Sized>() {}
    fn assert_nsobject<T: NSObjectProtocol + ?Sized>() {}
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}

    #[test]
    fn descriptor_and_protocol_conformances_match_the_header() {
        assert_nsobject::<MTLRasterizationRateSampleArray>();
        assert_nsobject::<MTLRasterizationRateLayerArray>();
        assert_copying::<MTLRasterizationRateLayerDescriptor>();
        assert_copying::<MTLRasterizationRateMapDescriptor>();
        assert_send_sync::<dyn MTLRasterizationRateMap>();
    }

    #[test]
    fn pointer_count_initializers_have_checked_slice_entry_points() {
        let _: fn(
            Allocated<MTLRasterizationRateLayerDescriptor>,
            MTLSize,
            NonNull<c_float>,
            NonNull<c_float>,
        ) -> Retained<MTLRasterizationRateLayerDescriptor> =
            MTLRasterizationRateLayerDescriptor::init_with_sample_count_horizontal_vertical;

        let _: fn(MTLSize, &[f32], &[f32]) -> Retained<MTLRasterizationRateLayerDescriptor> =
            MTLRasterizationRateLayerDescriptor::new_with_sample_count_horizontal_vertical;

        let _: fn(MTLSize, &[&MTLRasterizationRateLayerDescriptor]) -> Retained<MTLRasterizationRateMapDescriptor> =
            MTLRasterizationRateMapDescriptor::rasterization_rate_map_descriptor_with_screen_size_layers;
    }

    #[test]
    #[should_panic(expected = "horizontal sample count does not match sample_count.width")]
    fn layer_descriptor_rejects_a_short_horizontal_slice() {
        let _ = MTLRasterizationRateLayerDescriptor::new_with_sample_count_horizontal_vertical(
            MTLSize::new(2, 1, 0),
            &[1.0],
            &[1.0],
        );
    }
}
