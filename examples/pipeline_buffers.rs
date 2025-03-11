//! Example demonstrating the use of MTLPipeline types.
//!
//! This example shows how to create and configure pipeline buffer descriptors
//! for use with render and compute pipelines.

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice, MTLMutability,
    MTLPipelineBufferDescriptor, MTLComputePipelineDescriptor
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a compute pipeline descriptor
    let pipeline_descriptor = MTLComputePipelineDescriptor::new();
    pipeline_descriptor.set_label("Pipeline Buffer Example");
    
    // Get the buffers descriptor array
    let buffers = pipeline_descriptor.buffers();
    
    // Configure buffers at different indices
    let buffer0 = buffers.object(0);
    buffer0.set_mutability(MTLMutability::Immutable);
    println!("Configure buffer 0 with immutable mutability");
    
    let buffer1 = buffers.object(1);
    buffer1.set_mutability(MTLMutability::Mutable);
    println!("Configure buffer 1 with mutable mutability");
    
    // Read back the configurations
    println!("\nBuffer configurations:");
    println!("Buffer 0: Index = {}, Mutability = {:?}", 
            buffer0.index(), buffer0.mutability());
    println!("Buffer 1: Index = {}, Mutability = {:?}", 
            buffer1.index(), buffer1.mutability());
    
    // Create a standalone buffer descriptor
    println!("\nCreating standalone buffer descriptor");
    let standalone_desc = MTLPipelineBufferDescriptor::new();
    standalone_desc.set_index(5);
    standalone_desc.set_mutability(MTLMutability::Immutable);
    
    println!("Standalone buffer descriptor:");
    println!("Index: {}", standalone_desc.index());
    println!("Mutability: {:?}", standalone_desc.mutability());
    
    println!("\nPipeline buffer configuration complete");
}