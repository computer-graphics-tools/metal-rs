use objc2::{
    Encode, Encoding, RefEncode, extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLRenderPassAttachmentDescriptor;

/// Controls the MSAA depth resolve operation.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMultisampleDepthResolveFilter {
    Sample0 = 0,
    Min = 1,
    Max = 2,
}

#[deprecated(note = "use MTLMultisampleDepthResolveFilter")]
pub type MultisampleDepthResolveFilter = MTLMultisampleDepthResolveFilter;

unsafe impl Encode for MTLMultisampleDepthResolveFilter {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLMultisampleDepthResolveFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Depth attachment descriptor for a render pass.
    #[unsafe(super(MTLRenderPassAttachmentDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassDepthAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPassDepthAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPassDepthAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPassDepthAttachmentDescriptor {}
);

impl MTLRenderPassDepthAttachmentDescriptor {
    extern_methods!(
        /// The clear depth value to be used if the load action is Clear.
        #[unsafe(method(clearDepth))]
        #[unsafe(method_family = none)]
        pub fn clear_depth(&self) -> f64;

        /// Setter for [`clear_depth`][Self::clear_depth].
        #[unsafe(method(setClearDepth:))]
        #[unsafe(method_family = none)]
        pub fn set_clear_depth(
            &self,
            clear_depth: f64,
        );

        /// The filter to be used for depth multisample resolve. Defaults to Sample0.
        #[unsafe(method(depthResolveFilter))]
        #[unsafe(method_family = none)]
        pub fn depth_resolve_filter(&self) -> MTLMultisampleDepthResolveFilter;

        /// Setter for [`depth_resolve_filter`][Self::depth_resolve_filter].
        #[unsafe(method(setDepthResolveFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_resolve_filter(
            &self,
            filter: MTLMultisampleDepthResolveFilter,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLRenderPassDepthAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
