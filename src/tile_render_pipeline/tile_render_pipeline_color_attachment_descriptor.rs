use objc2::{extern_class, extern_conformance, extern_methods, rc::{Allocated, Retained}, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::PixelFormat;

extern_class!(
    /// Descriptor for a tile render pipeline color attachment.
    #[unsafe(super(NSObject))]
    #[name = "MTLTileRenderPipelineColorAttachmentDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TileRenderPipelineColorAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for TileRenderPipelineColorAttachmentDescriptor {}
);

unsafe impl CopyingHelper for TileRenderPipelineColorAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for TileRenderPipelineColorAttachmentDescriptor {}
);

impl TileRenderPipelineColorAttachmentDescriptor {
    extern_methods!(
        /// Pixel format for this color attachment.
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        pub fn pixel_format(&self) -> PixelFormat;

        /// Setter for [`pixel_format`][Self::pixel_format].
        #[unsafe(method(setPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_pixel_format(&self, fmt: PixelFormat);
    );
}

impl TileRenderPipelineColorAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}


