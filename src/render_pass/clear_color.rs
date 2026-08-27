use objc2::{Encode, Encoding, RefEncode};

/// Clear color used for render pass color attachments.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl MTLClearColor {
    /// Creates a clear color with the given red, green, blue, and alpha components.
    ///
    /// This is the Rust equivalent of Metal's `MTLClearColorMake` inline helper.
    pub const fn new(
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

unsafe impl Encode for MTLClearColor {
    const ENCODING: Encoding = Encoding::Struct("?", &[f64::ENCODING, f64::ENCODING, f64::ENCODING, f64::ENCODING]);
}

unsafe impl RefEncode for MTLClearColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
