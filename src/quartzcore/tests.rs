#[cfg(test)]
mod tests {
    use crate::quartzcore::{CAMetalLayer, CGSize};
    use crate::quartzcore::metal_layer::MTLPixelFormat;
    use foreign_types::ForeignType;

    #[test]
    fn test_create_metal_layer() {
        let layer = CAMetalLayer::new();
        assert!(!layer.as_ptr().is_null());
        
        // Set and get framebuffer_only
        layer.set_framebuffer_only(true);
        assert_eq!(layer.framebuffer_only(), true);
        
        // Set and get drawable size
        let size = CGSize::new(800.0, 600.0);
        layer.set_drawable_size(size);
        let retrieved_size = layer.drawable_size();
        assert_eq!(retrieved_size.width, 800.0);
        assert_eq!(retrieved_size.height, 600.0);
        
        // Set and get pixel format
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        assert_eq!(layer.pixel_format(), MTLPixelFormat::BGRA8Unorm);
    }
    
    // Note: This test requires a Metal device and might not work in all environments
    #[test]
    #[ignore]
    fn test_next_drawable() {
        let layer = CAMetalLayer::new();
        
        // Set drawable size
        layer.set_drawable_size(CGSize::new(800.0, 600.0));
        
        // In a real environment with a proper Metal device setup, this would return a drawable
        let drawable = layer.next_drawable();
        
        // Since we haven't set up a proper device, this will likely be None, so we don't assert
        println!("Drawable available: {}", drawable.is_some());
    }
}