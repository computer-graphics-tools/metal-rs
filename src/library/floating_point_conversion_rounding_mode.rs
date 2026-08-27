use objc2::{Encode, Encoding, RefEncode};

/// The rounding mode for narrowing floating-point conversions.
///
/// Availability: macOS 27.0+, iOS 27.0+
#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLFloatingPointConversionRoundingMode {
    ToNearestEven = 0,
    TowardZero = 1,
}

unsafe impl Encode for MTLFloatingPointConversionRoundingMode {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLFloatingPointConversionRoundingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use super::MTLFloatingPointConversionRoundingMode;

    #[test]
    fn rounding_mode_values_match_xcode_27_header() {
        assert_eq!(MTLFloatingPointConversionRoundingMode::ToNearestEven as isize, 0);
        assert_eq!(MTLFloatingPointConversionRoundingMode::TowardZero as isize, 1);
    }
}
