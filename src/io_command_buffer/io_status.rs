use objc2::{Encode, Encoding, RefEncode};

/// Status of an IO command buffer (ported from `MTLIOStatus`).
///
/// Availability: macOS 13.0+, iOS 16.0+
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLIOStatus {
    Pending = 0,
    Cancelled = 1,
    Error = 2,
    Complete = 3,
}

unsafe impl Encode for MTLIOStatus {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLIOStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::MTLIOStatus;

    #[test]
    fn io_status_matches_metal_header_values_and_layout() {
        assert_eq!(MTLIOStatus::Pending as i64, 0);
        assert_eq!(MTLIOStatus::Cancelled as i64, 1);
        assert_eq!(MTLIOStatus::Error as i64, 2);
        assert_eq!(MTLIOStatus::Complete as i64, 3);
        assert_eq!(size_of::<MTLIOStatus>(), size_of::<i64>());
        assert_eq!(align_of::<MTLIOStatus>(), align_of::<i64>());
    }
}
