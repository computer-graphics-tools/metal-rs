//! Common types and constants used in Metal API.
//!
//! This module contains various common structures and enumerations used throughout 
//! the Metal API, such as pixel formats, clear colors, and other basic types.

/// MTLClearColor - Represents a color used in clearing render targets.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl MTLClearColor {
    /// Creates a new clear color with the given RGBA components.
    #[must_use]
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        MTLClearColor { red, green, blue, alpha }
    }
}

impl Default for MTLClearColor {
    fn default() -> Self {
        MTLClearColor::new(0.0, 0.0, 0.0, 1.0)
    }
}

/// MTLPixelFormat - Enum representing the pixel formats for textures and render targets.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPixelFormat {
    /// Indicates an invalid pixel format.
    Invalid = 0,
    
    /// 8-bit normalized R component.
    R8Unorm = 10,
    
    /// 8-bit normalized RGBA components.
    RGBA8Unorm = 70,
    
    /// 8-bit normalized BGRA components.
    BGRA8Unorm = 80,
    
    /// 32-bit floating point R component.
    R32Float = 41,
    
    /// 32-bit floating point RGBA components.
    RGBA32Float = 125,
    
    /// 16-bit depth component.
    Depth16Unorm = 250,
    
    /// 32-bit floating point depth component.
    Depth32Float = 252,
    
    /// 24-bit depth and 8-bit stencil components.
    Depth24UnormStencil8 = 255,
}

/// MTLLoadAction - Enum specifying the action to take when a render pass begins.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLLoadAction {
    /// Don't care about the contents.
    DontCare = 0,
    
    /// Clear to a specified value.
    Clear = 1,
    
    /// Preserve the existing contents.
    Load = 2,
}

/// MTLStoreAction - Enum specifying the action to take when a render pass ends.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLStoreAction {
    /// Don't care about the contents after rendering.
    DontCare = 0,
    
    /// Store the results of rendering.
    Store = 1,
    
    /// Store the results of rendering and mark as resolved.
    MultisampleResolve = 2,
    
    /// Store the results of rendering and perform a custom resolve operation.
    StoreAndMultisampleResolve = 3,
    
    /// Don't store the results of rendering but perform a custom resolve operation.
    Unknown = 4,
    
    /// Mark content as resolved and discard the contents.
    CustomSampleDepthStore = 5,
}

/// MTLPrimitiveType - Enum representing the types of primitives that can be rendered.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPrimitiveType {
    /// Vertices are treated as a point list.
    Point = 0,
    
    /// Vertices are treated as a line list.
    Line = 1,
    
    /// Vertices are treated as a line strip.
    LineStrip = 2,
    
    /// Vertices are treated as a triangle list.
    Triangle = 3,
    
    /// Vertices are treated as a triangle strip.
    TriangleStrip = 4,
}