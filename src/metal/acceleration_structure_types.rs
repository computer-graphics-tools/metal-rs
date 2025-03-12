//! MTLAccelerationStructureTypes - A Rust wrapper around Metal's acceleration structure data types.
//!
//! This module provides Rust bindings to the data types used with MTLAccelerationStructure
//! from Apple's Metal framework. These types are essential for ray tracing and geometry handling
//! in Metal-based GPU applications.

use std::fmt;
use std::ops::{Index, IndexMut};

/// A 3D vector of 32-bit floating-point values stored in a packed format.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PackedFloat3 {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
    /// The z-component of the vector.
    pub z: f32,
}

impl PackedFloat3 {
    /// Creates a new `PackedFloat3` with the specified components.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::PackedFloat3;
    ///
    /// let vector = PackedFloat3::new(1.0, 2.0, 3.0);
    /// assert_eq!(vector.x, 1.0);
    /// assert_eq!(vector.y, 2.0);
    /// assert_eq!(vector.z, 3.0);
    /// ```
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Default for PackedFloat3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// A quaternion of 32-bit floating-point values stored in a packed format.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PackedFloatQuaternion {
    /// The x-component of the quaternion.
    pub x: f32,
    /// The y-component of the quaternion.
    pub y: f32,
    /// The z-component of the quaternion.
    pub z: f32,
    /// The w-component of the quaternion.
    pub w: f32,
}

impl PackedFloatQuaternion {
    /// Creates a new `PackedFloatQuaternion` with the specified components.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::PackedFloatQuaternion;
    ///
    /// let quaternion = PackedFloatQuaternion::new(0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(quaternion.x, 0.0);
    /// assert_eq!(quaternion.y, 0.0);
    /// assert_eq!(quaternion.z, 0.0);
    /// assert_eq!(quaternion.w, 1.0);
    /// ```
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a new identity quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::PackedFloatQuaternion;
    ///
    /// let quaternion = PackedFloatQuaternion::identity();
    /// assert_eq!(quaternion.x, 0.0);
    /// assert_eq!(quaternion.y, 0.0);
    /// assert_eq!(quaternion.z, 0.0);
    /// assert_eq!(quaternion.w, 1.0);
    /// ```
    #[must_use]
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Default for PackedFloatQuaternion {
    fn default() -> Self {
        Self::identity()
    }
}

/// A 4x3 matrix of 32-bit floating-point values stored in a packed format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PackedFloat4x3 {
    /// The rows of the matrix.
    pub rows: [PackedFloat3; 4],
}

impl PackedFloat4x3 {
    /// Creates a new `PackedFloat4x3` with the specified rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{PackedFloat3, PackedFloat4x3};
    ///
    /// let row0 = PackedFloat3::new(1.0, 0.0, 0.0);
    /// let row1 = PackedFloat3::new(0.0, 1.0, 0.0);
    /// let row2 = PackedFloat3::new(0.0, 0.0, 1.0);
    /// let row3 = PackedFloat3::new(0.0, 0.0, 0.0);
    ///
    /// let matrix = PackedFloat4x3::new([row0, row1, row2, row3]);
    /// ```
    #[must_use]
    pub fn new(rows: [PackedFloat3; 4]) -> Self {
        Self { rows }
    }

    /// Creates a new identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::PackedFloat4x3;
    ///
    /// let matrix = PackedFloat4x3::identity();
    /// ```
    #[must_use]
    pub fn identity() -> Self {
        Self {
            rows: [
                PackedFloat3::new(1.0, 0.0, 0.0),
                PackedFloat3::new(0.0, 1.0, 0.0),
                PackedFloat3::new(0.0, 0.0, 1.0),
                PackedFloat3::new(0.0, 0.0, 0.0),
            ],
        }
    }
}

impl Default for PackedFloat4x3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Index<usize> for PackedFloat4x3 {
    type Output = PackedFloat3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for PackedFloat4x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl fmt::Debug for PackedFloat4x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackedFloat4x3")
            .field("rows", &self.rows)
            .finish()
    }
}

/// An axis-aligned bounding box defined by minimum and maximum points.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AxisAlignedBoundingBox {
    /// The minimum point of the bounding box.
    pub min: PackedFloat3,
    /// The maximum point of the bounding box.
    pub max: PackedFloat3,
}

impl AxisAlignedBoundingBox {
    /// Creates a new `AxisAlignedBoundingBox` with the specified minimum and maximum points.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{AxisAlignedBoundingBox, PackedFloat3};
    ///
    /// let min = PackedFloat3::new(-1.0, -1.0, -1.0);
    /// let max = PackedFloat3::new(1.0, 1.0, 1.0);
    ///
    /// let aabb = AxisAlignedBoundingBox::new(min, max);
    /// ```
    #[must_use]
    pub fn new(min: PackedFloat3, max: PackedFloat3) -> Self {
        Self { min, max }
    }

    /// Creates a new empty `AxisAlignedBoundingBox`.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::AxisAlignedBoundingBox;
    ///
    /// let aabb = AxisAlignedBoundingBox::empty();
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self {
            min: PackedFloat3::new(f32::MAX, f32::MAX, f32::MAX),
            max: PackedFloat3::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }

    /// Returns whether the bounding box is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::AxisAlignedBoundingBox;
    ///
    /// let aabb = AxisAlignedBoundingBox::empty();
    /// assert!(aabb.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    /// Returns the center of the bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{AxisAlignedBoundingBox, PackedFloat3};
    ///
    /// let min = PackedFloat3::new(-1.0, -1.0, -1.0);
    /// let max = PackedFloat3::new(1.0, 1.0, 1.0);
    ///
    /// let aabb = AxisAlignedBoundingBox::new(min, max);
    /// let center = aabb.center();
    ///
    /// assert_eq!(center.x, 0.0);
    /// assert_eq!(center.y, 0.0);
    /// assert_eq!(center.z, 0.0);
    /// ```
    #[must_use]
    pub fn center(&self) -> PackedFloat3 {
        PackedFloat3::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
            (self.min.z + self.max.z) * 0.5,
        )
    }

    /// Returns the size of the bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{AxisAlignedBoundingBox, PackedFloat3};
    ///
    /// let min = PackedFloat3::new(-1.0, -1.0, -1.0);
    /// let max = PackedFloat3::new(1.0, 1.0, 1.0);
    ///
    /// let aabb = AxisAlignedBoundingBox::new(min, max);
    /// let size = aabb.size();
    ///
    /// assert_eq!(size.x, 2.0);
    /// assert_eq!(size.y, 2.0);
    /// assert_eq!(size.z, 2.0);
    /// ```
    #[must_use]
    pub fn size(&self) -> PackedFloat3 {
        PackedFloat3::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }
}

impl Default for AxisAlignedBoundingBox {
    fn default() -> Self {
        Self::empty()
    }
}

/// A component-based transformation that can be used with ray tracing.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ComponentTransform {
    /// The scale component of the transformation.
    pub scale: PackedFloat3,
    /// The shear component of the transformation.
    pub shear: PackedFloat3,
    /// The pivot position of the transformation.
    pub pivot: PackedFloat3,
    /// The rotation component of the transformation.
    pub rotation: PackedFloatQuaternion,
    /// The translation component of the transformation.
    pub translation: PackedFloat3,
}

impl ComponentTransform {
    /// Creates a new `ComponentTransform` with the specified components.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::{ComponentTransform, PackedFloat3, PackedFloatQuaternion};
    ///
    /// let scale = PackedFloat3::new(1.0, 1.0, 1.0);
    /// let shear = PackedFloat3::new(0.0, 0.0, 0.0);
    /// let pivot = PackedFloat3::new(0.0, 0.0, 0.0);
    /// let rotation = PackedFloatQuaternion::identity();
    /// let translation = PackedFloat3::new(0.0, 0.0, 0.0);
    ///
    /// let transform = ComponentTransform::new(scale, shear, pivot, rotation, translation);
    /// ```
    #[must_use]
    pub fn new(
        scale: PackedFloat3,
        shear: PackedFloat3,
        pivot: PackedFloat3,
        rotation: PackedFloatQuaternion,
        translation: PackedFloat3,
    ) -> Self {
        Self {
            scale,
            shear,
            pivot,
            rotation,
            translation,
        }
    }

    /// Creates a new identity `ComponentTransform`.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::ComponentTransform;
    ///
    /// let transform = ComponentTransform::identity();
    /// ```
    #[must_use]
    pub fn identity() -> Self {
        Self {
            scale: PackedFloat3::new(1.0, 1.0, 1.0),
            shear: PackedFloat3::new(0.0, 0.0, 0.0),
            pivot: PackedFloat3::new(0.0, 0.0, 0.0),
            rotation: PackedFloatQuaternion::identity(),
            translation: PackedFloat3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Default for ComponentTransform {
    fn default() -> Self {
        Self::identity()
    }
}