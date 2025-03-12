// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::{
    BinaryArchiveDescriptor, ComputePipelineDescriptor, Device, Function, Library, 
    MTLCreateSystemDefaultDevice
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    
    // Source code for a simple Metal shader
    let shader_source = r#"
        #include <metal_stdlib>
        using namespace metal;
        
        kernel void add_arrays(device const float* inA,
                               device const float* inB,
                               device float* result,
                               uint index [[thread_position_in_grid]])
        {
            result[index] = inA[index] + inB[index];
        }
    "#;
    
    // Create a Metal library from the source code
    let compile_options = metal::MTLCompileOptions::new();
    let library = device.new_library_with_source(shader_source, &compile_options).unwrap();
    
    // Get the kernel function
    let function = library.get_function("add_arrays", None).unwrap();
    
    // Create a temporary folder to store binary archives
    let temp_dir = std::env::temp_dir();
    
    // Path for the binary archive
    let archive_path = temp_dir.join("binary_archive.metallib");
    let url_string = format!("file://{}", archive_path.display());
    
    // Create a binary archive descriptor
    let descriptor = BinaryArchiveDescriptor::new();
    
    // Create a binary archive from the descriptor
    let binary_archive = match device.new_binary_archive_with_descriptor(&descriptor) {
        Ok(archive) => archive,
        Err(error) => {
            println!("Failed to create binary archive: {}", error);
            return;
        }
    };
    
    // Set a label on the binary archive
    binary_archive.set_label("Example Binary Archive");
    
    // Create a compute pipeline descriptor
    let compute_pipeline_descriptor = ComputePipelineDescriptor::new();
    compute_pipeline_descriptor.set_compute_function(&function);
    
    // Add the compute pipeline functions to the binary archive
    match binary_archive.add_compute_pipeline_functions(&compute_pipeline_descriptor) {
        Ok(_) => println!("Successfully added compute pipeline functions to binary archive"),
        Err(error) => {
            println!("Failed to add compute pipeline functions: {}", error);
            return;
        }
    }
    
    // Serialize the binary archive to disk
    match binary_archive.serialize_to_url(&url_string) {
        Ok(_) => println!("Successfully serialized binary archive to: {}", archive_path.display()),
        Err(error) => {
            println!("Failed to serialize binary archive: {}", error);
            return;
        }
    }
    
    // Check if we can load it back
    let descriptor = BinaryArchiveDescriptor::new();
    descriptor.set_url(&url_string);
    
    let loaded_binary_archive = match device.new_binary_archive_with_descriptor(&descriptor) {
        Ok(archive) => {
            println!("Successfully loaded binary archive from: {}", archive_path.display());
            archive
        },
        Err(error) => {
            println!("Failed to load binary archive: {}", error);
            return;
        }
    };
    
    // Get the label of the loaded archive
    if let Some(label) = loaded_binary_archive.label() {
        println!("Loaded binary archive label: {}", label);
    }
    
    println!("Binary archive example completed successfully");
}