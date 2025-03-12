// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::{
    MTLCreateSystemDefaultDevice, MTLVisibleFunctionTableDescriptor, 
    MTLVisibleFunctionTable
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Metal device: {}", device.name());
    
    // Create a visible function table descriptor
    let descriptor = MTLVisibleFunctionTableDescriptor::new();
    descriptor.set_function_count(4);
    
    println!("Created visible function table descriptor with function count: {}", 
             descriptor.function_count());
    
    // Create a visible function table
    let function_table = device.new_visible_function_table(&descriptor);
    println!("Created visible function table with GPU resource ID: {}", 
             function_table.gpu_resource_id());
    
    // In a real application, you would:
    // 1. Create shader functions with device.new_library_with_source()
    // 2. Get function handles for those functions
    // 3. Set functions in the table using:
    //    function_table.set_function(Some(&function_handle), index);
    
    println!("Visible function tables are typically used with ray tracing or other advanced Metal features.");
    println!("They allow shaders to dynamically call different functions based on runtime conditions.");
    
    println!("Example completed successfully");
}