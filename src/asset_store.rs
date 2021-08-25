use std::hash::Hash;

use crate::core::App;
use crate::errors::SmolError;
use crate::renderer::Texture;
use crate::{gfx::GfxContext, math::Vec2};
use hashbrown::HashMap;
use image::io::Reader;
use image::GenericImageView;
use ron::de::from_str;
use serde::Deserialize;
use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, TexturePacker, TexturePackerConfig,
};

pub struct TextureAsset<'a> {
    bytes: &'a [u8],
    name: String,
}

#[derive(Deserialize)]
pub struct PackedTexture {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

impl<'a> TextureAsset<'a> {
    pub fn new(bytes: &'a [u8], name: String) -> Self {
        Self { bytes, name }
    }
}

#[derive(Default)]
pub struct AssetStore {
    textures: HashMap<String, Texture>,
}

impl AssetStore {
    pub fn new(textures: HashMap<String, Texture>) -> Self {
        Self { textures }
    }

    pub fn insert_texture(&mut self, name: &str, texture: &Texture) -> Result<(), SmolError> {
        if self.textures.contains_key(name) {
            return Err(SmolError::new(
                "Asset store already has a texture with this name",
            ));
        }
        self.textures.insert(name.to_owned(), *texture);

        Ok(())
    }

    pub fn load_texture<'a>(
        &mut self,
        name: String,
        texture_asset: TextureAsset,
    ) -> Result<Texture, SmolError> {
        let (width, height, id) = GfxContext::generate_texture(texture_asset.bytes);
        let size = Vec2::new(width as f32, height as f32);
        let texture = Texture::new(id, size, Vec2::default(), size);
        self.insert_texture(&name, &texture)?;
        Ok(texture)
    }

    pub fn load_atlas_texture<'a>(
        &mut self,
        atlas_bytes: &'a [u8],
        atlas_details: &str,
    ) -> Result<HashMap<String, Texture>, SmolError> {
        let (width, height, id) = GfxContext::generate_texture(atlas_bytes);
        let map = from_str::<HashMap<String, PackedTexture>>(&atlas_details)?;
        let mut textures: HashMap<String, Texture> = HashMap::default();
        for (name, packed_tex) in map.into_iter() {
            let texture = Texture::new(
                id,
                Vec2::new(packed_tex.width as f32, packed_tex.height as f32),
                Vec2::new(packed_tex.x as f32, packed_tex.y as f32),
                Vec2::new(width as f32, height as f32),
            );
            self.insert_texture(&name, &texture)?;
            textures.insert(name, texture);
        }

        Ok(textures)
    }

    pub fn load_into_texture_atlas<'a>(
        &mut self,
        texture_assets: Vec<&'a TextureAsset>,
    ) -> Result<HashMap<String, Texture>, SmolError> {
        let texture_packer_config = TexturePackerConfig {
            max_width: std::u32::MAX,
            max_height: std::u32::MAX,
            allow_rotation: false,
            texture_outlines: false,
            border_padding: 0,
            texture_padding: 0,
            trim: false,
            ..Default::default()
        };

        let mut textures: HashMap<String, Texture> = HashMap::default();

        let mut packer = TexturePacker::new_skyline(texture_packer_config);

        for asset in texture_assets {
            let image = ImageImporter::import_from_memory(asset.bytes).unwrap();
            let _ = packer.pack_own(asset.name.clone(), image);
        }
        let image = ImageExporter::export(&packer).unwrap();
        let atlas_width = image.width();
        let atlas_height = image.height();
        let (_, _, id) = GfxContext::generate_texture(image.as_bytes());

        for (name, frame) in packer.get_frames() {
            let pos = Vec2::new(frame.frame.x as f32, frame.frame.y as f32);
            let size = Vec2::new(frame.frame.w as f32, frame.frame.h as f32);
            let texture = Texture::new(
                id,
                size,
                pos,
                Vec2::new(atlas_width as f32, atlas_height as f32),
            );
            self.insert_texture(name, &texture)?;
            textures.insert(name.to_string(), texture);
        }

        Ok(textures)
    }
}
