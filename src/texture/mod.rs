mod shared_texture_handle;
mod texture;
mod texture_compression_type;
mod texture_descriptor;
mod texture_swizzle;
mod texture_swizzle_channels;
mod texture_type;
mod texture_usage;
mod texture_view_descriptor;

pub use shared_texture_handle::MTLSharedTextureHandle;
pub use texture::{MTLTexture, TextureExt};
pub use texture_compression_type::MTLTextureCompressionType;
pub use texture_descriptor::MTLTextureDescriptor;
pub use texture_swizzle::MTLTextureSwizzle;
pub use texture_swizzle_channels::MTLTextureSwizzleChannels;
pub use texture_type::MTLTextureType;
pub use texture_usage::MTLTextureUsage;
pub use texture_view_descriptor::MTLTextureViewDescriptor;

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn texture_enum_values_match_the_xcode_27_header() {
        assert_eq!(MTLTextureType::Type1D as u64, 0);
        assert_eq!(MTLTextureType::Type1DArray as u64, 1);
        assert_eq!(MTLTextureType::Type2D as u64, 2);
        assert_eq!(MTLTextureType::Type2DArray as u64, 3);
        assert_eq!(MTLTextureType::Type2DMultisample as u64, 4);
        assert_eq!(MTLTextureType::Cube as u64, 5);
        assert_eq!(MTLTextureType::CubeArray as u64, 6);
        assert_eq!(MTLTextureType::Type3D as u64, 7);
        assert_eq!(MTLTextureType::Type2DMultisampleArray as u64, 8);
        assert_eq!(MTLTextureType::TextureBuffer as u64, 9);
        assert_eq!(MTLTextureSwizzle::Zero as u8, 0);
        assert_eq!(MTLTextureSwizzle::One as u8, 1);
        assert_eq!(MTLTextureSwizzle::Red as u8, 2);
        assert_eq!(MTLTextureSwizzle::Green as u8, 3);
        assert_eq!(MTLTextureSwizzle::Blue as u8, 4);
        assert_eq!(MTLTextureSwizzle::Alpha as u8, 5);
        assert_eq!(MTLTextureCompressionType::Lossless as i64, 0);
        assert_eq!(MTLTextureCompressionType::Lossy as i64, 1);
    }

    #[test]
    fn texture_usage_bits_match_the_xcode_27_header() {
        assert_eq!(MTLTextureUsage::UNKNOWN.bits(), 0x0000);
        assert_eq!(MTLTextureUsage::SHADER_READ.bits(), 0x0001);
        assert_eq!(MTLTextureUsage::SHADER_WRITE.bits(), 0x0002);
        assert_eq!(MTLTextureUsage::RENDER_TARGET.bits(), 0x0004);
        assert_eq!(MTLTextureUsage::PIXEL_FORMAT_VIEW.bits(), 0x0010);
        assert_eq!(MTLTextureUsage::SHADER_ATOMIC.bits(), 0x0020);
    }

    #[test]
    fn swizzle_channels_match_the_metal_abi_and_helpers() {
        const SWIZZLE: MTLTextureSwizzleChannels = MTLTextureSwizzleChannels::new(
            MTLTextureSwizzle::Blue,
            MTLTextureSwizzle::Green,
            MTLTextureSwizzle::Red,
            MTLTextureSwizzle::One,
        );

        assert_eq!(size_of::<MTLTextureSwizzleChannels>(), 4);
        assert_eq!(align_of::<MTLTextureSwizzleChannels>(), 1);
        assert_eq!(SWIZZLE.blue, MTLTextureSwizzle::Red);
        assert_eq!(MTLTextureSwizzleChannels::default(), MTLTextureSwizzleChannels::DEFAULT);
    }
}
