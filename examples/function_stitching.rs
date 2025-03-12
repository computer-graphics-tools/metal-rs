//! Example demonstrating Metal function stitching.
//!
//! This example shows how to use Metal's function stitching API to dynamically compose
//! shader functions at runtime. Function stitching allows you to create specialized
//! shader functions based on runtime conditions, rather than having to compile every
//! possible variation ahead of time.
//!
//! Note: Function stitching requires macOS 13.0+ or iOS 16.0+.

use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCompileOptions};
use metal_rs::metal::{
    MTLFunctionStitchingGraph, MTLFunctionStitchingGraphRef,
    MTLFunctionStitchingFunctionNode, MTLFunctionStitchingFunctionNodeRef,
    MTLFunctionStitchingInputNode, MTLFunctionStitchingInputNodeRef,
    MTLFunctionStitchingNode, MTLFunctionStitchingNodeRef,
    MTLFunctionStitchingAttribute, MTLFunctionStitchingAttributeRef,
    MTLFunctionStitchingAttributeAlwaysInline, MTLFunctionStitchingAttributeAlwaysInlineRef,
    MTLStitchedLibraryDescriptor
};
use metal_rs::foundation::{NSArray, NSString};

fn main() {
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    
    // Check if the device supports stitched libraries
    if !device.supports_stitched_libraries() {
        println!("This device does not support stitched libraries. Requires macOS 13.0+ or iOS 16.0+");
        return;
    }
    
    // Define the Metal Shading Language source code for our functions
    // We'll define two simple functions: add and multiply
    let source = r#"
        #include <metal_stdlib>
        using namespace metal;
        
        // First function - adds two floats
        float add(float a, float b) {
            return a + b;
        }
        
        // Second function - multiplies two floats
        float multiply(float a, float b) {
            return a * b;
        }
        
        // Third function - combined operation
        float combined(float a, float b, float c) {
            return add(multiply(a, b), c);
        }
    "#;
    
    // Compile options for our shader
    let compile_options = MTLCompileOptions::new();
    
    // Create a library from the source code
    let library = match device.new_library_with_source(source, &compile_options) {
        Ok(lib) => lib,
        Err(e) => {
            println!("Error creating library: {}", e.0);
            return;
        }
    };
    
    println!("Successfully created library");
    println!("Available functions:");
    
    // Get functions from the library
    let add_function = match library.get_function("add") {
        Ok(f) => f,
        Err(e) => {
            println!("Error getting add function: {}", e);
            return;
        }
    };
    
    let multiply_function = match library.get_function("multiply") {
        Ok(f) => f,
        Err(e) => {
            println!("Error getting multiply function: {}", e);
            return;
        }
    };
    
    // Now we'll create a stitched function that combines these functions
    // Step 1: Create input nodes for our stitched function
    let input_node_a = MTLFunctionStitchingInputNode::new_with_argument_index(0);
    let input_node_b = MTLFunctionStitchingInputNode::new_with_argument_index(1);
    let input_node_c = MTLFunctionStitchingInputNode::new_with_argument_index(2);
    
    // Step 2: Create function nodes
    // Create a node for the multiply function, using inputs a and b
    let multiply_func_name = NSString::from_rust_str("multiply");
    
    // Create array for input nodes - need to explicitly use the input node reference type
    let input_a_ref: &MTLFunctionStitchingInputNodeRef = input_node_a.as_ref();
    let input_b_ref: &MTLFunctionStitchingInputNodeRef = input_node_b.as_ref();
    let multiply_args = NSArray::from_slice(&[input_a_ref, input_b_ref]);
    
    let multiply_node = MTLFunctionStitchingFunctionNode::new_with_name_arguments_control_dependencies(
        &multiply_func_name,
        &multiply_args,
        &NSArray::new()
    );
    
    // Create a node for the add function, using the result of multiply and input c
    let add_func_name = NSString::from_rust_str("add");
    // We need to cast the nodes to the same type using the base MTLFunctionStitchingNode type
    let multiply_node_ref: &dyn AsRef<MTLFunctionStitchingNodeRef> = &multiply_node;
    let input_node_c_ref: &dyn AsRef<MTLFunctionStitchingNodeRef> = &input_node_c;
    let add_args = NSArray::from_slice(&[multiply_node_ref.as_ref(), input_node_c_ref.as_ref()]);
    let add_node = MTLFunctionStitchingFunctionNode::new_with_name_arguments_control_dependencies(
        &add_func_name,
        &add_args,
        &NSArray::new()
    );
    
    // Step 3: Create a function stitching graph
    let output_func_name = NSString::from_rust_str("stitched_func");
    // We need to collect node references of the same type
    let input_a_ref: &dyn AsRef<MTLFunctionStitchingNodeRef> = &input_node_a;
    let input_b_ref: &dyn AsRef<MTLFunctionStitchingNodeRef> = &input_node_b;
    let input_c_ref: &dyn AsRef<MTLFunctionStitchingNodeRef> = &input_node_c;
    let multiply_ref: &dyn AsRef<MTLFunctionStitchingNodeRef> = &multiply_node;
    let add_ref: &dyn AsRef<MTLFunctionStitchingNodeRef> = &add_node;
    
    let node_refs = [
        input_a_ref.as_ref(), input_b_ref.as_ref(), input_c_ref.as_ref(),
        multiply_ref.as_ref(), add_ref.as_ref()
    ];
    let nodes = NSArray::from_slice(&node_refs);
    
    // Create an attribute to force inlining (optional)
    let inline_attr = MTLFunctionStitchingAttributeAlwaysInline::new();
    let inline_attr_ref: &MTLFunctionStitchingAttributeRef = inline_attr.as_ref();
    let attributes = NSArray::from_slice(&[inline_attr_ref]);
    
    let graph = MTLFunctionStitchingGraph::new_with_function_name_nodes_output_node_attributes(
        &output_func_name,
        &nodes,
        &add_node,
        &attributes
    );
    
    // Step 4: Create a stitched library descriptor
    let graphs = NSArray::from_refs_slice(&[&graph]);
    let descriptor = MTLStitchedLibraryDescriptor::new();
    descriptor.set_function_graphs(&graphs);
    
    // Step 5: Create the stitched library
    let stitched_library = match device.new_stitched_library_with_descriptor(&descriptor) {
        Ok(lib) => lib,
        Err(e) => {
            println!("Error creating stitched library: {}", e.0);
            return;
        }
    };
    
    println!("Successfully created stitched library");
    
    // Step 6: Get the stitched function
    let stitched_function = match stitched_library.get_function("stitched_func") {
        Ok(f) => f,
        Err(e) => {
            println!("Error getting stitched function: {}", e);
            return;
        }
    };
    
    println!("Successfully created stitched function: {}", stitched_function.name());
    println!("Function type: {:?}", stitched_function.function_type());
    
    // The stitched function is now ready to be used in a compute pipeline or other Metal operations
    println!("Function stitching example completed successfully");
}