// This example demonstrates the MTLComputePassDescriptor 

use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLDispatchType};

fn main() {
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a compute pass descriptor
    let compute_pass_descriptor = metal_rs::metal::MTLComputePassDescriptor::new();
    println!("Created compute pass descriptor");
    
    // Set the dispatch type to concurrent
    compute_pass_descriptor.set_dispatch_type(MTLDispatchType::Concurrent);
    println!("Set dispatch type to Concurrent");
    
    // Get the dispatch type
    let dispatch_type = compute_pass_descriptor.dispatch_type();
    println!("Current dispatch type: {:?}", dispatch_type);
    
    // Access the sample buffer attachments
    let sample_buffer_attachments = compute_pass_descriptor.sample_buffer_attachments();
    println!("Obtained sample buffer attachments");
    
    // Get a specific attachment
    let sample_attachment = sample_buffer_attachments.object(0);
    println!("Accessed sample attachment at index 0");
    
    // Configure the sample attachment
    sample_attachment.set_start_of_encoder_sample_index(0);
    sample_attachment.set_end_of_encoder_sample_index(1);
    println!("Configured sample attachment indexes");
    
    // Read back values
    println!(
        "Sample attachment indexes - start: {}, end: {}", 
        sample_attachment.start_of_encoder_sample_index(),
        sample_attachment.end_of_encoder_sample_index()
    );
    
    println!("Example completed successfully!");
}