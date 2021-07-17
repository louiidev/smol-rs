use std::{collections::HashMap};
use serde::{ Deserialize};
use ron::{de::{from_str}};
use crate::{core::load_texture_from_bytes, math::{Vector2Int}, render::{PartialTexture, Texture}};


#[derive(Debug, Deserialize)]
pub struct AseTextureData {
    pub width: u32,
    pub height: u32,
    pub basename: String,
    pub frame: u32,
    pub x: u32,
    pub y: u32,
}


pub struct TexturePacker {
    data: HashMap<String, AseTextureData>,
    texture: Texture
}

impl TexturePacker {
    pub fn new() -> Self {
        let texture = load_texture_from_bytes(include_bytes!("../assets/atlas.png"));
        let data = from_str::<HashMap<String, AseTextureData>>(include_str!("../assets/atlas.ron")).unwrap();

        TexturePacker {
            texture,
            data
        }
    }

    pub fn get_texture(&self, name: &str) -> PartialTexture {
        let data = self.data.get(name).unwrap();
        self.texture.create_partial(data.width, data.height, Vector2Int::new(data.x as i32 , data.y as i32))
    }
}