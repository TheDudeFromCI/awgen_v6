//! This module implements an editable tileset asset.

use bevy::asset::{LoadState, RenderAssetUsages};
use bevy::image::{ImageAddressMode, ImageFormatSetting, ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Tileset is an asset that represents a collection of square textures that can
/// be used to texture the terrain for static blocks in Awgen.
#[derive(Debug, Asset, TypePath)]
pub struct Tileset {
    /// The asset handle to the raw texture.
    image: Handle<Image>,

    /// The height and width of each texture within the tileset.
    size: u32,

    /// The number of tiles in the tileset.
    length: u32,

    /// A list of image handles that to attempt to append to the tileset when
    /// they are fully loaded. Tiles are appended in the order they appear in
    /// this list.
    pub(super) add_when_loaded: Vec<Handle<Image>>,
}

impl Tileset {
    /// Creates a new empty tileset. The tileset will not contain any tiles
    /// and will have a size and length of 0. The size of the tileset will be
    /// determined by the first texture added to this tileset.
    pub fn new(images: &mut ResMut<Assets<Image>>) -> Self {
        let mut image = Image {
            data: Some(vec![0; 4 * 4 * 4]),
            ..default()
        };

        image.asset_usage = RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD;
        image.texture_descriptor.mip_level_count = 1;
        image.texture_descriptor.dimension = TextureDimension::D2;
        image.texture_descriptor.format = TextureFormat::Rgba8UnormSrgb;
        image.texture_descriptor.size = Extent3d {
            width: 4,
            height: 4,
            depth_or_array_layers: 2,
        };

        image.sampler = ImageSampler::nearest();
        let sampler_descriptor = image.sampler.get_or_init_descriptor();
        sampler_descriptor.address_mode_u = ImageAddressMode::Repeat;
        sampler_descriptor.address_mode_v = ImageAddressMode::Repeat;
        sampler_descriptor.lod_max_clamp = 0.0;

        let handle = images.add(image);
        debug!("Created new empty tileset with handle: {:?}", &handle);

        Self {
            image: handle,
            size: 0,
            length: 0,
            add_when_loaded: Vec::new(),
        }
    }

    /// Gets the size of the tileset. All tiles must be square, so this value
    /// represents both the width and height of each tile. Tiles are always a
    /// power of two.
    ///
    /// Note that a tileset which contains no tiles will have a size of 0.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Gets the number of tiles in the tileset. This value may check as tiles
    /// are added or removed.
    ///
    /// This value does not count deferred tiles that have not yet been
    /// added to the tileset.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Gets the total number of tiles in the tileset, including deferred
    /// tiles that have not yet been added.
    pub fn length_full(&self) -> u32 {
        // This includes the deferred tiles that have not yet been added.
        self.length + self.add_when_loaded.len() as u32
    }

    /// Gets the handle to the image that contains the texture array.
    pub fn image(&self) -> &Handle<Image> {
        &self.image
    }

    /// Opens this tileset in edit mode to be modified. This will return a
    /// [`TilesetEditor`] which can be used to add or replace tiles within the
    /// tileset.
    pub fn edit<'a, 'w>(
        &'a mut self,
        images: &'a mut ResMut<'w, Assets<Image>>,
        asset_server: &'a AssetServer,
    ) -> TilesetEditor<'a, 'w> {
        TilesetEditor {
            tileset: self,
            images,
            asset_server,
        }
    }
}

/// An editor utility for modifying a tileset.
///
/// NOTE: Then added or replacing tiles within this tileset, this editor
/// requires all image assets to be fully loaded before modifying the tileset.
pub struct TilesetEditor<'a, 'w> {
    /// The tileset being edited.
    tileset: &'a mut Tileset,

    /// The image assets.
    images: &'a mut ResMut<'w, Assets<Image>>,

    /// The asset server used to load images.
    asset_server: &'a AssetServer,
}

impl TilesetEditor<'_, '_> {
    /// Gets the size of each tile within the tileset. This is the same as
    /// [`Tileset::size`].
    ///
    /// Note that a tileset which contains no tiles will have a size of 0. The
    /// size will be determined by the first tile added to the tileset.
    pub fn size(&self) -> u32 {
        self.tileset.size
    }

    /// Gets the number of tiles in the tileset. This is the same as
    /// [`Tileset::length`].
    pub fn length(&self) -> u32 {
        self.tileset.length
    }

    /// Reads the binary data from the given image handle and processes it into
    /// tile data.
    fn get_data(&self, source: &Handle<Image>) -> Result<TileData, TilesetEditorError> {
        match self.asset_server.get_load_state(source) {
            None => {
                return Err(TilesetEditorError::TextureNotFound(format!(
                    "{:?}",
                    &source
                )));
            }
            Some(LoadState::Loaded) => {}
            Some(LoadState::Loading) | Some(LoadState::NotLoaded) => {
                return Err(TilesetEditorError::TileLoading(format!(
                    "{:?} is still loading",
                    &source
                )));
            }
            Some(LoadState::Failed(..)) => {
                return Err(TilesetEditorError::DataInaccessible(format!(
                    "{:?} failed to load",
                    &source
                )));
            }
        }

        let Some(tile_img) = self.images.get(source) else {
            return Err(TilesetEditorError::TextureNotFound(format!(
                "{:?}",
                &source
            )));
        };

        let tile_size = tile_img.size();

        if tile_size.x != tile_size.y {
            return Err(TilesetEditorError::TileNotSquare(tile_size.x, tile_size.y));
        }

        if !is_power_of_two(tile_size.x) {
            return Err(TilesetEditorError::TileNotPowerOfTwo(tile_size.x));
        }

        let Some(tile_data) = &tile_img.data else {
            return Err(TilesetEditorError::DataInaccessible(format!(
                "{:?}",
                &source
            )));
        };

        let mipmaps = mipmap_count(tile_size.x);
        let mut binary = tile_data.clone();
        generate_mipmaps(&mut binary, tile_size.x, mipmaps);

        Ok(TileData {
            binary,
            size: tile_size.x,
            mipmaps,
        })
    }

    /// Adds a new tile to the tileset. The tile will be created from the given
    /// image handle. The image must be fully loaded before calling this method.
    pub fn add_tile(&mut self, source: &Handle<Image>) -> Result<(), TilesetEditorError> {
        let tile_data = self.get_data(source)?;

        let image = self.images.get_mut(&self.tileset.image).ok_or_else(|| {
            TilesetEditorError::TextureNotFound(format!("{:?}", &self.tileset.image))
        })?;
        let Some(tileset_data) = &mut image.data else {
            return Err(TilesetEditorError::DataInaccessible(format!(
                "{:?}",
                &self.tileset.image
            )));
        };

        if self.tileset.length == 0 {
            self.tileset.size = tile_data.size;
            image.sampler.get_or_init_descriptor().lod_max_clamp = tile_data.mipmaps as f32;
            image.texture_descriptor.mip_level_count = tile_data.mipmaps + 1;
        } else if tile_data.size != self.tileset.size {
            return Err(TilesetEditorError::TileSizeMismatch(
                self.tileset.size,
                tile_data.size,
            ));
        }

        self.tileset.length += 1;
        image.texture_descriptor.size = Extent3d {
            width: self.tileset.size,
            height: self.tileset.size,
            depth_or_array_layers: self.tileset.length.max(2),
        };

        let block_size = expected_byte_size(self.tileset.size, tile_data.mipmaps);
        let expected_size = block_size * (self.tileset.length as usize).max(2);

        if tileset_data.len() != expected_size {
            tileset_data.resize(expected_size, 0);
        }

        let offset = block_size * (self.tileset.length as usize - 1);
        let end = offset + tile_data.binary.len();
        debug!(
            "Writing tile data to tileset. offset: {offset}, src len: {block_size}, dst len: {expected_size}"
        );

        tileset_data[offset .. end].copy_from_slice(&tile_data.binary);

        info!(
            "Added tile ({:?}) to tileset ({:?})",
            source,
            self.tileset.image()
        );

        Ok(())
    }

    /// Appends a new image to the tileset when it is fully loaded. This method
    /// will wait until the image is fully loaded before adding it to the
    /// end of the tileset. Tiles are appended in the order they are added.
    ///
    /// Note that even if this method is called on a fully loaded image source,
    /// it may not be appended to the tileset immediately.
    pub fn append_deferred(&mut self, asset_path: &str) {
        self.tileset
            .add_when_loaded
            .push(self.asset_server.load_with_settings(
                asset_path,
                |settings: &mut ImageLoaderSettings| {
                    settings.format = ImageFormatSetting::FromExtension;
                    settings.is_srgb = true;
                    settings.sampler = ImageSampler::nearest();
                    settings.asset_usage =
                        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD;
                },
            ));
    }
}

/// Errors that can be thrown while editing a tileset.
#[derive(Debug, thiserror::Error)]
pub enum TilesetEditorError {
    /// An error that occurs when a texture asset cannot be found.
    #[error("The texture asset cannot be found: {0}")]
    TextureNotFound(String),

    /// An error that occurs when a texture's data is not loaded or accessible.
    /// (This can occur if the data is offloaded to the GPU without CPU
    /// persistence.)
    #[error("The texture data is not loaded or accessible: {0}")]
    DataInaccessible(String),

    /// An error that occurs when attempting to add a tile that is not square.
    #[error("The tile is not square: {0}x{1}")]
    TileNotSquare(u32, u32),

    /// An error that occurs when attempting to add a tile that is not a power
    /// of two.
    #[error("The tile size is not a power of two: {0}")]
    TileNotPowerOfTwo(u32),

    /// An error that occurs when the size of the tile does not match the
    /// expected size.
    #[error("Tile size does not match the tileset: expected {0}x{0}, got {1}x{1}")]
    TileSizeMismatch(u32, u32),

    /// An error that occurs when the tile data is still being loaded.
    #[error("The tile data is still being loaded: {0}")]
    TileLoading(String),
}

/// Checks if the given number is a power of two.
fn is_power_of_two(n: u32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

/// Calculates the number of mipmaps for the given image size.
fn mipmap_count(size: u32) -> u32 {
    let mut count = 0;
    let mut s = size;
    while s > 4 {
        count += 1;
        s /= 2;
    }
    count
}

/// Calculates the expected byte size of the image data for the given size and
/// number of mipmaps, per layer.
fn expected_byte_size(size: u32, mipmaps: u32) -> usize {
    let mut bytes = 0;

    let mut s = size;
    for _ in 0 ..= mipmaps {
        bytes += s * s * 4;
        s /= 2;
    }

    bytes as usize
}

/// Generates mipmaps for the given image bytes and append them to the end of
/// the byte vector.
fn generate_mipmaps(bytes: &mut Vec<u8>, mut size: u32, mipmaps: u32) {
    let mut offset = 0;
    for _ in 0 .. mipmaps {
        size /= 2;
        let mut new_bytes = Vec::new();

        for y in 0 .. size {
            for x in 0 .. size {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                let mut a = 0;

                for j in 0 .. 2 {
                    for i in 0 .. 2 {
                        let index = ((y * 2 + j) * size * 2 + x * 2 + i) as usize * 4;
                        r += bytes[offset + index] as u32;
                        g += bytes[offset + index + 1] as u32;
                        b += bytes[offset + index + 2] as u32;
                        a += bytes[offset + index + 3] as u32;
                    }
                }

                r /= 4;
                g /= 4;
                b /= 4;
                a /= 4;

                new_bytes.push(r as u8);
                new_bytes.push(g as u8);
                new_bytes.push(b as u8);
                new_bytes.push(a as u8);
            }
        }

        bytes.extend_from_slice(&new_bytes);
        offset += new_bytes.len();
    }
}

/// TileData is a structure that contains the binary data of an image that has
/// been processes into a viable tile format for the tileset.
struct TileData {
    /// The binary data of the tile, including mipmaps.
    binary: Vec<u8>,

    /// The size of the tile in pixels.
    size: u32,

    /// The number of mipmaps in the tile.
    mipmaps: u32,
}
