use crate::gfx::GfxContext;
use crate::math::Vector2;
use crate::renderer::{Font, Texture};
use crate::{errors::SmolError, App};
use hashbrown::HashMap;
use image::GenericImageView;
use nalgebra::Vector;
use serde::Deserialize;
use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, TexturePacker, 
    TexturePackerConfig,
};

use glyph_brush::{ab_glyph::*, *};

#[macro_export]
macro_rules! import_file {
    ($path:expr) => {
        (
            &$path[$path.match_indices("/").last().unwrap_or((0, "")).0 + 1
                ..$path
                    .match_indices(".")
                    .last()
                    .unwrap_or(($path.len(), ""))
                    .0],
            &$path[$path.match_indices(".").last().unwrap_or((0, "")).0 + 1..$path.len()],
            include_bytes!($path),
        )
    };
}

pub type Asset<'a> = (&'a str, &'a str, &'a [u8]);

#[derive(Deserialize, Debug)]
pub struct PackedTexture {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

#[derive(Default)]
pub struct AssetStore {
    textures: HashMap<String, Texture>,
    fonts: HashMap<String, Font>,
}

impl App {
    pub fn load_font<'a>(&mut self, asset: Asset<'a>) -> Result<Font, SmolError> {
        let b = asset.2.to_vec();
        let font = FontArc::try_from_vec(b)?;
        let glyph_brush: GlyphBrush<[f32; 13]> = GlyphBrushBuilder::using_font(font).build();
        let dimensions = glyph_brush.texture_dimensions();
        let texture_id = GfxContext::generate_font_texture(dimensions);
        let size = Vector::from([dimensions.0 as f32, dimensions.1 as f32]);
        let texture = Texture::new(texture_id, size, Vector2::default(), size);
        let font = Font {
            id: self.renderer.glyph_brushs.len(),
            texture,
        };
        self.insert_font(asset.0, font)?;
        self.renderer.glyph_brushs.insert(font, glyph_brush);

        Ok(font)
    }

    pub fn insert_font(&mut self, name: &str, font: Font) -> Result<(), SmolError> {
        if self.asset_store.textures.contains_key(name) {
            return Err(SmolError::new(
                "Asset store already has a texture with this name",
            ));
        }
        self.asset_store.fonts.insert(name.to_owned(), font);

        Ok(())
    }

    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.asset_store.fonts.get(name)
    }

    pub fn insert_texture(&mut self, name: &str, texture: Texture) -> Result<(), SmolError> {
        if self.asset_store.textures.contains_key(name) {
            return Err(SmolError::new(
                "Asset store already has a texture with this name",
            ));
        }
        self.asset_store.textures.insert(name.to_owned(), texture);

        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<Texture> {
        if let Some(texture) = self.asset_store.textures.get(name) {
            Some(*texture)
        } else {
            None
        }
    }

    pub fn load_texture<'a>(&mut self, asset: Asset<'a>) -> Result<Texture, SmolError> {
        let (width, height, id) = GfxContext::generate_texture(asset.2, "");
        let size = Vector::from([width as f32, height as f32]);
        let texture = Texture::new(id, size, Vector2::default(), size);
        self.insert_texture(&asset.0, texture)?;
        Ok(texture)
    }

    pub fn load_single_aseprite_texture<'a>(
        &mut self,
        asset: Asset<'a>,
    ) -> Result<Texture, SmolError> {
        let (width, height, id) = GfxContext::generate_texture(asset.2, "aseprite");
        let size = Vector::from([width as f32, height as f32]);
        let texture = Texture::new(id, size, Vector2::default(), size);
        self.insert_texture(&asset.0, texture)?;
        Ok(texture)
    }

    pub fn load_into_texture_atlas<'a>(
        &mut self,
        texture_assets: Vec<&'a Asset>,
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
            let image = ImageImporter::import_from_memory(asset.2).unwrap();
            let _ = packer.pack_own(asset.0.clone(), image);
        }
        let image = ImageExporter::export(&packer).unwrap();
        let atlas_width = image.width();
        let atlas_height = image.height();
        let (_, _, id) = GfxContext::generate_texture(image.as_bytes(), "");

        for (name, frame) in packer.get_frames() {
            let pos = Vector::from([frame.frame.x as f32, frame.frame.y as f32]);
            let size = Vector::from([frame.frame.w as f32, frame.frame.h as f32]);
            let texture = Texture::new(
                id,
                size,
                pos,
                Vector::from([atlas_width as f32, atlas_height as f32]),
            );
            self.insert_texture(name, texture)?;
            textures.insert(name.to_string(), texture);
        }

        Ok(textures)
    }
}
