mod attribute;
mod function_node;
mod graph;
mod input_node;
mod options;
mod stitched_library_descriptor;

pub use attribute::{MTLFunctionStitchingAttribute, MTLFunctionStitchingAttributeAlwaysInline};
pub use function_node::MTLFunctionStitchingFunctionNode;
pub use graph::MTLFunctionStitchingGraph;
pub use input_node::{MTLFunctionStitchingInputNode, MTLFunctionStitchingNode};
pub use options::MTLStitchedLibraryOptions;
pub use stitched_library_descriptor::MTLStitchedLibraryDescriptor;

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use objc2::{
        rc::{Allocated, Retained},
        runtime::ProtocolObject,
    };
    use objc2_foundation::{NSCopying, NSObjectProtocol};

    use super::*;

    fn assert_copying<T: NSCopying + ?Sized>() {}
    fn assert_nsobject<T: NSObjectProtocol + ?Sized>() {}
    fn assert_attribute<T: MTLFunctionStitchingAttribute + ?Sized>() {}
    fn assert_node<T: MTLFunctionStitchingNode + ?Sized>() {}

    #[test]
    fn type_inheritance_and_conformances_match_the_header() {
        assert_nsobject::<dyn MTLFunctionStitchingAttribute>();
        assert_nsobject::<dyn MTLFunctionStitchingNode>();
        assert_copying::<dyn MTLFunctionStitchingNode>();

        assert_nsobject::<MTLFunctionStitchingAttributeAlwaysInline>();
        assert_attribute::<MTLFunctionStitchingAttributeAlwaysInline>();

        assert_nsobject::<MTLFunctionStitchingInputNode>();
        assert_copying::<MTLFunctionStitchingInputNode>();
        assert_node::<MTLFunctionStitchingInputNode>();

        assert_nsobject::<MTLFunctionStitchingFunctionNode>();
        assert_copying::<MTLFunctionStitchingFunctionNode>();
        assert_node::<MTLFunctionStitchingFunctionNode>();

        assert_nsobject::<MTLFunctionStitchingGraph>();
        assert_copying::<MTLFunctionStitchingGraph>();
        assert_nsobject::<MTLStitchedLibraryDescriptor>();
        assert_copying::<MTLStitchedLibraryDescriptor>();
    }

    #[test]
    fn designated_initializers_are_public_with_checked_collection_inputs() {
        let _: fn() -> Retained<MTLFunctionStitchingAttributeAlwaysInline> =
            MTLFunctionStitchingAttributeAlwaysInline::new;

        let _: fn(Allocated<MTLFunctionStitchingInputNode>, usize) -> Retained<MTLFunctionStitchingInputNode> =
            MTLFunctionStitchingInputNode::init_with_argument_index;

        let _: fn(
            Allocated<MTLFunctionStitchingFunctionNode>,
            &str,
            &[&ProtocolObject<dyn MTLFunctionStitchingNode>],
            &[&MTLFunctionStitchingFunctionNode],
        ) -> Retained<MTLFunctionStitchingFunctionNode> =
            MTLFunctionStitchingFunctionNode::init_with_name_arguments_control_dependencies;

        let _: fn(
            Allocated<MTLFunctionStitchingGraph>,
            &str,
            &[&MTLFunctionStitchingFunctionNode],
            Option<&MTLFunctionStitchingFunctionNode>,
            &[&ProtocolObject<dyn MTLFunctionStitchingAttribute>],
        ) -> Retained<MTLFunctionStitchingGraph> =
            MTLFunctionStitchingGraph::init_with_function_name_nodes_output_node_attributes;
    }

    #[test]
    fn stitched_library_options_match_the_header_abi_and_values() {
        assert_eq!(size_of::<MTLStitchedLibraryOptions>(), size_of::<usize>());
        assert_eq!(align_of::<MTLStitchedLibraryOptions>(), align_of::<usize>());
        assert_eq!(MTLStitchedLibraryOptions::None.0, 0);
        assert_eq!(MTLStitchedLibraryOptions::FailOnBinaryArchiveMiss.0, 1 << 0);
        assert_eq!(MTLStitchedLibraryOptions::StoreLibraryInMetalPipelinesScript.0, 1 << 1);
    }
}
