//! Simple example demonstrating the MTLPipeline types.
//!
//! This example creates and demonstrates the basic functionality of MTLPipeline types.

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice, MTLMutability,
    MTLPipelineBufferDescriptor
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a standalone pipeline buffer descriptor
    println!("Creating pipeline buffer descriptor");
    let buffer_desc = MTLPipelineBufferDescriptor::new();
    
    // Configure it
    buffer_desc.set_index(1);
    buffer_desc.set_mutability(MTLMutability::Immutable);
    
    // Read back the configuration
    println!("Buffer descriptor configuration:");
    println!("Index: {}", buffer_desc.index());
    println!("Mutability: {:?}", buffer_desc.mutability());
    
    println!("\nPipeline buffer configuration complete");
}