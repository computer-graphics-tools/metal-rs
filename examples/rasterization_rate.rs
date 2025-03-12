//! Example demonstrating the use of MTLRasterizationRate for
//! implementing variable rate rasterization.

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice,
    MTLRasterizationRateMapDescriptor,
    MTLRasterizationRateLayerDescriptor,
    MTLSize
};
use metal_rs::metal::types::MTLCoordinate2D;
use metal_rs::foundation::NSNumber;

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());

    // Set up screen size for the rasterization rate map
    let screen_size = MTLSize {
        width: 1920,
        height: 1080,
        depth: 1
    };

    // Create a layer descriptor for variable rate shading
    // This example creates a higher-resolution center and lower-resolution edges
    let center_layer = MTLRasterizationRateLayerDescriptor::with_sample_count(MTLSize {
        width: 4,  // 4 horizontal samples
        height: 4, // 4 vertical samples
        depth: 1
    });

    // Get sample arrays to configure rates
    let horizontal = center_layer.as_ref().horizontal();
    let vertical = center_layer.as_ref().vertical();

    // Create horizontal pattern (higher detail in center)
    // Rates are multipliers for rasterization density, where:
    // - 1.0 is normal resolution
    // - Values > 1.0 increase resolution (e.g., 2.0 doubles resolution)
    // - Values < 1.0 decrease resolution (e.g., 0.5 halves resolution)
    
    // In this example, we're setting up a pattern where:
    // - Edges are at 0.5x resolution (less detail)
    // - Center is at 1.5x resolution (more detail)
    // For each horizontal/vertical sample point
    
    // Create a Number for each rate
    let number_factory = |val: f32| {
        NSNumber::from_float(val)
    };
    
    // Configure horizontal rates (left to right)
    // Less detail on edges, more in center
    horizontal.as_ref().set_object(&number_factory(0.5), 0); // Left edge
    horizontal.as_ref().set_object(&number_factory(1.0), 1); // Left-center
    horizontal.as_ref().set_object(&number_factory(1.5), 2); // Right-center
    horizontal.as_ref().set_object(&number_factory(0.5), 3); // Right edge
    
    // Configure vertical rates (top to bottom)
    // Less detail on edges, more in center
    vertical.as_ref().set_object(&number_factory(0.5), 0); // Top edge
    vertical.as_ref().set_object(&number_factory(1.0), 1); // Top-center
    vertical.as_ref().set_object(&number_factory(1.5), 2); // Bottom-center
    vertical.as_ref().set_object(&number_factory(0.5), 3); // Bottom edge

    // Create a rasterization rate map descriptor
    let rate_map_descriptor = MTLRasterizationRateMapDescriptor::with_screen_size_and_layer(
        screen_size,
        &center_layer
    );
    rate_map_descriptor.as_ref().set_label("Variable Rate Rasterization Map");

    // Create the rasterization rate map from the descriptor
    let rate_map = device.new_rasterization_rate_map(&rate_map_descriptor);
    
    // Print information about the rate map
    println!("Created rasterization rate map:");
    println!("  Screen size: {}x{}", 
             rate_map.as_ref().screen_size().width, 
             rate_map.as_ref().screen_size().height);
    println!("  Physical granularity: {}x{}", 
             rate_map.as_ref().physical_granularity().width,
             rate_map.as_ref().physical_granularity().height);
    
    // Get physical size of layer 0
    let physical_size = rate_map.as_ref().physical_size(0);
    println!("  Layer 0 physical size: {}x{}", 
             physical_size.width, 
             physical_size.height);
    
    // Get parameter buffer information
    let param_info = rate_map.as_ref().parameter_buffer_size_and_align();
    println!("  Parameter buffer size: {} bytes", param_info.size);
    println!("  Parameter buffer alignment: {} bytes", param_info.align);

    // Map a screen coordinate to a physical coordinate
    let screen_coord = MTLCoordinate2D { x: 960.0, y: 540.0 }; // Center of screen
    let physical_coord = rate_map.as_ref().map_screen_to_physical_coordinates(screen_coord, 0);
    println!("  Center screen coordinate ({}, {}) maps to physical coordinate ({}, {})",
             screen_coord.x, screen_coord.y, physical_coord.x, physical_coord.y);

    // Note: In a real application, you would:
    // 1. Create a buffer to hold the parameter data
    // 2. Copy the parameters to the buffer using rate_map.copy_parameter_data_to_buffer()
    // 3. Set the buffer in your render pipeline
    // 4. Configure your shaders to use the variable rate rasterization
    
    println!("To use this in a real application, you would:");
    println!("1. Create a Metal buffer to hold parameters");
    println!("2. Copy parameters to the buffer using rate_map.copy_parameter_data_to_buffer()");
    println!("3. Set the rasterization rate map on the render command encoder");
    println!("4. Render with variable rate rasterization");
}