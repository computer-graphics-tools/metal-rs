use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use super::MTLFunctionStitchingNode;

extern_class!(
    /// Function node that calls a specified function.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingFunctionNode;
);

extern_conformance!(
    unsafe impl MTLFunctionStitchingNode for MTLFunctionStitchingFunctionNode {}
);

extern_conformance!(
    unsafe impl NSCopying for MTLFunctionStitchingFunctionNode {}
);

unsafe impl CopyingHelper for MTLFunctionStitchingFunctionNode {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionStitchingFunctionNode {}
);

impl MTLFunctionStitchingFunctionNode {
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
        ) -> Retained<NSArray<ProtocolObject<dyn super::MTLFunctionStitchingNode>>>;

        /// Setter for [`arguments`][Self::arguments].
        #[unsafe(method(setArguments:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_arguments(
            &self,
            arguments: &NSArray<ProtocolObject<dyn super::MTLFunctionStitchingNode>>,
        );

        #[unsafe(method(controlDependencies))]
        #[unsafe(method_family = none)]
        pub unsafe fn control_dependencies(
            &self,
        ) -> Retained<NSArray<MTLFunctionStitchingFunctionNode>>;

        /// Setter for [`control_dependencies`][Self::control_dependencies].
        #[unsafe(method(setControlDependencies:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_control_dependencies(
            &self,
            control_dependencies: &NSArray<MTLFunctionStitchingFunctionNode>,
        );

        #[unsafe(method(initWithName:arguments:controlDependencies:))]
        #[unsafe(method_family = init)]
        pub unsafe fn init_with_name_arguments_control_dependencies(
            this: Allocated<Self>,
            name: &NSString,
            arguments: &NSArray<ProtocolObject<dyn super::MTLFunctionStitchingNode>>,
            control_dependencies: &NSArray<MTLFunctionStitchingFunctionNode>,
        ) -> Retained<Self>;
    );
}
