use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Container of logs emitted by Metal (bridged from `MTLLogContainer`).
    pub unsafe trait MTLLogContainer: NSObjectProtocol {}
);
