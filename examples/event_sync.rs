// This example demonstrates how to use MTLEvent for GPU timeline synchronization
use metal_rs::metal::{MTLCreateSystemDefaultDevice, CommandEncoder};

fn main() {
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Create an event for GPU timeline synchronization
    let event = device.new_event();
    event.set_label("Sync Event");
    println!("Created event: {:?}", event);
    
    // Create a shared event for cross-process synchronization
    let shared_event = device.new_shared_event();
    shared_event.set_label("Shared Sync Event");
    println!("Created shared event: {:?}", shared_event);
    
    // Set and get the signaled value on the shared event
    shared_event.set_signaled_value(42);
    let current_value = shared_event.signaled_value();
    println!("Set shared event value to 42, current value is: {}", current_value);
    
    // Create a shared event handle for inter-process communication
    let shared_event_handle = shared_event.new_shared_event_handle();
    println!("Created shared event handle");
    
    // Create a new shared event from the handle
    let shared_event_from_handle = device.new_shared_event_from_handle(&shared_event_handle);
    shared_event_from_handle.set_label("Event from handle");
    println!("Created shared event from handle: {:?}", shared_event_from_handle);
    
    // Verify the value is preserved
    let handle_value = shared_event_from_handle.signaled_value();
    println!("Value from handle-created event: {}", handle_value);
    
    println!("Example completed successfully!");
}