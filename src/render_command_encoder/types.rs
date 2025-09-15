use objc2::{Encode, Encoding, RefEncode};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

unsafe impl Encode for PrimitiveType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for PrimitiveType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

unsafe impl Encode for VisibilityResultMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for VisibilityResultMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScissorRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

unsafe impl Encode for ScissorRect {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLScissorRect=QQQQ}",
        &[
            usize::ENCODING,
            usize::ENCODING,
            usize::ENCODING,
            usize::ENCODING,
        ],
    );
}

unsafe impl RefEncode for ScissorRect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub origin_x: f64,
    pub origin_y: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

unsafe impl Encode for Viewport {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLViewport=dddddd}",
        &[
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
        ],
    );
}

unsafe impl RefEncode for Viewport {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

unsafe impl Encode for CullMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for CullMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Winding {
    Clockwise = 0,
    CounterClockwise = 1,
}

unsafe impl Encode for Winding {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for Winding {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DepthClipMode {
    Clip = 0,
    Clamp = 1,
}

unsafe impl Encode for DepthClipMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for DepthClipMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TriangleFillMode {
    Fill = 0,
    Lines = 1,
}

unsafe impl Encode for TriangleFillMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for TriangleFillMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexAmplificationViewMapping {
    pub viewport_array_index_offset: u32,
    pub render_target_array_index_offset: u32,
}

unsafe impl Encode for VertexAmplificationViewMapping {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLVertexAmplificationViewMapping=II}",
        &[u32::ENCODING, u32::ENCODING],
    );
}

unsafe impl RefEncode for VertexAmplificationViewMapping {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
