//! Example demonstrating the implementation of MTLBlitPassDescriptor.
//! 
//! This example shows how MTLBlitPassDescriptor and associated types 
//! can be created and configured to prepare for blit operations.
//! Note: Due to limitations with the example harness, we can't directly
//! demonstrate the new_blit_command_encoder_with_descriptor method in this example.

use metal_rs::metal::{
    MTLBlitPassDescriptor, MTLCreateSystemDefaultDevice
};

fn main() {
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a blit pass descriptor
    println!("Creating MTLBlitPassDescriptor...");
    let blit_pass_descriptor = MTLBlitPassDescriptor::new();
    
    // Demonstrate accessing and modifying sample buffer attachments
    println!("Configuring sample buffer attachments...");
    let attachments = blit_pass_descriptor.sample_buffer_attachments();
    
    // Configure the first attachment (index 0)
    let attachment0 = attachments.object(0);
    attachment0.set_start_of_encoder_sample_index(10);
    attachment0.set_end_of_encoder_sample_index(20);
    
    // Configure the second attachment (index 1)
    let attachment1 = attachments.object(1);
    attachment1.set_start_of_encoder_sample_index(30);
    attachment1.set_end_of_encoder_sample_index(40);
    
    // Demonstrate reading values from attachments
    println!("\nAttachment configuration values:");
    println!("Attachment 0 start sample index: {}", attachment0.start_of_encoder_sample_index());
    println!("Attachment 0 end sample index: {}", attachment0.end_of_encoder_sample_index());
    println!("Attachment 1 start sample index: {}", attachment1.start_of_encoder_sample_index());
    println!("Attachment 1 end sample index: {}", attachment1.end_of_encoder_sample_index());
    
    println!("\nMTLBlitPassDescriptor successfully created and configured.");
    println!("In real usage, you would pass this descriptor to:");
    println!("  command_buffer.new_blit_command_encoder_with_descriptor(&blit_pass_descriptor)");
}