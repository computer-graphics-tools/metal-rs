use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use super::FunctionStitchingNode;

extern_class!(
    /// Function node that calls a specified function.
    #[unsafe(super(NSObject))]
    #[name = "MTLFunctionStitchingFunctionNode"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FunctionStitchingFunctionNode;
);

extern_conformance!(
    unsafe impl FunctionStitchingNode for FunctionStitchingFunctionNode {}
);

extern_conformance!(
    unsafe impl NSCopying for FunctionStitchingFunctionNode {}
);

unsafe impl CopyingHelper for FunctionStitchingFunctionNode {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for FunctionStitchingFunctionNode {}
);

impl FunctionStitchingFunctionNode {
    extern_methods!(
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[unsafe(method(setName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_name(&self, name: &NSString);

        #[unsafe(method(arguments))]
        #[unsafe(method_family = none)]
        pub unsafe fn arguments(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn super::FunctionStitchingNode>>>;

        /// Setter for [`arguments`][Self::arguments].
        #[unsafe(method(setArguments:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_arguments(
            &self,
            arguments: &NSArray<ProtocolObject<dyn super::FunctionStitchingNode>>,
        );

        #[unsafe(method(controlDependencies))]
        #[unsafe(method_family = none)]
        pub unsafe fn control_dependencies(
            &self,
        ) -> Retained<NSArray<FunctionStitchingFunctionNode>>;

        /// Setter for [`control_dependencies`][Self::control_dependencies].
        #[unsafe(method(setControlDependencies:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_control_dependencies(
            &self,
            control_dependencies: &NSArray<FunctionStitchingFunctionNode>,
        );

        #[unsafe(method(initWithName:arguments:controlDependencies:))]
        #[unsafe(method_family = init)]
        pub unsafe fn init_with_name_arguments_control_dependencies(
            this: Allocated<Self>,
            name: &NSString,
            arguments: &NSArray<ProtocolObject<dyn super::FunctionStitchingNode>>,
            control_dependencies: &NSArray<FunctionStitchingFunctionNode>,
        ) -> Retained<Self>;
    );
}
