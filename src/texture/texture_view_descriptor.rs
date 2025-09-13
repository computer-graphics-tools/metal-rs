use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSRange};

use crate::{PixelFormat, TextureSwizzleChannels, TextureType};

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltextureviewdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[name = "MTLTextureViewDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TextureViewDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for TextureViewDescriptor {}
);

unsafe impl CopyingHelper for TextureViewDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for TextureViewDescriptor {}
);

impl TextureViewDescriptor {
    extern_methods!(
        /// A desired pixel format of a texture view.
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        pub fn pixel_format(&self) -> PixelFormat;

        /// Setter for [`pixelFormat`][Self::pixelFormat].
        #[unsafe(method(setPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_pixel_format(&self, pixel_format: PixelFormat);

        /// A desired texture view of a texture view.
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        pub fn texture_type(&self) -> TextureType;

        /// Setter for [`textureType`][Self::textureType].
        #[unsafe(method(setTextureType:))]
        #[unsafe(method_family = none)]
        pub fn set_texture_type(&self, texture_type: TextureType);

        /// A desired range of mip levels of a texture view.
        #[unsafe(method(levelRange))]
        #[unsafe(method_family = none)]
        pub fn level_range(&self) -> NSRange;

        /// Setter for [`levelRange`][Self::levelRange].
        #[unsafe(method(setLevelRange:))]
        #[unsafe(method_family = none)]
        pub fn set_level_range(&self, level_range: NSRange);

        /// A desired range of slices of a texture view.
        #[unsafe(method(sliceRange))]
        #[unsafe(method_family = none)]
        pub fn slice_range(&self) -> NSRange;

        /// Setter for [`sliceRange`][Self::sliceRange].
        #[unsafe(method(setSliceRange:))]
        #[unsafe(method_family = none)]
        pub fn set_slice_range(&self, slice_range: NSRange);

        /// A desired swizzle format of a texture view.
        #[unsafe(method(swizzle))]
        #[unsafe(method_family = none)]
        pub fn swizzle(&self) -> TextureSwizzleChannels;

        /// Setter for [`swizzle`][Self::swizzle].
        #[unsafe(method(setSwizzle:))]
        #[unsafe(method_family = none)]
        pub fn set_swizzle(&self, swizzle: TextureSwizzleChannels);
    );
}

/// Methods declared on superclass `NSObject`.
impl TextureViewDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
