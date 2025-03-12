//! Example demonstrating the use of MTLResidencySet for managing
//! memory residency of resources.

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice,
    MTLResidencySetDescriptor,
    MTLResourceOptions
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());

    // Create some resources that implement the MTLAllocation protocol
    // Note: MTLBuffer, MTLTexture, and other resource types implement MTLAllocation
    let buffer1 = device.new_buffer(1024 * 1024, MTLResourceOptions::CPUCacheModeDefaultCache);
    let buffer2 = device.new_buffer(2 * 1024 * 1024, MTLResourceOptions::StorageModePrivate);
    let buffer3 = device.new_buffer(4 * 1024 * 1024, MTLResourceOptions::StorageModeManaged);

    // Create a residency set descriptor
    let residency_descriptor = MTLResidencySetDescriptor::new();
    residency_descriptor.as_ref().set_label("Example Residency Set");
    residency_descriptor.as_ref().set_initial_capacity(10); // Set initial capacity for allocations
    
    // Create a residency set with the descriptor
    let residency_set = device.new_residency_set(&residency_descriptor);
    
    // NOTE: The following code is conceptual and will not compile correctly.
    // In a real implementation, we would need to properly cast MTLBuffer to MTLAllocation
    // or have the Metal API properly expose this functionality.
    
    // For illustration purposes only:
    println!("Created a residency set with descriptor");
    println!("Initial capacity: {}", residency_descriptor.as_ref().initial_capacity());
    println!("Conceptually, we would now:");
    println!("1. Add buffers to the residency set");
    println!("2. Request residency to make sure they're in memory");
    println!("3. Perform operations using the buffers");
    println!("4. End residency when done");
    
    // In actual implementation with proper casting:
    // residency_set.as_ref().add_allocation(buffer1_as_allocation);
    // residency_set.as_ref().request_residency();
    // residency_set.as_ref().end_residency();
    // residency_set.as_ref().remove_all_allocations();

    // Note: Residency sets are useful for managing groups of resources that
    // need to be resident together, especially for applications with
    // explicit memory management.
}