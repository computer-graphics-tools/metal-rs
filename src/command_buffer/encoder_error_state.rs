use objc2::{Encode, Encoding, RefEncode};

/// The execution state of a Metal command encoder after command-buffer execution.
///
/// Availability: macOS 11.0+, iOS 14.0+
#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCommandEncoderErrorState {
    Unknown = 0,
    Completed = 1,
    Affected = 2,
    Pending = 3,
    Faulted = 4,
}

unsafe impl Encode for MTLCommandEncoderErrorState {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLCommandEncoderErrorState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use super::MTLCommandEncoderErrorState;

    #[test]
    fn error_states_match_metal_header_values() {
        assert_eq!(MTLCommandEncoderErrorState::Unknown as isize, 0);
        assert_eq!(MTLCommandEncoderErrorState::Completed as isize, 1);
        assert_eq!(MTLCommandEncoderErrorState::Affected as isize, 2);
        assert_eq!(MTLCommandEncoderErrorState::Pending as isize, 3);
        assert_eq!(MTLCommandEncoderErrorState::Faulted as isize, 4);
    }
}
