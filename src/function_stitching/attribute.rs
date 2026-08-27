use objc2::{
    extern_class, extern_conformance, extern_methods, extern_protocol,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Attribute applied to the produced stitched function.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLFunctionStitchingAttribute` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLFunctionStitchingAttribute: NSObjectProtocol {}
);

extern_class!(
    /// Applies the `__attribute__((always_inline))` attribute to the produced stitched function.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingAttributeAlwaysInline;
);

extern_conformance!(
    unsafe impl MTLFunctionStitchingAttribute for MTLFunctionStitchingAttributeAlwaysInline {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionStitchingAttributeAlwaysInline {}
);

/// Methods inherited from `NSObject`.
impl MTLFunctionStitchingAttributeAlwaysInline {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
