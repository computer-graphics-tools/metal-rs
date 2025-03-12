// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::{
    MTLCreateSystemDefaultDevice, MTLIntersectionFunctionTableDescriptor, 
    MTLIntersectionFunctionSignature
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Metal device: {}", device.name());
    
    // Create an intersection function table descriptor
    let descriptor = MTLIntersectionFunctionTableDescriptor::new();
    descriptor.set_function_count(16);
    
    println!("Created intersection function table descriptor with function count: {}", 
             descriptor.function_count());
    
    // Create an intersection function table
    let intersection_table = device.new_intersection_function_table(&descriptor);
    println!("Created intersection function table with GPU resource ID: {}", 
             intersection_table.gpu_resource_id());
    
    // Set opaque triangle intersection functions with different signatures
    let sig = MTLIntersectionFunctionSignature::TRIANGLE_DATA | MTLIntersectionFunctionSignature::WORLD_SPACE_DATA;
    intersection_table.set_opaque_triangle_intersection_function(sig, 0);
    
    println!("Set opaque triangle intersection functions with signature: {:?}", sig);
    
    // In a real application, you would:
    // 1. Create shader functions with device.new_library_with_source()
    // 2. Get function handles for intersection functions
    // 3. Set custom functions in the table using:
    //    intersection_table.set_function(Some(&function_handle), index);
    
    println!("Intersection function tables are used with ray tracing features in Metal.");
    println!("They allow ray-geometry intersection tests to be specialized per object.");
    
    println!("Example completed successfully");
}