//! This module implements the [`ImagePreviewData`] type for Awgen asset
//! previews.

use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// A newtype wrapper for image preview data.
///
/// An image preview is always a 128x128 RGBA image, stored in RGBA8UnormSrgb
/// format.
#[derive(Debug, Clone)]
pub struct ImagePreviewData(Vec<u8>);
impl ImagePreviewData {
    /// The width of the preview image in pixels.
    pub const WIDTH: usize = 128;

    /// The height of the preview image in pixels.
    pub const HEIGHT: usize = 128;

    /// The number of bits per pixel in the preview image.
    pub const BITS_PER_PIXEL: usize = 4;

    /// Creates a new placeholder preview image filled with white pixels.
    pub fn new() -> Self {
        Self(vec![255; Self::WIDTH * Self::HEIGHT * Self::BITS_PER_PIXEL])
    }
}

impl<I> Index<I> for ImagePreviewData
where
    I: SliceIndex<[u8]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I> IndexMut<I> for ImagePreviewData
where
    I: SliceIndex<[u8]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Default for ImagePreviewData {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ImagePreviewData> for Image {
    fn from(preview: ImagePreviewData) -> Self {
        Image::new(
            Extent3d {
                width: ImagePreviewData::WIDTH as u32,
                height: ImagePreviewData::HEIGHT as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            preview.0,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        )
    }
}
