use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLResourceOptions};
use std::mem::size_of;
use foreign_types::ForeignType;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Get the default device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create a command queue
    let command_queue = device.new_command_queue();
    
    // Create source buffer with data
    let data: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    let data_size = size_of::<[f32; 4]>();
    let source_buffer = device.new_buffer_with_data(
        data.as_ptr() as *const _, 
        data_size,
        MTLResourceOptions::StorageModeSharedCpuCacheModeWriteCombined
    );
    
    // Create destination buffer
    let dest_buffer = device.new_buffer(
        data_size,
        MTLResourceOptions::StorageModeSharedCpuCacheModeWriteCombined
    );
    
    // Create command buffer
    let command_buffer = command_queue.new_command_buffer();
    // For now, let's skip setting the label to avoid Objective-C interaction complications
    
    // Create blit command encoder
    // For now, let's use the one we know exists from the Metal docs
let blit_encoder = unsafe {
    use objc::{msg_send, sel, sel_impl};
    use objc::runtime::Object;
    use metal_rs::metal::MTLBlitCommandEncoder;
    
    let encoder_ptr: *mut Object = msg_send![command_buffer.as_ptr(), blitCommandEncoder];
    MTLBlitCommandEncoder::from_ptr(encoder_ptr)
};
    // For now, let's skip setting the label to avoid Objective-C interaction complications
    
    // Copy from source to destination
    blit_encoder.copy_from_buffer(
        &source_buffer,
        0,  // source offset
        &dest_buffer,
        0,  // destination offset
        data_size as u64
    );
    
    // End encoding
    blit_encoder.end_encoding();
    
    // Commit command buffer and wait for completion
    command_buffer.commit();
    // Since we can't use waitUntilCompleted directly, let's sleep briefly
    sleep(Duration::from_millis(50));
    
    // Verify the copy worked by reading back the destination buffer
    let dest_ptr = dest_buffer.contents() as *const f32;
    let mut result = [0.0f32; 4];
    unsafe {
        for i in 0..4 {
            result[i] = *dest_ptr.add(i);
        }
    }
    
    println!("Source buffer: {:?}", data);
    println!("Destination buffer after copy: {:?}", result);
    
    assert_eq!(data, result, "Buffer copy failed!");
    println!("Buffer copy successful!");
}