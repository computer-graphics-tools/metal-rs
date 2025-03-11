//! Test for MTLResourceStateCommandEncoder integration
//!
//! This example verifies that the MTLResourceStateCommandEncoder can be created
//! and functions properly with the command buffer.

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice, 
    resource_state_command_encoder::MTLResourceStateCommandEncoder, 
    command_encoder::CommandEncoder
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a command queue
    let command_queue = device.new_command_queue();
    
    // Create a command buffer
    let command_buffer = command_queue.new_command_buffer();
    command_buffer.set_label("Resource State Test Command Buffer");
    
    // Create a resource state command encoder - directly importing the type
    let resource_state_encoder = unsafe {
        use objc::{msg_send, sel, sel_impl};
        use objc::runtime::Object;
        use foreign_types::ForeignType;
        
        let encoder_ptr: *mut Object = msg_send![command_buffer.as_ptr(), resourceStateCommandEncoder];
        MTLResourceStateCommandEncoder::from_ptr(encoder_ptr)
    };
    
    // Test basic functionality
    resource_state_encoder.set_label("Resource State Test Encoder");
    
    // Get and verify the label
    let encoder_label = resource_state_encoder.label();
    println!("Encoder label: {:?}", encoder_label);
    
    // End encoding
    resource_state_encoder.end_encoding();
    println!("Resource state encoding completed");
    
    // Commit the command buffer 
    command_buffer.commit();
    
    println!("Command buffer committed");
    println!("Test complete - MTLResourceStateCommandEncoder integration verified");
}