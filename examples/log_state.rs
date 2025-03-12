// Example demonstrating the use of MTLLogState for GPU logging

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice,
    MTLLogStateDescriptor,
    MTLLogLevel,
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    
    println!("Using device: {}", device.name());
    
    // Create a log state descriptor
    let descriptor = MTLLogStateDescriptor::new();
    
    // Configure logging to capture Info level and above
    descriptor.set_level(MTLLogLevel::Info);
    
    // Set a reasonable buffer size for logs
    descriptor.set_buffer_size(1024);
    
    // Create the log state
    let mut log_state = match device.new_log_state(&descriptor) {
        Ok(state) => state,
        Err(err) => {
            println!("Failed to create log state: {:?}", err);
            return;
        }
    };
    
    // Add a log handler to process Metal log messages
    log_state.add_log_handler(|subsystem, category, level, message| {
        println!(
            "[{:?}] {}/{}: {}",
            level,
            subsystem,
            category,
            message
        );
    });
    
    println!("Log state configured. Run some Metal operations to see logs.");
    
    // Create a command queue which will generate some log messages
    let queue = device.new_command_queue();
    
    // Create and commit a command buffer to generate logs
    let command_buffer = queue.new_command_buffer();
    command_buffer.commit();
    command_buffer.wait_until_completed();
    
    println!("Done. Any Metal logs should have been printed above.");
}