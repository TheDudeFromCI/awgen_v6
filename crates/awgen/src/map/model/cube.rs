//! This module implements the cube block model.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map::Occlusion;
use crate::map::model::TileFace;
use crate::tiles::{TerrainMesh, TerrainPoly, TerrainQuad};

/// A cube block model.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct Cube {
    /// The tile information for the top (Y+) face of the cube.
    pub pos_y: TileFace,

    /// The tile information for the north (Z+) face of the cube.
    pub pos_z: TileFace,

    /// The tile information for the south (Z-) face of the cube.
    pub neg_z: TileFace,

    /// The tile information for the east (X+) face of the cube.
    pub pos_x: TileFace,

    /// The tile information for the west (X-) face of the cube.
    pub neg_x: TileFace,
}

impl Cube {
    /// Draws the cube into the provided mesh at the specified transform.
    pub fn draw(&self, mesh: &mut TerrainMesh, transform: Transform, occlusion: Occlusion) {
        // pos y
        if !occlusion.contains(Occlusion::PosY) {
            let mut quad = TerrainQuad::unit();
            quad.shift(Vec3::Y);
            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(self.pos_y.rotation);
            quad.set_layer(self.pos_y.tile_index);
            mesh.add_polygon(quad);
        }

        // pos x
        if !occlusion.contains(Occlusion::PosZ) {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_x(90f32.to_radians()));
            quad.shift(Vec3::new(0.0, 0.5, 0.5));
            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(self.pos_z.rotation);
            quad.set_layer(self.pos_z.tile_index);
            mesh.add_polygon(quad);
        }

        // neg x
        if !occlusion.contains(Occlusion::NegZ) {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_x(-90f32.to_radians()));
            quad.shift(Vec3::new(0.0, 0.5, -0.5));
            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(self.neg_z.rotation);
            quad.set_layer(self.neg_z.tile_index);
            mesh.add_polygon(quad);
        }

        // pos z
        if !occlusion.contains(Occlusion::PosX) {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_z(-90f32.to_radians()));
            quad.shift(Vec3::new(0.5, 0.5, 0.0));
            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(self.pos_x.rotation);
            quad.set_layer(self.pos_x.tile_index);
            mesh.add_polygon(quad);
        }

        // neg z
        if !occlusion.contains(Occlusion::NegX) {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_z(90f32.to_radians()));
            quad.shift(Vec3::new(-0.5, 0.5, 0.0));
            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(self.neg_x.rotation);
            quad.set_layer(self.neg_x.tile_index);
            mesh.add_polygon(quad);
        }
    }
}
