// Example demonstrating the use of MTLLinkedFunctions
// This allows specifying a set of functions to be linked into a pipeline

use metal_rs::foundation::NSArray;
use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLLinkedFunctions, MTLComputePipelineDescriptor};

const SHADER_SRC: &str = r#"
#include <metal_stdlib>
using namespace metal;

// Main compute function that calls our helper function
kernel void compute_main(device float *inBuffer [[buffer(0)]],
                          device float *outBuffer [[buffer(1)]],
                          uint id [[thread_position_in_grid]]) {
    // Call the helper function
    outBuffer[id] = square_value(inBuffer[id]);
}

// Helper function that will be linked
float square_value(float input) {
    return input * input;
}
"#;

fn main() {
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    
    // Create a library from Metal shader source
    let compile_options = Default::default();
    let library = device.new_library_with_source(SHADER_SRC, &compile_options).unwrap();
    
    // Get compute and helper functions
    let compute_function = library.get_function("compute_main").unwrap();
    let helper_function = library.get_function("square_value").unwrap();
    
    // Create a compute pipeline descriptor
    let pipeline_descriptor = MTLComputePipelineDescriptor::new();
    pipeline_descriptor.set_compute_function(&compute_function);
    pipeline_descriptor.set_label("Linked Function Example Pipeline");
    
    // Create linked functions
    let linked_functions = MTLLinkedFunctions::new();
    // Create an array of functions to link
    let functions = NSArray::from_refs_slice(&[&helper_function]);
    linked_functions.set_functions(&functions);
    
    // Set linked functions on pipeline descriptor
    pipeline_descriptor.set_linked_functions(&linked_functions);
    
    // Create compute pipeline state
    let pipeline_state = device.new_compute_pipeline_state_with_descriptor(
        &pipeline_descriptor, 
        metal_rs::metal::MTLPipelineOption::None
    ).unwrap();
    
    println!("Created compute pipeline state with linked functions: {:#?}", pipeline_state);
    println!("This example demonstrates how to use MTLLinkedFunctions to link helper functions into a Metal pipeline.");
}