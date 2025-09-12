use objc2::{Encode, Encoding, RefEncode};

use crate::texture::TextureSwizzle;

/// Channel swizzle to use when reading or sampling from the texture (from `MTLTextureSwizzleChannels`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureSwizzleChannels {
    pub red: TextureSwizzle,
    pub green: TextureSwizzle,
    pub blue: TextureSwizzle,
    pub alpha: TextureSwizzle,
}

unsafe impl Encode for TextureSwizzleChannels {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLTextureSwizzleChannels=CCCC}",
        &[
            TextureSwizzle::ENCODING,
            TextureSwizzle::ENCODING,
            TextureSwizzle::ENCODING,
            TextureSwizzle::ENCODING,
        ],
    );
}

unsafe impl RefEncode for TextureSwizzleChannels {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl TextureSwizzleChannels {
    pub fn new(red: TextureSwizzle, green: TextureSwizzle, blue: TextureSwizzle, alpha: TextureSwizzle) -> Self {
        Self { red, green, blue, alpha }
    }
}

impl Default for TextureSwizzleChannels {
    fn default() -> Self {
        Self {
            red: TextureSwizzle::Red,
            green: TextureSwizzle::Green,
            blue: TextureSwizzle::Blue,
            alpha: TextureSwizzle::Alpha,
        }
    }
}


