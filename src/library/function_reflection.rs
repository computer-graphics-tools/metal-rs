use objc2::{extern_class, extern_conformance, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Reflection info for a function in a Metal library.
    #[unsafe(super(NSObject))]
    #[name = "MTLFunctionReflection"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FunctionReflection;
);

unsafe impl Send for FunctionReflection {}
unsafe impl Sync for FunctionReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for FunctionReflection {}
);
