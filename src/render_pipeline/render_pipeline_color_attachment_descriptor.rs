use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{BlendFactor, BlendOperation, ColorWriteMask};
use crate::PixelFormat;

extern_class!(
    /// Color attachment state for a render pipeline.
    #[unsafe(super(NSObject))]
    #[name = "MTLRenderPipelineColorAttachmentDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RenderPipelineColorAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for RenderPipelineColorAttachmentDescriptor {}
);

unsafe impl CopyingHelper for RenderPipelineColorAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for RenderPipelineColorAttachmentDescriptor {}
);

impl RenderPipelineColorAttachmentDescriptor {
    extern_methods!(
        /// Pixel format. Defaults to PixelFormat::Invalid
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        pub fn pixel_format(&self) -> PixelFormat;

        /// Setter for [`pixel_format`][Self::pixel_format].
        #[unsafe(method(setPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_pixel_format(&self, pixel_format: PixelFormat);

        /// Enable blending. Defaults to false.
        #[unsafe(method(isBlendingEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_blending_enabled(&self) -> bool;

        /// Setter for [`is_blending_enabled`][Self::is_blending_enabled].
        #[unsafe(method(setBlendingEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_blending_enabled(&self, enabled: bool);

        /// Defaults to BlendFactor::One
        #[unsafe(method(sourceRGBBlendFactor))]
        #[unsafe(method_family = none)]
        pub fn source_rgb_blend_factor(&self) -> BlendFactor;

        /// Setter for [`source_rgb_blend_factor`][Self::source_rgb_blend_factor].
        #[unsafe(method(setSourceRGBBlendFactor:))]
        #[unsafe(method_family = none)]
        pub fn set_source_rgb_blend_factor(&self, factor: BlendFactor);

        /// Defaults to BlendFactor::Zero
        #[unsafe(method(destinationRGBBlendFactor))]
        #[unsafe(method_family = none)]
        pub fn destination_rgb_blend_factor(&self) -> BlendFactor;

        /// Setter for [`destination_rgb_blend_factor`][Self::destination_rgb_blend_factor].
        #[unsafe(method(setDestinationRGBBlendFactor:))]
        #[unsafe(method_family = none)]
        pub fn set_destination_rgb_blend_factor(&self, factor: BlendFactor);

        /// Defaults to BlendOperation::Add
        #[unsafe(method(rgbBlendOperation))]
        #[unsafe(method_family = none)]
        pub fn rgb_blend_operation(&self) -> BlendOperation;

        /// Setter for [`rgb_blend_operation`][Self::rgb_blend_operation].
        #[unsafe(method(setRgbBlendOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_rgb_blend_operation(&self, op: BlendOperation);

        /// Defaults to BlendFactor::One
        #[unsafe(method(sourceAlphaBlendFactor))]
        #[unsafe(method_family = none)]
        pub fn source_alpha_blend_factor(&self) -> BlendFactor;

        /// Setter for [`source_alpha_blend_factor`][Self::source_alpha_blend_factor].
        #[unsafe(method(setSourceAlphaBlendFactor:))]
        #[unsafe(method_family = none)]
        pub fn set_source_alpha_blend_factor(&self, factor: BlendFactor);

        /// Defaults to BlendFactor::Zero
        #[unsafe(method(destinationAlphaBlendFactor))]
        #[unsafe(method_family = none)]
        pub fn destination_alpha_blend_factor(&self) -> BlendFactor;

        /// Setter for [`destination_alpha_blend_factor`][Self::destination_alpha_blend_factor].
        #[unsafe(method(setDestinationAlphaBlendFactor:))]
        #[unsafe(method_family = none)]
        pub fn set_destination_alpha_blend_factor(&self, factor: BlendFactor);

        /// Defaults to BlendOperation::Add
        #[unsafe(method(alphaBlendOperation))]
        #[unsafe(method_family = none)]
        pub fn alpha_blend_operation(&self) -> BlendOperation;

        /// Setter for [`alpha_blend_operation`][Self::alpha_blend_operation].
        #[unsafe(method(setAlphaBlendOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_alpha_blend_operation(&self, op: BlendOperation);

        /// Defaults to ColorWriteMask::All
        #[unsafe(method(writeMask))]
        #[unsafe(method_family = none)]
        pub fn write_mask(&self) -> ColorWriteMask;

        /// Setter for [`write_mask`][Self::write_mask].
        #[unsafe(method(setWriteMask:))]
        #[unsafe(method_family = none)]
        pub fn set_write_mask(&self, mask: ColorWriteMask);
    );
}
