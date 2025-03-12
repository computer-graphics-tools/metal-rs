// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::{
    MTLArgument, MTLArgumentType, MTLBindingAccess, MTLCreateSystemDefaultDevice,
    MTLDataType, MTLLibrary, MTLPipelineOption
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    
    // Source code for a Metal shader with various argument types
    let shader_source = r#"
        #include <metal_stdlib>
        using namespace metal;
        
        struct Uniforms {
            float4 color;
            float2 scale;
            int counter;
        };
        
        kernel void reflection_example(device const Uniforms& uniforms [[buffer(0)]],
                                       device const float* inputBuffer [[buffer(1)]],
                                       device float* outputBuffer [[buffer(2)]],
                                       texture2d<float> inputTexture [[texture(0)]],
                                       sampler texSampler [[sampler(0)]],
                                       threadgroup float* sharedMem [[threadgroup(0)]],
                                       uint index [[thread_position_in_grid]])
        {
            // This function is just for demonstration of argument reflection
            outputBuffer[index] = inputBuffer[index] * uniforms.scale.x + uniforms.counter;
        }
    "#;
    
    // Create a Metal library from the source code
    let compile_options = metal::MTLCompileOptions::new();
    let library = device.new_library_with_source(shader_source, &compile_options).unwrap();
    
    // Get the function by name
    let function = library.get_function("reflection_example", None).unwrap();
    
    // Get the function reflection data
    // Note: In a real application, you would use MTLComputePipelineState.function_by_index(0)
    // to get the real MTLArgument objects. This is just a simplified example.
    println!("Function Reflection Example:");
    println!("---------------------------");
    println!("Function name: reflection_example");
    println!();
    
    println!("Expected argument reflections (if available):");
    println!("- Buffer 0: Uniforms struct (containing float4 color, float2 scale, int counter)");
    println!("- Buffer 1: float array (inputBuffer)");
    println!("- Buffer 2: float array (outputBuffer)");
    println!("- Texture 0: texture2d<float>");
    println!("- Sampler 0: sampler");
    println!("- Threadgroup 0: float array (sharedMem)");
    println!();
    
    println!("To access actual argument reflection data:");
    println!("1. Create a compute pipeline state from this function");
    println!("2. Use pipeline_state.function_by_index(0) to get the function");
    println!("3. Iterate through the function's arguments to inspect their types");
    println!();
    
    // Show how you would create a pipeline and use reflection in a real scenario
    let pipeline_descriptor = metal::MTLComputePipelineDescriptor::new();
    pipeline_descriptor.set_compute_function(&function);
    
    let pipeline_state = device.new_compute_pipeline_state_with_descriptor(
        &pipeline_descriptor,
        metal::MTLPipelineOption::ArgumentInfo
    ).expect("Failed to create compute pipeline state");
    
    // Now we can access the reflection data
    println!("Actual argument reflection data:");
    println!("----------------------------------");
    
    let function = pipeline_state.function_by_index(0).unwrap();
    
    // Print general function info
    println!("Function name: {}", function.name());
    println!();
    
    // Get the arguments
    // Use our newly implemented function to get the arguments
    if let Some(arguments) = function.arguments() {
        println!("Arguments:");
        for (i, arg) in arguments.iter().enumerate() {
            println!("Argument {}: {}", i, arg.name());
            println!("  Type: {:?}", arg.type_());
            println!("  Access: {:?}", arg.access());
            println!("  Index: {}", arg.index());
            println!("  Active: {}", arg.active());
            
            match arg.type_() {
                MTLArgumentType::Buffer => {
                    println!("  Buffer Details:");
                    println!("    Alignment: {} bytes", arg.buffer_alignment());
                    println!("    Data Size: {} bytes", arg.buffer_data_size());
                    println!("    Data Type: {:?}", arg.buffer_data_type());
                    
                    if let Some(struct_type) = arg.buffer_struct_type() {
                        println!("    Struct Members:");
                        for member in struct_type.members() {
                            println!("      {}: {:?} (offset: {} bytes)", 
                                     member.name(), member.data_type(), member.offset());
                        }
                    }
                },
                MTLArgumentType::Texture => {
                    println!("  Texture Details:");
                    println!("    Texture Type: {:?}", arg.texture_type());
                    println!("    Data Type: {:?}", arg.texture_data_type());
                    println!("    Is Depth Texture: {}", arg.is_depth_texture());
                },
                MTLArgumentType::Sampler => {
                    println!("  Sampler Details:");
                },
                MTLArgumentType::ThreadgroupMemory => {
                    println!("  Threadgroup Memory Details:");
                    println!("    Alignment: {} bytes", arg.threadgroup_memory_alignment());
                    println!("    Data Size: {} bytes", arg.threadgroup_memory_data_size());
                },
                _ => {}
            }
            println!();
        }
    } else {
        println!("No reflection data available for arguments.");
    }
    
    println!("Example completed successfully");
}