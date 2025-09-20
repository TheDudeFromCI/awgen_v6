//! This module implements a block model and how to render it.

use bevy::prelude::*;

use crate::tiles::{TerrainMesh, TerrainPoly, TerrainQuad, TileRot};

/// Contains the definition for a block on the map, and how it should be
/// rendered.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Default, Clone)]
pub enum BlockModel {
    /// An empty block, which does not render anything.
    #[default]
    Empty,

    /// A unit cube.
    Cube {
        /// The tile information for the top (Y+) face of the cube, or none if
        /// that face is not rendered.
        up: Option<QuadFace>,

        /// The tile information for the north (Z+) face of the cube, or none if
        /// that face is not rendered.
        north: Option<QuadFace>,

        /// The tile information for the south (Z-) face of the cube, or none if
        /// that face is not rendered.
        south: Option<QuadFace>,

        /// The tile information for the east (X+) face of the cube, or none if
        /// that face is not rendered.
        east: Option<QuadFace>,

        /// The tile information for the west (X-) face of the cube, or none if
        /// that face is not rendered.
        west: Option<QuadFace>,
    },
}

impl BlockModel {
    /// Draws the block into the provided mesh at the specified transform.
    pub fn draw(&self, mesh: &mut TerrainMesh, transform: Transform) {
        match self {
            BlockModel::Empty => {}
            BlockModel::Cube {
                up,
                north,
                south,
                east,
                west,
            } => {
                if let Some(face) = up {
                    let mut quad = TerrainQuad::unit();
                    quad.shift(Vec3::Y);

                    quad.scale(transform.scale);
                    quad.rotate(transform.rotation);
                    quad.shift(transform.translation);
                    quad.rotate_uv(face.tile_rot);
                    quad.set_layer(face.tile_index);
                    mesh.add_polygon(quad);
                }

                if let Some(face) = east {
                    let mut quad = TerrainQuad::unit();
                    quad.rotate(Quat::from_rotation_x(90f32.to_radians()));
                    quad.shift(Vec3::new(0.0, 0.5, 0.5));

                    quad.scale(transform.scale);
                    quad.rotate(transform.rotation);
                    quad.shift(transform.translation);
                    quad.rotate_uv(face.tile_rot);
                    quad.set_layer(face.tile_index);
                    mesh.add_polygon(quad);
                }

                if let Some(face) = west {
                    let mut quad = TerrainQuad::unit();
                    quad.rotate(Quat::from_rotation_x(-90f32.to_radians()));
                    quad.shift(Vec3::new(0.0, 0.5, -0.5));

                    quad.scale(transform.scale);
                    quad.rotate(transform.rotation);
                    quad.shift(transform.translation);
                    quad.rotate_uv(face.tile_rot);
                    quad.set_layer(face.tile_index);
                    mesh.add_polygon(quad);
                }

                if let Some(face) = north {
                    let mut quad = TerrainQuad::unit();
                    quad.rotate(Quat::from_rotation_z(-90f32.to_radians()));
                    quad.shift(Vec3::new(0.5, 0.5, 0.0));

                    quad.scale(transform.scale);
                    quad.rotate(transform.rotation);
                    quad.shift(transform.translation);
                    quad.rotate_uv(face.tile_rot);
                    quad.set_layer(face.tile_index);
                    mesh.add_polygon(quad);
                }

                if let Some(face) = south {
                    let mut quad = TerrainQuad::unit();
                    quad.rotate(Quat::from_rotation_z(90f32.to_radians()));
                    quad.shift(Vec3::new(-0.5, 0.5, 0.0));

                    quad.scale(transform.scale);
                    quad.rotate(transform.rotation);
                    quad.shift(transform.translation);
                    quad.rotate_uv(face.tile_rot);
                    quad.set_layer(face.tile_index);
                    mesh.add_polygon(quad);
                }
            }
        }
    }
}

/// Represents a face of a block, which contains tile information for rendering.
#[derive(Debug, Default, Clone, Copy)]
pub struct QuadFace {
    /// The tile index for the block face.
    pub tile_index: u32,

    /// The tile rotation for the block face.
    pub tile_rot: TileRot,
}
