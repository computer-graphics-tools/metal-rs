use core::{ffi::c_float, ptr::NonNull};

use objc2::{
    ClassType, extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLRasterizationRateSampleArray;
use crate::types::MTLSize;

extern_class!(
    /// Describes the minimum rasterization rate screen space using two piecewise linear functions.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateLayerDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRasterizationRateLayerDescriptor {}
);

unsafe impl CopyingHelper for MTLRasterizationRateLayerDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRasterizationRateLayerDescriptor {}
);

impl MTLRasterizationRateLayerDescriptor {
    extern_methods!(
        /// Initialize a descriptor for a layer with the given number of quality samples on the horizontal and vertical axis.
        #[unsafe(method(initWithSampleCount:))]
        #[unsafe(method_family = init)]
        pub fn init_with_sample_count(
            this: Allocated<Self>,
            sample_count: MTLSize,
        ) -> Retained<Self>;

        /// Initializes a descriptor with sample values from raw arrays.
        ///
        /// `horizontal` must point to at least `sample_count.width` readable
        /// `float` values, and `vertical` must point to at least
        /// `sample_count.height` readable `float` values.
        #[unsafe(method(initWithSampleCount:horizontal:vertical:))]
        #[unsafe(method_family = init)]
        pub fn init_with_sample_count_horizontal_vertical(
            this: Allocated<Self>,
            sample_count: MTLSize,
            horizontal: NonNull<c_float>,
            vertical: NonNull<c_float>,
        ) -> Retained<Self>;

        /// The number of quality samples used on the horizontal and vertical axes.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        pub fn sample_count(&self) -> MTLSize;

        /// The maximum number of quality samples this descriptor can use for the horizontal and vertical axes.
        #[unsafe(method(maxSampleCount))]
        #[unsafe(method_family = none)]
        pub fn max_sample_count(&self) -> MTLSize;

        /// Pointer to mutable storage array for horizontal samples.
        #[unsafe(method(horizontalSampleStorage))]
        #[unsafe(method_family = none)]
        pub fn horizontal_sample_storage(&self) -> NonNull<c_float>;

        /// Pointer to mutable storage array for vertical samples.
        #[unsafe(method(verticalSampleStorage))]
        #[unsafe(method_family = none)]
        pub fn vertical_sample_storage(&self) -> NonNull<c_float>;

        /// Bounds-checked access helper for horizontal samples.
        #[unsafe(method(horizontal))]
        #[unsafe(method_family = none)]
        pub fn horizontal(&self) -> Retained<MTLRasterizationRateSampleArray>;

        /// Bounds-checked access helper for vertical samples.
        #[unsafe(method(vertical))]
        #[unsafe(method_family = none)]
        pub fn vertical(&self) -> Retained<MTLRasterizationRateSampleArray>;

        /// Setter for `sampleCount`.
        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_sample_count(
            &self,
            sample_count: MTLSize,
        );
    );
}

impl MTLRasterizationRateLayerDescriptor {
    /// Creates a layer descriptor and copies the provided sample arrays.
    ///
    /// `horizontal.len()` must equal `sample_count.width`, and
    /// `vertical.len()` must equal `sample_count.height`.
    pub fn new_with_sample_count_horizontal_vertical(
        sample_count: MTLSize,
        horizontal: &[f32],
        vertical: &[f32],
    ) -> Retained<Self> {
        assert_eq!(horizontal.len(), sample_count.width, "horizontal sample count does not match sample_count.width");
        assert_eq!(vertical.len(), sample_count.height, "vertical sample count does not match sample_count.height");

        let horizontal = NonNull::from(horizontal).cast::<c_float>();
        let vertical = NonNull::from(vertical).cast::<c_float>();
        let allocated: Allocated<Self> = unsafe { msg_send![Self::class(), alloc] };
        Self::init_with_sample_count_horizontal_vertical(allocated, sample_count, horizontal, vertical)
    }
}
