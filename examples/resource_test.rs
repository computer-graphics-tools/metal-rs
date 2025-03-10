// Example demonstrating the MTLResource functionality
// This example shows how to use the MTLResource protocol with buffers and textures

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice, MTLResourceOptions, MTLPurgeableState,
    MTLStorageMode, MTLCPUCacheMode, MTLHazardTrackingMode, MTLTextureDescriptor,
    MTLPixelFormat, MTLResourceRef
};

fn main() {
    // Get the default device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a buffer (which implements MTLResource)
    let buffer = device.new_buffer(1024, MTLResourceOptions::CPUCacheModeDefaultCache);
    buffer.set_label("Test Buffer");
    
    // Use MTLResource methods to query properties
    println!("\nBuffer Resource Properties:");
    println!("------------------------");
    println!("Label: {:?}", buffer.label());
    println!("CPU Cache Mode: {:?}", buffer.cpu_cache_mode());
    println!("Storage Mode: {:?}", buffer.storage_mode());
    println!("Hazard Tracking Mode: {:?}", buffer.hazard_tracking_mode());
    println!("Resource Options: {:?}", buffer.resource_options());
    println!("Allocated Size: {} bytes", buffer.allocated_size());
    println!("Is Aliasable: {}", buffer.is_aliasable());
    
    // Set buffer to be purgeable
    let old_state = buffer.set_purgeable_state(MTLPurgeableState::Volatile);
    println!("Changed purgeable state from {:?} to Volatile", old_state);
    
    // Create a texture (which also implements MTLResource)
    let texture_descriptor = MTLTextureDescriptor::new();
    texture_descriptor.set_width(512);
    texture_descriptor.set_height(512);
    texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
    texture_descriptor.set_storage_mode(MTLStorageMode::Private);
    
    let texture = device.new_texture(&texture_descriptor);
    texture.set_label("Test Texture");
    
    // Use MTLResource methods to query properties
    println!("\nTexture Resource Properties:");
    println!("---------------------------");
    println!("Label: {:?}", texture.label());
    println!("CPU Cache Mode: {:?}", texture.cpu_cache_mode());
    println!("Storage Mode: {:?}", texture.storage_mode());
    println!("Hazard Tracking Mode: {:?}", texture.hazard_tracking_mode());
    println!("Resource Options: {:?}", texture.resource_options());
    println!("Allocated Size: {} bytes", texture.allocated_size());
    println!("Is Aliasable: {}", texture.is_aliasable());
    
    // Demonstrate polymorphic use of resources
    println!("\nPolymorphic Resource Access:");
    println!("---------------------------");
    use_resource(&buffer);
    use_resource(&texture);
}

// Function that works with any MTLResource
fn use_resource<T: AsRef<MTLResourceRef>>(resource: &T) {
    let resource_ref = resource.as_ref();
    println!("Resource with label {:?} has storage mode {:?}", 
        resource_ref.label(), 
        resource_ref.storage_mode());
}