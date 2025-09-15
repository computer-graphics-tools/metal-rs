use objc2::{Encode, Encoding, RefEncode};

/// Signature defining what data is provided to an intersection function (from `MTLIntersectionFunctionSignature`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIntersectionFunctionSignature(pub u64);
bitflags::bitflags! {
    impl MTLIntersectionFunctionSignature: u64 {
        const None = 0;
        const Instancing = 1<<0;
        const TriangleData = 1<<1;
        const WorldSpaceData = 1<<2;
        const InstanceMotion = 1<<3;
        const PrimitiveMotion = 1<<4;
        const ExtendedLimits = 1<<5;
        const MaxLevels = 1<<6;
        const CurveData = 1<<7;
        const IntersectionFunctionBuffer = 1<<8;
        const UserData = 1<<9;
    }
}

unsafe impl Encode for MTLIntersectionFunctionSignature {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLIntersectionFunctionSignature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
