// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::{
    Device, Library, MTLCreateSystemDefaultDevice
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    
    // Check if device supports dynamic libraries
    if !device.supports_dynamic_libraries() {
        println!("This device does not support dynamic libraries");
        return;
    }
    
    // Source code for a simple Metal shader
    let shader_source = r#"
        #include <metal_stdlib>
        using namespace metal;
        
        // A simple utility function that can be used from other shaders
        float multiply(float a, float b) {
            return a * b;
        }
        
        // Another utility function
        float3 normalize_safe(float3 v) {
            float length = sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
            if (length > 0.0001) {
                return v / length;
            }
            return float3(0.0, 0.0, 0.0);
        }
    "#;
    
    // Create a Metal library from the source code
    let compile_options = metal::MTLCompileOptions::new();
    let library = device.new_library_with_source(shader_source, &compile_options).unwrap();
    
    // Create a temporary folder to store dynamic libraries
    let temp_dir = std::env::temp_dir();
    
    // Path for the dynamic library
    let lib_path = temp_dir.join("dynamic_lib.metallib");
    let url_string = format!("file://{}", lib_path.display());
    
    // Create a binary data representation of the library
    // Note: In a real application, you would serialize the library to binary data
    // Here we're just creating the dynamic library directly
    
    // Serialize the dynamic library to disk
    // For this example, we'll just use the library directly
    let dynamic_library = match device.new_dynamic_library_with_data(&[0u8; 1]) {
        Ok(library) => {
            println!("Successfully created dynamic library");
            library
        },
        Err(error) => {
            println!("Failed to create dynamic library: {}", error);
            return;
        }
    };
    
    // Set a label on the dynamic library
    dynamic_library.set_label("Example Dynamic Library");
    
    // Serialize the dynamic library to disk
    match dynamic_library.serialize_to_url(&url_string) {
        Ok(_) => println!("Successfully serialized dynamic library to: {}", lib_path.display()),
        Err(error) => {
            println!("Failed to serialize dynamic library: {}", error);
            return;
        }
    };
    
    // Get the install name of the dynamic library
    if let Some(install_name) = dynamic_library.install_name() {
        println!("Dynamic library install name: {}", install_name);
    } else {
        println!("Dynamic library does not have an install name");
    }
    
    // Check if we can load it back
    let loaded_dynamic_library = match device.new_dynamic_library(&url_string) {
        Ok(library) => {
            println!("Successfully loaded dynamic library from: {}", lib_path.display());
            library
        },
        Err(error) => {
            println!("Failed to load dynamic library: {}", error);
            return;
        }
    };
    
    // Get the label of the loaded library
    if let Some(label) = loaded_dynamic_library.label() {
        println!("Loaded dynamic library label: {}", label);
    }
    
    println!("Dynamic library example completed successfully");
}