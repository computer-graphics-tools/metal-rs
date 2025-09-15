use objc2::{
    extern_class, extern_conformance, extern_methods, extern_protocol,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use super::FunctionStitchingFunctionNode;

extern_class!(
    /// Function graph describing a DAG used to produce a stitched function.
    #[unsafe(super(NSObject))]
    #[name = "MTLFunctionStitchingGraph"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FunctionStitchingGraph;
);

extern_conformance!(
    unsafe impl NSCopying for FunctionStitchingGraph {}
);

unsafe impl CopyingHelper for FunctionStitchingGraph {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for FunctionStitchingGraph {}
);

extern_protocol!(
    #[name = "MTLFunctionStitchingAttribute"]
    pub unsafe trait __HiddenAttrProtocol: NSObjectProtocol {}
);

impl FunctionStitchingGraph {
    extern_methods!(
        #[unsafe(method(functionName))]
        #[unsafe(method_family = none)]
        pub unsafe fn function_name(&self) -> Retained<NSString>;

        /// Setter for [`function_name`][Self::function_name].
        #[unsafe(method(setFunctionName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_function_name(&self, function_name: &NSString);

        #[unsafe(method(nodes))]
        #[unsafe(method_family = none)]
        pub unsafe fn nodes(&self) -> Retained<NSArray<FunctionStitchingFunctionNode>>;

        /// Setter for [`nodes`][Self::nodes].
        #[unsafe(method(setNodes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_nodes(&self, nodes: &NSArray<FunctionStitchingFunctionNode>);

        #[unsafe(method(outputNode))]
        #[unsafe(method_family = none)]
        pub unsafe fn output_node(&self) -> Option<Retained<FunctionStitchingFunctionNode>>;

        /// Setter for [`output_node`][Self::output_node].
        #[unsafe(method(setOutputNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_output_node(&self, output_node: Option<&FunctionStitchingFunctionNode>);

        #[unsafe(method(attributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributes(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn super::FunctionStitchingAttribute>>>;

        /// Setter for [`attributes`][Self::attributes].
        #[unsafe(method(setAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_attributes(
            &self,
            attributes: &NSArray<ProtocolObject<dyn super::FunctionStitchingAttribute>>,
        );

        #[unsafe(method(initWithFunctionName:nodes:outputNode:attributes:))]
        #[unsafe(method_family = init)]
        pub unsafe fn init_with_function_name_nodes_output_node_attributes(
            this: Allocated<Self>,
            function_name: &NSString,
            nodes: &NSArray<FunctionStitchingFunctionNode>,
            output_node: Option<&FunctionStitchingFunctionNode>,
            attributes: &NSArray<ProtocolObject<dyn super::FunctionStitchingAttribute>>,
        ) -> Retained<Self>;
    );
}
