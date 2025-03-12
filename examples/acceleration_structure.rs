// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::{
    MTLCreateSystemDefaultDevice, MTLAccelerationStructureDescriptor, 
    MTLAccelerationStructureUsage
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Metal device: {}", device.name());
    
    // Create an acceleration structure descriptor
    let descriptor = MTLAccelerationStructureDescriptor::new();
    descriptor.set_usage(MTLAccelerationStructureUsage::PREFER_FAST_BUILD | MTLAccelerationStructureUsage::REFIT);
    
    println!("Created acceleration structure descriptor with usage: {:?}", descriptor.usage());
    
    // Get the size requirements for the acceleration structure
    let sizes = device.acceleration_structure_sizes(&descriptor);
    println!("Acceleration structure size: {} bytes", sizes.acceleration_structure_size);
    println!("Build scratch buffer size: {} bytes", sizes.build_scratch_buffer_size);
    println!("Refit scratch buffer size: {} bytes", sizes.refit_scratch_buffer_size);
    
    // Create an acceleration structure
    let accel_struct = device.new_acceleration_structure(sizes.acceleration_structure_size);
    println!("Created acceleration structure with size: {} bytes", accel_struct.size());
    println!("GPU resource ID: {}", accel_struct.gpu_resource_id());
    
    // Calculate heap requirements
    let (heap_size, heap_align) = device.heap_acceleration_structure_size_and_align(sizes.acceleration_structure_size);
    println!("Heap size: {} bytes, alignment: {} bytes", heap_size, heap_align);
    
    // In a real application, you would:
    // 1. Create a command buffer and acceleration structure command encoder
    // 2. Create geometry descriptors (triangle, bounding box, etc.)
    // 3. Build the acceleration structure
    // 4. Use it for ray tracing
    
    println!("Acceleration structures are used with ray tracing in Metal.");
    println!("They provide spatial data structures to accelerate ray-geometry intersection tests.");
    
    println!("Example completed successfully");
}