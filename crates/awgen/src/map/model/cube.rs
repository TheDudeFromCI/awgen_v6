//! This module implements the cube block model.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map::model::TileFace;
use crate::tiles::{TerrainMesh, TerrainPoly, TerrainQuad};

/// A cube block model.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct Cube {
    /// The tile information for the top (Y+) face of the cube, or none if
    /// that face is not rendered.
    pub up: Option<TileFace>,

    /// The tile information for the north (Z+) face of the cube, or none if
    /// that face is not rendered.
    pub north: Option<TileFace>,

    /// The tile information for the south (Z-) face of the cube, or none if
    /// that face is not rendered.
    pub south: Option<TileFace>,

    /// The tile information for the east (X+) face of the cube, or none if
    /// that face is not rendered.
    pub east: Option<TileFace>,

    /// The tile information for the west (X-) face of the cube, or none if
    /// that face is not rendered.
    pub west: Option<TileFace>,
}

impl Cube {
    /// Draws the cube into the provided mesh at the specified transform.
    pub fn draw(&self, mesh: &mut TerrainMesh, transform: Transform) {
        if let Some(face) = self.up {
            let mut quad = TerrainQuad::unit();
            quad.shift(Vec3::Y);

            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(face.rotation);
            quad.set_layer(face.tile_index);
            mesh.add_polygon(quad);
        }

        if let Some(face) = self.east {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_x(90f32.to_radians()));
            quad.shift(Vec3::new(0.0, 0.5, 0.5));

            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(face.rotation);
            quad.set_layer(face.tile_index);
            mesh.add_polygon(quad);
        }

        if let Some(face) = self.west {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_x(-90f32.to_radians()));
            quad.shift(Vec3::new(0.0, 0.5, -0.5));

            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(face.rotation);
            quad.set_layer(face.tile_index);
            mesh.add_polygon(quad);
        }

        if let Some(face) = self.north {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_z(-90f32.to_radians()));
            quad.shift(Vec3::new(0.5, 0.5, 0.0));

            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(face.rotation);
            quad.set_layer(face.tile_index);
            mesh.add_polygon(quad);
        }

        if let Some(face) = self.south {
            let mut quad = TerrainQuad::unit();
            quad.rotate(Quat::from_rotation_z(90f32.to_radians()));
            quad.shift(Vec3::new(-0.5, 0.5, 0.0));

            quad.scale(transform.scale);
            quad.rotate(transform.rotation);
            quad.shift(transform.translation);
            quad.rotate_uv(face.rotation);
            quad.set_layer(face.tile_index);
            mesh.add_polygon(quad);
        }
    }
}
