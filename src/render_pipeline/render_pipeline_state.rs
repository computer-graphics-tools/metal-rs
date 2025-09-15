use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Bridged protocol for `MTLRenderPipelineState`.
    pub unsafe trait MTLRenderPipelineState: NSObjectProtocol {}
);
