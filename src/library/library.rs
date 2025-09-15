use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use super::{MTLFunction, MTLFunctionReflection, MTLLibraryType};

extern_protocol!(
    /// Metal library interface.
    pub unsafe trait MTLLibrary: NSObjectProtocol + Send + Sync {
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);

        #[unsafe(method(newFunctionWithName:))]
        #[unsafe(method_family = new)]
        fn new_function_with_name(
            &self,
            function_name: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        #[unsafe(method(reflectionForFunctionWithName:))]
        #[unsafe(method_family = none)]
        unsafe fn reflection_for_function_with_name(
            &self,
            function_name: &NSString,
        ) -> Option<Retained<MTLFunctionReflection>>;

        #[unsafe(method(functionNames))]
        #[unsafe(method_family = none)]
        fn function_names(&self) -> Retained<NSArray<NSString>>;

        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        unsafe fn r#type(&self) -> MTLLibraryType;

        #[unsafe(method(installName))]
        #[unsafe(method_family = none)]
        fn install_name(&self) -> Option<Retained<NSString>>;
    }
);
