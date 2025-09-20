//! This module implements the [`TileRot`] enum, which represents the rotation
//! of a tile on a block face.

#![allow(missing_docs)]

use bevy::prelude::*;

/// Represents the rotation of a tile on a block face.
#[derive(Debug, Default, Clone, Copy)]
pub struct TileRot(Mat2);

impl TileRot {
    /// Rotates the tile by the specified angle in degrees.
    pub fn rotate(&mut self, angle: f32) {
        self.0 *= Mat2::from_angle(angle.to_radians());
    }

    /// Mirrors the tile across the X-axis.
    pub fn mirror_x(&mut self) {
        self.0 *= Mat2::from_scale_angle(Vec2::new(-1.0, 1.0), 0.0);
    }

    /// Mirrors the tile across the Y-axis.
    pub fn mirror_y(&mut self) {
        self.0 *= Mat2::from_scale_angle(Vec2::new(1.0, -1.0), 0.0);
    }

    /// Transforms the UV coordinates of a tile using the rotation matrix.
    pub fn transform_uv(&self, uv: Vec2) -> Vec2 {
        self.0 * uv
    }

    /// Rotates the tile by the specified angle in degrees and returns a new
    /// [`TileRot`]. This method is identical to [`TileRot::rotate`], but
    /// returns self for chaining.
    pub fn into_rotated(mut self, angle: f32) -> Self {
        self.rotate(angle);
        self
    }

    /// Mirrors the tile across the X-axis and returns a new [`TileRot`]. This
    /// method is identical to [`TileRot::mirror_x`], but returns self for
    /// chaining.
    pub fn into_mirrored_x(mut self) -> Self {
        self.mirror_x();
        self
    }

    /// Mirrors the tile across the Y-axis and returns a new [`TileRot`]. This
    /// method is identical to [`TileRot::mirror_y`], but returns self for
    /// chaining.
    pub fn into_mirrored_y(mut self) -> Self {
        self.mirror_y();
        self
    }
}
