//! This module implements a builder pattern for creating a mesh that can be
//! used to render terrain with a tileset.

use std::ops::Mul;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, MeshVertexAttribute, PrimitiveTopology};
use bevy::render::render_resource::VertexFormat;

/// A vertex attribute that stores the texture coordinates `(x, y)` of a vertex
/// and the texture array layer it belongs to `(z)`.
pub const ATTRIBUTE_UV_LAYER: MeshVertexAttribute =
    MeshVertexAttribute::new("UvLayer", 4039395644538880, VertexFormat::Float32x3);

/// A temporary buffer for storing mesh data capable of rendering terrain.
#[derive(Debug, Default, Clone)]
pub struct TerrainMesh {
    /// The vertex positions of the mesh.
    positions: Vec<[f32; 3]>,

    /// The vertex texture coordinates of the mesh.
    uvs: Vec<[f32; 3]>,

    /// The vertex normals of the mesh.
    normals: Vec<[f32; 3]>,

    /// The vertex colors of the mesh.
    colors: Vec<[f32; 4]>,

    /// The indices of the mesh.
    indices: Vec<u32>,
}

impl TerrainMesh {
    /// The initial capacity of the vertices.
    const INIT_CAPACITY_VERTS: usize = 1024;

    /// The initial capacity of the indices.
    const INIT_CAPACITY_INDICES: usize = 2048;

    /// Creates a new terrain mesh.
    pub fn new() -> Self {
        Self {
            positions: Vec::with_capacity(Self::INIT_CAPACITY_VERTS),
            uvs: Vec::with_capacity(Self::INIT_CAPACITY_VERTS),
            normals: Vec::with_capacity(Self::INIT_CAPACITY_VERTS),
            colors: Vec::with_capacity(Self::INIT_CAPACITY_VERTS),
            indices: Vec::with_capacity(Self::INIT_CAPACITY_INDICES),
        }
    }

    /// Gets a reference to the vertex positions of the mesh.
    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }

    /// Gets a reference to the indices of the mesh.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Gets a reference to the vertex texture coordinates of the mesh.
    pub fn tex_coords(&self) -> &[[f32; 3]] {
        &self.uvs
    }

    /// Gets a reference to the vertex normals of the mesh.
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    /// Gets a reference to the vertex colors of the mesh.
    pub fn colors(&self) -> &[[f32; 4]] {
        &self.colors
    }

    /// Gets the number of triangles in the mesh.
    pub fn tri_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Appends the mesh data from another mesh to this mesh.
    pub fn append(&mut self, other: &Self, transform: Transform) {
        let offset = self.positions.len() as u32;
        let matrix = transform.compute_matrix();

        self.positions.reserve(other.positions.len());
        for position in &other.positions {
            let position = matrix * Vec4::new(position[0], position[1], position[2], 1.0);
            self.positions.push([position.x, position.y, position.z]);
        }

        self.normals.reserve(other.normals.len());
        for normal in &other.normals {
            let normal = matrix * Vec4::new(normal[0], normal[1], normal[2], 0.0);
            self.normals.push([normal.x, normal.y, normal.z]);
        }

        self.uvs.extend_from_slice(&other.uvs);
        self.colors.extend_from_slice(&other.colors);

        self.indices
            .extend(other.indices.iter().map(|i| i + offset));
    }

    /// Appends a triangle to the mesh.
    pub fn add_triangle(&mut self, triangle: TerrainTriangle) {
        let offset = self.positions.len() as u32;

        for vert in triangle.vertices() {
            let pos = [vert.position.x, vert.position.y, vert.position.z];
            let uv = [vert.uv.x, vert.uv.y, vert.layer as f32];
            let normal = [vert.normal.x, vert.normal.y, vert.normal.z];

            let color = vert.color.to_srgba();
            let color = [color.red, color.green, color.blue, color.alpha];

            self.positions.push(pos);
            self.uvs.push(uv);
            self.normals.push(normal);
            self.colors.push(color);
        }

        self.indices.push(offset);
        self.indices.push(offset + 1);
        self.indices.push(offset + 2);
    }

    /// Appends a quad to the mesh.
    pub fn add_quad(&mut self, quad: TerrainQuad) {
        let offset = self.positions.len() as u32;

        for vert in quad.vertices() {
            let pos = [vert.position.x, vert.position.y, vert.position.z];
            let uv = [vert.uv.x, vert.uv.y, vert.layer as f32];
            let normal = [vert.normal.x, vert.normal.y, vert.normal.z];

            let color = vert.color.to_srgba();
            let color = [color.red, color.green, color.blue, color.alpha];

            self.positions.push(pos);
            self.uvs.push(uv);
            self.normals.push(normal);
            self.colors.push(color);
        }

        self.indices.push(offset);
        self.indices.push(offset + 1);
        self.indices.push(offset + 2);

        self.indices.push(offset);
        self.indices.push(offset + 2);
        self.indices.push(offset + 3);
    }
}

impl From<TerrainMesh> for Mesh {
    fn from(value: TerrainMesh) -> Self {
        let indices = if value.indices.len() > u16::MAX as usize {
            Indices::U32(value.indices)
        } else {
            Indices::U16(value.indices.iter().map(|&i| i as u16).collect())
        };

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, value.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, value.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, value.colors)
        .with_inserted_attribute(ATTRIBUTE_UV_LAYER, value.uvs)
        .with_inserted_indices(indices)
    }
}

/// A vertex that stores the position, normal, texture coordinates, layer, and
/// color of a terrain vertex.
#[derive(Debug, Clone, Copy)]
pub struct TerrainVertex {
    /// The position of the vertex.
    pub position: Vec3,

    /// The normal of the vertex.
    pub normal: Vec3,

    /// The texture coordinates of the vertex.
    pub uv: Vec2,

    /// The texture array layer of the vertex.
    pub layer: u32,

    /// The color of the vertex.
    pub color: Color,
}

impl Mul<TerrainVertex> for Mat4 {
    type Output = TerrainVertex;

    fn mul(self, rhs: TerrainVertex) -> Self::Output {
        let pos4 = self * Vec4::new(rhs.position.x, rhs.position.y, rhs.position.z, 1.0);
        let norm4 = self * Vec4::new(rhs.normal.x, rhs.normal.y, rhs.normal.z, 0.0);

        TerrainVertex {
            position: pos4.xyz(),
            normal: norm4.xyz(),
            uv: rhs.uv,
            layer: rhs.layer,
            color: rhs.color,
        }
    }
}

impl Mul<TerrainVertex> for Transform {
    type Output = TerrainVertex;

    fn mul(self, rhs: TerrainVertex) -> Self::Output {
        self.compute_matrix() * rhs
    }
}

/// A triangle that stores the vertices for a [`TerrainMesh`].
#[derive(Debug, Clone, Copy)]
pub struct TerrainTriangle(pub TerrainVertex, pub TerrainVertex, pub TerrainVertex);

impl TerrainTriangle {
    /// Returns an array of the vertices of the triangle.
    fn vertices(&self) -> [TerrainVertex; 3] {
        [self.0, self.1, self.2]
    }
}

/// A quad that stores the vertices for a [`TerrainMesh`].
#[derive(Debug, Clone, Copy)]
pub struct TerrainQuad(
    pub TerrainVertex,
    pub TerrainVertex,
    pub TerrainVertex,
    pub TerrainVertex,
);

impl TerrainQuad {
    /// Creates a new `TerrainQuad` centered at the origin with a size of 1x1
    /// and a normal of `Vec3::Y`.
    pub fn unit() -> Self {
        let v1 = TerrainVertex {
            position: Vec3::new(0.5, 0.0, 0.5),
            normal: Vec3::Y,
            uv: Vec2::ONE,
            layer: 0,
            color: Color::WHITE,
        };
        let v2 = TerrainVertex {
            position: Vec3::new(0.5, 0.0, -0.5),
            normal: Vec3::Y,
            uv: Vec2::X,
            layer: 0,
            color: Color::WHITE,
        };
        let v3 = TerrainVertex {
            position: Vec3::new(-0.5, 0.0, -0.5),
            normal: Vec3::Y,
            uv: Vec2::ZERO,
            layer: 0,
            color: Color::WHITE,
        };
        let v4 = TerrainVertex {
            position: Vec3::new(-0.5, 0.0, 0.5),
            normal: Vec3::Y,
            uv: Vec2::Y,
            layer: 0,
            color: Color::WHITE,
        };

        Self(v1, v2, v3, v4)
    }

    /// Shifts the quad by the given offset. Returns self for chaining.
    pub fn shift(mut self, offset: Vec3) -> Self {
        self.0.position += offset;
        self.1.position += offset;
        self.2.position += offset;
        self.3.position += offset;

        self
    }

    /// Rotates the quad by the given rotation, relative to the origin. Returns
    /// self for chaining.
    pub fn rotate(mut self, rotation: Quat) -> Self {
        self.0.position = rotation * self.0.position;
        self.1.position = rotation * self.1.position;
        self.2.position = rotation * self.2.position;
        self.3.position = rotation * self.3.position;

        self.0.normal = rotation * self.0.normal;
        self.1.normal = rotation * self.1.normal;
        self.2.normal = rotation * self.2.normal;
        self.3.normal = rotation * self.3.normal;

        self
    }

    /// Scales the quad by the given scale factor, relative to the origin.
    /// Returns self for chaining.
    pub fn scale(mut self, scale: Vec3) -> Self {
        self.0.position *= scale;
        self.1.position *= scale;
        self.2.position *= scale;
        self.3.position *= scale;

        self.0.normal = self.0.normal.normalize();
        self.1.normal = self.1.normal.normalize();
        self.2.normal = self.2.normal.normalize();
        self.3.normal = self.3.normal.normalize();

        self
    }

    /// Sets the layer of the quad. This is used to determine which texture
    /// array layer the quad belongs to.
    pub fn set_layer(mut self, layer: u32) -> Self {
        self.0.layer = layer;
        self.1.layer = layer;
        self.2.layer = layer;
        self.3.layer = layer;

        self
    }

    /// Returns an array of the vertices of the quad.
    fn vertices(&self) -> [TerrainVertex; 4] {
        [self.0, self.1, self.2, self.3]
    }
}
