use objc2::{Encode, Encoding, RefEncode};

/// Matrix layout (from `MTLMatrixLayout`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMatrixLayout {
    /// Column-major order.
    ColumnMajor = 0,
    /// Row-major order.
    RowMajor = 1,
}

unsafe impl Encode for MTLMatrixLayout {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLMatrixLayout {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
