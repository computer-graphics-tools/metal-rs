//! Example demonstrating the use of MTLBlitPassDescriptor.
//!
//! This example shows how to create and configure a blit pass descriptor.

use metal_rs::metal::{
    MTLBlitPassDescriptor, MTLCreateSystemDefaultDevice
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a blit pass descriptor
    let blit_pass_descriptor = MTLBlitPassDescriptor::new();
    
    // Configure sample buffer attachments
    let sample_buffer_attachments = blit_pass_descriptor.sample_buffer_attachments();
    let attachment = sample_buffer_attachments.object(0);
    attachment.set_start_of_encoder_sample_index(0);
    attachment.set_end_of_encoder_sample_index(100);
    
    println!("Blit Pass Sample Buffer Attachment configured:");
    println!("  Start sample index: {}", attachment.start_of_encoder_sample_index());
    println!("  End sample index: {}", attachment.end_of_encoder_sample_index());
    
    println!("Successfully created and configured MTLBlitPassDescriptor");
}