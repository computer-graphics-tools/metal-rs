//! Example demonstrating the use of MTLFunctionDescriptor for
//! creating specialized Metal shader functions.

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice,
    MTLFunctionDescriptor, MTLFunctionOptions,
    MTLFunctionConstantValues
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());

    // Create a Metal library from source code with multiple functions
    let shader_src = r#"
    #include <metal_stdlib>
    using namespace metal;

    constant bool UseTexture [[function_constant(0)]];
    constant float ColorMultiplier [[function_constant(1)]];

    kernel void compute_function(device float4* output [[buffer(0)]],
                                uint id [[thread_position_in_grid]]) {
        if (UseTexture) {
            // This would normally sample from a texture
            output[id] = float4(1.0, 0.0, 0.0, 1.0) * ColorMultiplier;
        } else {
            // Just use a constant color
            output[id] = float4(0.0, 0.0, 1.0, 1.0) * ColorMultiplier;
        }
    }

    vertex float4 vertex_function(uint vertexID [[vertex_id]]) {
        const float4 positions[3] = {
            float4(-0.5, -0.5, 0.0, 1.0),
            float4( 0.0,  0.5, 0.0, 1.0),
            float4( 0.5, -0.5, 0.0, 1.0)
        };
        return positions[vertexID];
    }

    fragment float4 fragment_function() {
        return float4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let library = device.new_library_with_source(shader_src, &Default::default())
        .expect("Failed to create Metal library");

    // Get all available function names in the library
    let function_names = library.function_names();
    println!("Available functions in library: {:?}", function_names);

    // Create a function descriptor for the compute function
    let function_descriptor = MTLFunctionDescriptor::function_descriptor();
    function_descriptor.as_ref().set_name("compute_function");
    function_descriptor.as_ref().set_specialized_name("compute_function_specialized");
    function_descriptor.as_ref().set_options(MTLFunctionOptions::CompileToBinary);

    // Set up function constants
    let constant_values = MTLFunctionConstantValues::new();
    
    // Set UseTexture to true
    constant_values.set_bool_constant(true, 0);
    
    // Set ColorMultiplier to 0.5
    constant_values.set_float_constant(0.5, 1);
    
    // Set the constant values in the function descriptor
    function_descriptor.as_ref().set_constant_values(Some(&constant_values));
    
    // Create the specialized function
    let specialized_function = library.new_function_with_descriptor(&function_descriptor)
        .expect("Failed to create specialized function");
        
    println!("Created specialized function: {}", specialized_function.name());
}