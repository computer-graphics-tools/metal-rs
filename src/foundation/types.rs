//! Foundation framework common types.

use std::fmt;
use std::hash::{Hash, Hasher};

/// Objective-C integer type.
pub type NSInteger = isize;

/// Objective-C unsigned integer type.
pub type NSUInteger = usize;

/// Represents the comparison result in Objective-C.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum NSComparisonResult {
    /// Ascending order.
    OrderedAscending = -1,
    /// Equal.
    OrderedSame = 0,
    /// Descending order.
    OrderedDescending = 1,
}

/// Represents a range in Objective-C.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct NSRange {
    /// Location of the range.
    pub location: NSUInteger,
    /// Length of the range.
    pub length: NSUInteger,
}

impl NSRange {
    /// Creates a new range.
    #[inline]
    pub fn new(location: NSUInteger, length: NSUInteger) -> Self {
        NSRange { location, length }
    }
    
    /// Returns a range that represents a not-found result.
    #[inline]
    pub fn not_found() -> Self {
        NSRange {
            location: NSUInteger::MAX,
            length: 0,
        }
    }
    
    /// Returns whether the range represents a not-found result.
    #[inline]
    pub fn is_not_found(&self) -> bool {
        self.location == NSUInteger::MAX
    }
}

impl fmt::Display for NSRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{location: {}, length: {}}}", self.location, self.length)
    }
}

impl Hash for NSRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        self.length.hash(state);
    }
}

/// Defines the coordinate system of the receiver in macOS.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum NSBackingStoreType {
    /// Store pixels that become invalid when they're on the screen.
    Retained = 0,
    /// Buffered store keeps pixels until explicitly freed.
    Buffered = 2,
    /// Nonretained doesn't store pixels at all.
    Nonretained = 1,
}

/// Represents a point in a coordinate system.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct NSPoint {
    /// The x-coordinate.
    pub x: f64,
    /// The y-coordinate.
    pub y: f64,
}

impl NSPoint {
    /// Creates a new point.
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        NSPoint { x, y }
    }
}

/// Represents a size in a coordinate system.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct NSSize {
    /// The width.
    pub width: f64,
    /// The height.
    pub height: f64,
}

impl NSSize {
    /// Creates a new size.
    #[inline]
    pub fn new(width: f64, height: f64) -> Self {
        NSSize { width, height }
    }
}

/// Represents a rectangle in a coordinate system.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct NSRect {
    /// The origin point of the rectangle.
    pub origin: NSPoint,
    /// The size of the rectangle.
    pub size: NSSize,
}

impl NSRect {
    /// Creates a new rectangle.
    #[inline]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        NSRect {
            origin: NSPoint::new(x, y),
            size: NSSize::new(width, height),
        }
    }
}