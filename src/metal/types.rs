//! Common types and constants used in Metal API.
//!
//! This module contains various common structures and enumerations used throughout 
//! the Metal API, such as pixel formats, clear colors, and other basic types.

// MTLClearColor has been moved to render_pass module

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

// MTLLoadAction and MTLStoreAction have been moved to render_pass module

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