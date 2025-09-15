use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{ClearColor, RenderPassAttachmentDescriptor};
use crate::Texture;

extern_class!(
    /// Color attachment descriptor for a render pass.
    #[unsafe(super(RenderPassAttachmentDescriptor, NSObject))]
    #[name = "MTLRenderPassColorAttachmentDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RenderPassColorAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for RenderPassColorAttachmentDescriptor {}
);

unsafe impl CopyingHelper for RenderPassColorAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for RenderPassColorAttachmentDescriptor {}
);

impl RenderPassColorAttachmentDescriptor {
    extern_methods!(
        /// The texture used for this color attachment.
        #[unsafe(method(texture))]
        #[unsafe(method_family = none)]
        pub fn texture(&self) -> Option<Retained<ProtocolObject<dyn Texture>>>;

        /// Setter for [`texture`][Self::texture].
        #[unsafe(method(setTexture:))]
        #[unsafe(method_family = none)]
        pub fn set_texture(&self, texture: Option<&ProtocolObject<dyn Texture>>);

        /// The clear color to be used if the load action is Clear.
        #[unsafe(method(clearColor))]
        #[unsafe(method_family = none)]
        pub fn clear_color(&self) -> ClearColor;

        /// Setter for [`clear_color`][Self::clear_color].
        #[unsafe(method(setClearColor:))]
        #[unsafe(method_family = none)]
        pub fn set_clear_color(&self, clear_color: ClearColor);
    );
}

impl Default for RenderPassColorAttachmentDescriptor {
    fn default() -> Self {
        // Not constructible directly in Rust; provided for API symmetry only.
        panic!("Use Objective-C allocation to create instances");
    }
}
