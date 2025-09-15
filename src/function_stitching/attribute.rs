use objc2::{extern_class, extern_conformance, extern_protocol, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Attribute applied to the produced stitched function.
    pub unsafe trait MTLFunctionStitchingAttribute: NSObjectProtocol {}
);

extern_class!(
    /// Applies the `__attribute__((always_inline))` attribute to the produced stitched function.
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
