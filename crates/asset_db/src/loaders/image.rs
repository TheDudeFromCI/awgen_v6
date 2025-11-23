//! Awgen image asset loader and saver.

use std::io::{Read, Write};

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext, RenderAssetUsages};
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d,
    TextureDataOrder,
    TextureDescriptor,
    TextureDimension,
    TextureFormat,
    TextureUsages,
};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use crate::loaders::{AssetDataError, AwgenAsset, ByteWriter};

/// The Awgen image asset type name.
pub const AWGEN_IMAGE_TYPE: &str = "awgen_image";

/// The magic number used to identify Awgen image assets.
const MAGIC_NUMBER: &[u8] = AWGEN_IMAGE_TYPE.as_bytes();

impl AwgenAsset for Image {
    fn type_name() -> &'static str {
        AWGEN_IMAGE_TYPE
    }

    fn save(&self) -> Result<Vec<u8>, AssetDataError> {
        let mut writer = ByteWriter::new();
        writer.write_all(MAGIC_NUMBER)?;

        if self.texture_descriptor.size.depth_or_array_layers != 1 {
            return Err(AssetDataError(String::from("Only 2D images are supported")));
        }

        if self.texture_descriptor.format != TextureFormat::Rgba8UnormSrgb {
            return Err(AssetDataError(String::from(
                "Only Rgba8UnormSrgb format is supported",
            )));
        }

        if self.data_order != TextureDataOrder::LayerMajor {
            return Err(AssetDataError(String::from(
                "Only LayerMajor data order is supported",
            )));
        }

        let width = self.texture_descriptor.size.width as i32;
        let height = self.texture_descriptor.size.height as i32;
        let mipmaps = self.texture_descriptor.mip_level_count as i32;
        writer.write_num(width)?;
        writer.write_num(height)?;
        writer.write_num(mipmaps)?;

        let Some(data) = &self.data else {
            return Err(AssetDataError(String::from("Image has no data")));
        };

        let mut encoder = ZlibEncoder::new(writer, Compression::new(4));
        encoder.write_all(data)?;

        let writer = encoder.finish()?;
        Ok(writer.data)
    }
}

/// Awgen image asset loader.
pub struct AwgenImageAssetLoader;
impl AssetLoader for AwgenImageAssetLoader {
    type Asset = Image;
    type Settings = ();
    type Error = AssetDataError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        _: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut magic_number = [0u8; MAGIC_NUMBER.len()];
        reader.read_exact(&mut magic_number).await?;

        if magic_number != MAGIC_NUMBER {
            return Err(AssetDataError(String::from("Invalid image format")));
        }

        let mut int_buf = [0u8; 4];

        reader.read_exact(&mut int_buf).await?;
        let width = i32::from_le_bytes(int_buf);

        reader.read_exact(&mut int_buf).await?;
        let height = i32::from_le_bytes(int_buf);

        reader.read_exact(&mut int_buf).await?;
        let mipmaps = i32::from_le_bytes(int_buf);

        let mut compressed_data = Vec::new();
        reader.read_to_end(&mut compressed_data).await?;

        let mut decoder = ZlibDecoder::new(compressed_data.as_slice());

        let mut uncompressed_data = Vec::new();
        decoder.read_to_end(&mut uncompressed_data)?;

        Ok(Image {
            data: Some(uncompressed_data),
            data_order: TextureDataOrder::LayerMajor,
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: width as u32,
                    height: height as u32,
                    depth_or_array_layers: 1,
                },
                mip_level_count: mipmaps as u32,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            sampler: ImageSampler::nearest(),
            texture_view_descriptor: None,
            asset_usage: RenderAssetUsages::RENDER_WORLD,
            copy_on_resize: false,
        })
    }

    fn extensions(&self) -> &[&str] {
        &[AWGEN_IMAGE_TYPE]
    }
}
