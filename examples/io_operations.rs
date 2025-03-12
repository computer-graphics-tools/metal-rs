// Example demonstrating Metal I/O operations

use metal_rs::{
    foundation::NSURL,
    metal::{
        MTLCreateSystemDefaultDevice,
        MTLIOCommandQueueDescriptor,
        MTLIOCommandQueueType,
        MTLIOPriority,
        MTLIOStatus,
        MTLIOCompressionMethod,
        MTLIOCompressionContext,
    },
};
use std::fs::File;
use std::io::Write;

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    
    println!("Using device: {}", device.name());
    
    // Create a temporary file for I/O operations
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join("metal_io_example_input.dat");
    let output_path = temp_dir.join("metal_io_example_output.dat");
    let compressed_path = temp_dir.join("metal_io_example_compressed.dat");
    
    println!("Creating temporary files:");
    println!("  Input: {:?}", input_path);
    println!("  Output: {:?}", output_path);
    println!("  Compressed: {:?}", compressed_path);
    
    // Create sample data
    let data_size = 1024 * 1024; // 1MB
    let mut data = vec![0u8; data_size];
    for i in 0..data_size {
        data[i] = (i % 256) as u8;
    }
    
    // Write data to input file
    let mut file = File::create(&input_path).expect("Failed to create input file");
    file.write_all(&data).expect("Failed to write data to input file");
    
    // Create I/O command queue
    let queue_descriptor = MTLIOCommandQueueDescriptor::new();
    queue_descriptor.set_max_command_buffer_count(10);
    queue_descriptor.set_priority(MTLIOPriority::Normal);
    queue_descriptor.set_queue_type(MTLIOCommandQueueType::Serial);
    
    let queue = device.new_io_command_queue(&queue_descriptor);
    queue.set_label("Example I/O Queue");
    
    // Create file handles
    let input_url = NSURL::file_url_with_path(&input_path.to_string_lossy(), false);
    let output_url = NSURL::file_url_with_path(&output_path.to_string_lossy(), false);
    
    let input_handle = match device.new_io_file_handle(&input_url) {
        Some(handle) => {
            println!("Successfully created input file handle");
            handle
        },
        None => {
            println!("Failed to create input file handle");
            return;
        }
    };
    
    // Create a buffer to receive the data
    let buffer = device.new_buffer(data_size, 0);
    
    // Create a command buffer and load the data
    let command_buffer = queue.command_buffer();
    command_buffer.set_label("Load Data Command Buffer");
    
    // Load data from file to buffer
    command_buffer.load_buffer(&buffer, 0, &input_handle, 0, data_size);
    
    // Set up completion handler
    command_buffer.add_completed_handler(|status| {
        println!("Command buffer completed with status: {:?}", status);
    });
    
    // Execute command buffer
    println!("Committing command buffer to load data...");
    command_buffer.commit();
    command_buffer.wait_until_completed();
    
    // Check status
    println!("Command buffer status: {:?}", command_buffer.status());
    if let Some(error) = command_buffer.error() {
        println!("Error: {:?}: {}", error.0, error.1);
    }
    
    // Demonstrate compression
    println!("Demonstrating compression...");
    match MTLIOCompressionContext::new(
        &compressed_path.to_string_lossy(),
        MTLIOCompressionMethod::LZFSE,
        0
    ) {
        Some(context) => {
            println!("Created compression context");
            context.append_data(&data[0..data_size / 2]);
            println!("Appended first half of data");
            context.append_data(&data[data_size / 2..]);
            println!("Appended second half of data");
            
            let status = context.flush_and_destroy();
            println!("Compression completed with status: {:?}", status);
            
            if let Ok(metadata) = std::fs::metadata(&compressed_path) {
                println!("Compressed file size: {} bytes (compression ratio: {:.2})", 
                    metadata.len(), 
                    data_size as f64 / metadata.len() as f64);
            }
        },
        None => {
            println!("Failed to create compression context");
        }
    }
    
    println!("I/O operations example completed.");
    
    // Clean up temporary files
    let _ = std::fs::remove_file(&input_path);
    let _ = std::fs::remove_file(&output_path);
    let _ = std::fs::remove_file(&compressed_path);
}