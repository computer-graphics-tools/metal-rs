use objc2::{Encode, Encoding, RefEncode};

/// Curve basis (from `MTLCurveBasis`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveBasis {
    /// B-Spline basis.
    BSpline = 0,
    /// Catmull-Rom basis.
    CatmullRom = 1,
    /// Linear basis with 2 control points.
    Linear = 2,
    /// Bézier basis.
    Bezier = 3,
}

unsafe impl Encode for MTLCurveBasis {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveBasis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
