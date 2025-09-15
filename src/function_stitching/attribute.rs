use objc2::{extern_class, extern_conformance, extern_protocol, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Attribute applied to the produced stitched function.
    #[name = "MTLFunctionStitchingAttribute"]
    pub unsafe trait FunctionStitchingAttribute: NSObjectProtocol {}
);

extern_class!(
    /// Applies the `__attribute__((always_inline))` attribute to the produced stitched function.
    #[unsafe(super(NSObject))]
    #[name = "MTLFunctionStitchingAttributeAlwaysInline"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FunctionStitchingAttributeAlwaysInline;
);

extern_conformance!(
    unsafe impl FunctionStitchingAttribute for FunctionStitchingAttributeAlwaysInline {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for FunctionStitchingAttributeAlwaysInline {}
);
