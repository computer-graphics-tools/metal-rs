use metal_rs::metal::{
    MTLCreateSystemDefaultDevice, 
    MTLSamplerDescriptor, 
    MTLSamplerMinMagFilter, 
    MTLSamplerMipFilter,
    MTLSamplerAddressMode
};

fn main() {
    // Get the default device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a sampler descriptor
    let descriptor = MTLSamplerDescriptor::new();
    
    // Configure sampler settings
    descriptor.set_min_filter(MTLSamplerMinMagFilter::Linear);
    descriptor.set_mag_filter(MTLSamplerMinMagFilter::Linear);
    descriptor.set_mip_filter(MTLSamplerMipFilter::Linear);
    
    descriptor.set_s_address_mode(MTLSamplerAddressMode::ClampToEdge);
    descriptor.set_t_address_mode(MTLSamplerAddressMode::ClampToEdge);
    descriptor.set_r_address_mode(MTLSamplerAddressMode::ClampToEdge);
    
    descriptor.set_normalized_coordinates(true);
    descriptor.set_lod_min_clamp(0.0);
    descriptor.set_lod_max_clamp(8.0);
    descriptor.set_max_anisotropy(16);
    
    descriptor.set_label("Test Sampler");
    
    println!("Sampler descriptor configured:");
    println!("  - Min filter: {:?}", descriptor.min_filter());
    println!("  - Mag filter: {:?}", descriptor.mag_filter());
    println!("  - Mip filter: {:?}", descriptor.mip_filter());
    println!("  - S address mode: {:?}", descriptor.s_address_mode());
    println!("  - T address mode: {:?}", descriptor.t_address_mode());
    println!("  - R address mode: {:?}", descriptor.r_address_mode());
    println!("  - Normalized coordinates: {}", descriptor.normalized_coordinates());
    println!("  - LOD min clamp: {}", descriptor.lod_min_clamp());
    println!("  - LOD max clamp: {}", descriptor.lod_max_clamp());
    println!("  - Max anisotropy: {}", descriptor.max_anisotropy());
    println!("  - Label: {}", descriptor.label());
    
    // Create a sampler state using the descriptor
    let sampler_state = device.new_sampler_state(&descriptor);
    println!("Created sampler state with label: {}", sampler_state.label());
    
    // Verify the device - we don't use device_ref directly but it demonstrates that we can access it
    let _device_ref = sampler_state.device();
    println!("Sampler created on device: {}", device.name());
    
    println!("Sampler test completed successfully!");
}