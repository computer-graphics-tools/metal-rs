use objc2::{extern_class, extern_conformance, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Reflection info for a render pipeline.
    #[unsafe(super(NSObject))]
    #[name = "MTLRenderPipelineReflection"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RenderPipelineReflection;
);

unsafe impl Send for RenderPipelineReflection {}
unsafe impl Sync for RenderPipelineReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for RenderPipelineReflection {}
);
