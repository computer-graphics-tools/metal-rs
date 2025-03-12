// Example demonstrating MTLFunction types and properties
// This example intentionally avoids MTLFunctionHandle as it's not implemented in the current API version

use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLFunctionType};
use std::thread::sleep;
use std::time::Duration;

const SHADER_SRC: &str = r#"
#include <metal_stdlib>
using namespace metal;

// Vertex shader function
vertex float4 vertex_main(uint vertexID [[vertex_id]]) {
    const float4 positions[3] = {
        float4(-0.5, -0.5, 0.0, 1.0),
        float4( 0.0,  0.5, 0.0, 1.0),
        float4( 0.5, -0.5, 0.0, 1.0)
    };
    return positions[vertexID];
}

// Fragment shader function
fragment float4 fragment_main() {
    return float4(1.0, 0.0, 0.0, 1.0);
}

// Compute shader function
kernel void compute_main(device float *data [[buffer(0)]],
                       uint id [[thread_position_in_grid]]) {
    data[id] = data[id] * 2.0;
}
"#;

fn main() {
    println!("Starting function example...");
    
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a library from Metal shader source
    println!("Compiling Metal shader code...");
    
    let compile_options = Default::default();
    match device.new_library_with_source(SHADER_SRC, &compile_options) {
        Ok(library) => {
            println!("Shader compilation successful");
            
            // Get the list of available functions
            let function_names = library.function_names();
            println!("\nAvailable functions in the library:");
            for name in &function_names {
                println!("  - {}", name);
            }
            
            // Retrieve each function and display its type
            println!("\nFunction Information:");
            println!("---------------------");
            
            for name in &function_names {
                match library.get_function(name) {
                    Ok(function) => {
                        println!("\nFunction: {}", function.name());
                        let function_type = function.function_type();
                        let type_name = match function_type {
                            MTLFunctionType::Vertex => "Vertex Shader",
                            MTLFunctionType::Fragment => "Fragment Shader",
                            MTLFunctionType::Kernel => "Compute Kernel",
                            MTLFunctionType::Visible => "Visible Function (Ray Tracing)",
                            MTLFunctionType::Intersection => "Intersection Function (Ray Tracing)",
                            MTLFunctionType::Object => "Object Function",
                            MTLFunctionType::Mesh => "Mesh Function",
                        };
                        println!("  Type: {} ({:?})", type_name, function_type);
                        
                        // The device property shows which device this function was created for
                        println!("  Device: {}", function.device().name());
                        
                        // The library property shows which library this function belongs to
                        println!("  Library: {}", function.library().label().unwrap_or_else(|| "Unlabeled".to_string()));
                    },
                    Err(err) => println!("Error getting function '{}': {:?}", name, err)
                }
                
                // Sleep briefly to allow output to be displayed
                sleep(Duration::from_millis(10));
            }
            
            println!("\nNote: MTLFunctionHandle is not directly available in the current Metal API version.");
            println!("This functionality is provided as a placeholder for future implementation.");
        },
        Err(err) => println!("Error creating library: {:?}", err)
    }
}