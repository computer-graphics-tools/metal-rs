use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Bridged protocol for `MTLDepthStencilState`.
    #[name = "MTLDepthStencilState"]
    pub unsafe trait DepthStencilState: NSObjectProtocol + Send + Sync {}
);


