use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// A Metal device that represents a GPU.
    ///
    /// See Apple's documentation for `MTLDevice` for details.
    #[name = "MTLDevice"]
    pub unsafe trait Device: NSObjectProtocol {}
);


