//! This module implements a material that is capable of rendering tilesets.

use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup,
    RenderPipelineDescriptor,
    ShaderRef,
    SpecializedMeshPipelineError,
};

use crate::tileset::mesh::ATTRIBUTE_UV_LAYER;

/// The path to the tileset shader.
pub const TILESET_SHADER_PATH: &str = "embedded://awgen/tileset/shader.wgsl";

/// TilesetMaterial is a Bevy material that uses a shader to render tilesets.
#[derive(Debug, Default, Clone, Asset, TypePath, AsBindGroup)]
pub struct TilesetMaterial {
    /// The tileset texture, which is a 2D array of textures.
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// The alpha mode of the material.
    pub alpha_mode: AlphaMode,
}

impl Material for TilesetMaterial {
    fn vertex_shader() -> ShaderRef {
        TILESET_SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        TILESET_SHADER_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        TILESET_SHADER_PATH.into()
    }

    fn prepass_fragment_shader() -> ShaderRef {
        TILESET_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(2),
            ATTRIBUTE_UV_LAYER.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
