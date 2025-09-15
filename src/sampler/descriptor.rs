use core::ffi::c_float;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::{AddressMode, BorderColor, MinMagFilter, MipFilter, ReductionMode};

extern_class!(
    /// A mutable descriptor used to configure a sampler.
    #[unsafe(super(NSObject))]
    #[name = "MTLSamplerDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SamplerDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for SamplerDescriptor {}
);

unsafe impl CopyingHelper for SamplerDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SamplerDescriptor {}
);

impl SamplerDescriptor {
    extern_methods!(
        #[unsafe(method(minFilter))]
        #[unsafe(method_family = none)]
        pub fn min_filter(&self) -> MinMagFilter;

        #[unsafe(method(setMinFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_min_filter(&self, min_filter: MinMagFilter);

        #[unsafe(method(magFilter))]
        #[unsafe(method_family = none)]
        pub fn mag_filter(&self) -> MinMagFilter;

        #[unsafe(method(setMagFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_mag_filter(&self, mag_filter: MinMagFilter);

        #[unsafe(method(mipFilter))]
        #[unsafe(method_family = none)]
        pub fn mip_filter(&self) -> MipFilter;

        #[unsafe(method(setMipFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_mip_filter(&self, mip_filter: MipFilter);

        #[unsafe(method(maxAnisotropy))]
        #[unsafe(method_family = none)]
        pub fn max_anisotropy(&self) -> usize;

        #[unsafe(method(setMaxAnisotropy:))]
        #[unsafe(method_family = none)]
        pub fn set_max_anisotropy(&self, max_anisotropy: usize);

        #[unsafe(method(sAddressMode))]
        #[unsafe(method_family = none)]
        pub fn s_address_mode(&self) -> AddressMode;

        #[unsafe(method(setSAddressMode:))]
        #[unsafe(method_family = none)]
        pub fn set_s_address_mode(&self, mode: AddressMode);

        #[unsafe(method(tAddressMode))]
        #[unsafe(method_family = none)]
        pub fn t_address_mode(&self) -> AddressMode;

        #[unsafe(method(setTAddressMode:))]
        #[unsafe(method_family = none)]
        pub fn set_t_address_mode(&self, mode: AddressMode);

        #[unsafe(method(rAddressMode))]
        #[unsafe(method_family = none)]
        pub fn r_address_mode(&self) -> AddressMode;

        #[unsafe(method(setRAddressMode:))]
        #[unsafe(method_family = none)]
        pub fn set_r_address_mode(&self, mode: AddressMode);

        #[unsafe(method(borderColor))]
        #[unsafe(method_family = none)]
        pub fn border_color(&self) -> BorderColor;

        #[unsafe(method(setBorderColor:))]
        #[unsafe(method_family = none)]
        pub fn set_border_color(&self, color: BorderColor);

        #[unsafe(method(reductionMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn reduction_mode(&self) -> ReductionMode;

        #[unsafe(method(setReductionMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_reduction_mode(&self, mode: ReductionMode);

        #[unsafe(method(normalizedCoordinates))]
        #[unsafe(method_family = none)]
        pub fn normalized_coordinates(&self) -> bool;

        #[unsafe(method(setNormalizedCoordinates:))]
        #[unsafe(method_family = none)]
        pub fn set_normalized_coordinates(&self, normalized: bool);

        #[unsafe(method(lodMinClamp))]
        #[unsafe(method_family = none)]
        pub fn lod_min_clamp(&self) -> c_float;

        #[unsafe(method(setLodMinClamp:))]
        #[unsafe(method_family = none)]
        pub fn set_lod_min_clamp(&self, v: c_float);

        #[unsafe(method(lodMaxClamp))]
        #[unsafe(method_family = none)]
        pub fn lod_max_clamp(&self) -> c_float;

        #[unsafe(method(setLodMaxClamp:))]
        #[unsafe(method_family = none)]
        pub fn set_lod_max_clamp(&self, v: c_float);

        #[unsafe(method(lodAverage))]
        #[unsafe(method_family = none)]
        pub fn lod_average(&self) -> bool;

        #[unsafe(method(setLodAverage:))]
        #[unsafe(method_family = none)]
        pub fn set_lod_average(&self, v: bool);

        #[unsafe(method(lodBias))]
        #[unsafe(method_family = none)]
        pub unsafe fn lod_bias(&self) -> c_float;

        #[unsafe(method(setLodBias:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_lod_bias(&self, v: c_float);

        #[unsafe(method(supportArgumentBuffers))]
        #[unsafe(method_family = none)]
        pub fn support_argument_buffers(&self) -> bool;

        #[unsafe(method(setSupportArgumentBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_support_argument_buffers(&self, v: bool);

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);
    );
}

/// Methods declared on superclass `NSObject`.
impl SamplerDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
