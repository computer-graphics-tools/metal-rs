use objc2::{
    ClassType, extern_class, extern_conformance, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use crate::MTLFunctionStitchingNode;

extern_class!(
    /// Function node that calls a specified function.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
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
    /// The name of the function to call.
    pub fn name(&self) -> String {
        let s: Retained<NSString> = unsafe { msg_send![self, name] };
        s.to_string()
    }

    /// Setter for [`name`][Self::name].
    pub fn set_name(
        &self,
        name: &str,
    ) {
        unsafe {
            let _: () = msg_send![self, setName: &*NSString::from_str(name)];
        }
    }

    pub fn arguments(&self) -> Box<[Retained<ProtocolObject<dyn MTLFunctionStitchingNode>>]> {
        let array: Retained<NSArray<ProtocolObject<dyn MTLFunctionStitchingNode>>> =
            unsafe { msg_send![self, arguments] };
        array.to_vec().into_boxed_slice()
    }

    /// Setter for [`arguments`][Self::arguments].
    pub fn set_arguments(
        &self,
        arguments: &[&ProtocolObject<dyn MTLFunctionStitchingNode>],
    ) {
        let arguments = NSArray::from_slice(arguments);
        unsafe {
            let _: () = msg_send![self, setArguments: &*arguments];
        }
    }

    pub fn control_dependencies(&self) -> Box<[Retained<MTLFunctionStitchingFunctionNode>]> {
        let array: Retained<NSArray<MTLFunctionStitchingFunctionNode>> =
            unsafe { msg_send![self, controlDependencies] };
        array.to_vec().into_boxed_slice()
    }

    /// Setter for [`control_dependencies`][Self::control_dependencies].
    pub fn set_control_dependencies(
        &self,
        control_dependencies: &[&MTLFunctionStitchingFunctionNode],
    ) {
        let control_dependencies = NSArray::from_slice(control_dependencies);
        unsafe {
            let _: () = msg_send![self, setControlDependencies: &*control_dependencies];
        }
    }

    /// Initializes a function node with its call and dependency graph.
    pub fn init_with_name_arguments_control_dependencies(
        this: Allocated<Self>,
        name: &str,
        arguments: &[&ProtocolObject<dyn MTLFunctionStitchingNode>],
        control_dependencies: &[&MTLFunctionStitchingFunctionNode],
    ) -> Retained<Self> {
        let name = NSString::from_str(name);
        let arguments = NSArray::from_slice(arguments);
        let control_dependencies = NSArray::from_slice(control_dependencies);
        unsafe {
            msg_send![
                this,
                initWithName: &*name,
                arguments: &*arguments,
                controlDependencies: &*control_dependencies,
            ]
        }
    }

    /// Creates a function node with its call and dependency graph.
    pub fn new(
        name: &str,
        arguments: &[&ProtocolObject<dyn MTLFunctionStitchingNode>],
        control_dependencies: &[&MTLFunctionStitchingFunctionNode],
    ) -> Retained<Self> {
        let class = Self::class();
        let allocated: Allocated<Self> = unsafe { msg_send![class, alloc] };
        Self::init_with_name_arguments_control_dependencies(allocated, name, arguments, control_dependencies)
    }
}
