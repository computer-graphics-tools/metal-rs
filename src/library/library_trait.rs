use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use super::{Function, FunctionReflection, LibraryType};

extern_protocol!(
    /// Metal library interface.
    #[name = "MTLLibrary"]
    pub unsafe trait Library: NSObjectProtocol + Send + Sync {
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
        ) -> Option<Retained<ProtocolObject<dyn Function>>>;

        #[unsafe(method(reflectionForFunctionWithName:))]
        #[unsafe(method_family = none)]
        unsafe fn reflection_for_function_with_name(
            &self,
            function_name: &NSString,
        ) -> Option<Retained<FunctionReflection>>;

        #[unsafe(method(functionNames))]
        #[unsafe(method_family = none)]
        fn function_names(&self) -> Retained<NSArray<NSString>>;

        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        unsafe fn r#type(&self) -> LibraryType;

        #[unsafe(method(installName))]
        #[unsafe(method_family = none)]
        fn install_name(&self) -> Option<Retained<NSString>>;
    }
);
