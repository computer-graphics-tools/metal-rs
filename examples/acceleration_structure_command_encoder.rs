//! Example demonstrating how to use acceleration structure command encoder
//! 
//! This example shows how to create and build a basic acceleration structure
//! using the MTLAccelerationStructureCommandEncoder.

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice,
    MTLAccelerationStructureDescriptor,
    MTLAccelerationStructureUsage,
    MTLResourceOptions
};

fn main() {
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a command queue
    let command_queue = device.new_command_queue();
    
    // Create a command buffer
    let command_buffer = command_queue.new_command_buffer();
    command_buffer.set_label("Acceleration Structure Command Buffer");
    
    // Create an acceleration structure command encoder
    let acceleration_encoder = command_buffer.new_acceleration_structure_command_encoder();
    acceleration_encoder.set_label("Acceleration Structure Encoder");
    
    // Create an acceleration structure descriptor
    let descriptor = MTLAccelerationStructureDescriptor::new();
    descriptor.set_usage(MTLAccelerationStructureUsage::NONE);
    
    // Get the size requirements for the acceleration structure
    let sizes = device.acceleration_structure_sizes(descriptor);
    println!(
        "Acceleration structure sizes: structure={}, build scratch={}, refit scratch={}",
        sizes.acceleration_structure_size,
        sizes.build_scratch_buffer_size,
        sizes.refit_scratch_buffer_size
    );
    
    // Create an acceleration structure
    let accel_struct = device.new_acceleration_structure(sizes.acceleration_structure_size);
    accel_struct.set_label("Empty Acceleration Structure");
    
    // Create a scratch buffer
    let scratch_buffer = device.new_buffer(
        sizes.build_scratch_buffer_size,
        MTLResourceOptions::StorageModePrivate,
    );
    scratch_buffer.set_label("Build Scratch Buffer");
    
    // Build the acceleration structure
    acceleration_encoder.build_acceleration_structure(
        &accel_struct,
        &descriptor,
        &scratch_buffer,
        0,
    );
    
    // End encoding
    acceleration_encoder.end_encoding();
    
    // Commit the command buffer
    command_buffer.commit();
    
    // Wait for completion
    command_buffer.wait_until_completed();
    
    // Print information about the built acceleration structure
    println!("Acceleration structure built successfully");
    println!("Size: {} bytes", accel_struct.size());
    println!("GPU resource ID: {}", accel_struct.gpu_resource_id());
    
    println!("Example completed successfully");
}