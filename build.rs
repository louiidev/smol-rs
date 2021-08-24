use asefile::AsepriteFile;
use hashbrown::HashMap;
use image::{DynamicImage, ImageBuffer, Rgba};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io::Write, path::PathBuf};
use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, TexturePacker, TexturePackerConfig,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AseTextureData {
    pub width: u32,
    pub height: u32,
    pub basename: String,
    pub frame: u32,
    pub x: u32,
    pub y: u32,
}

struct AseFile {
    path: PathBuf,
    name: String,
}

pub struct AsespritePacker {
    pub image: DynamicImage,
    pub packed_texture_data: HashMap<String, AseTextureData>,
}

impl AsespritePacker {
    pub fn new() {
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

        let mut packer: TexturePacker<ImageBuffer<Rgba<u8>, Vec<u8>>, String> =
            TexturePacker::new_skyline(texture_packer_config);

        let mut packed_texture_data: HashMap<String, AseTextureData> = HashMap::default();

        let path = Path::new("./ase_files");
        let ase_files: Vec<AseFile> = {
            let paths = fs::read_dir(path).unwrap();
            paths
                .map(|p| {
                    let path_buff = p.unwrap();

                    let name = path_buff
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    AseFile {
                        path: path_buff.path().as_path().to_owned(),
                        name,
                    }
                })
                .collect()
        };

        for file in ase_files {
            // let path = env::current_dir().unwrap();
            let ase_file = AsepriteFile::read_file(file.path.as_path());

            match ase_file {
                Err(e) => panic!(e),
                Ok(ase) => {
                    for frame_num in 0..ase.num_frames() {
                        let frame = ase.frame(frame_num);
                        let key: String = if ase.num_frames() > 1 {
                            format!("{}_{}", file.name.to_string(), frame_num)
                        } else {
                            file.name.to_string()
                        };
                        let texture = image::imageops::flip_vertical(&frame.image());
                        let _res = packer.pack_own(key.clone(), texture);
                        let frame_data = packer.get_frame(&key).unwrap();
                        let source = frame_data.frame;
                        packed_texture_data.insert(
                            key.clone(),
                            AseTextureData {
                                width: source.w,
                                height: source.h,
                                x: source.x,
                                y: source.y,
                                basename: file.name.to_string(),
                                frame: frame_num,
                            },
                        );
                    }
                }
            }
        }

        let image = ImageExporter::export(&packer).unwrap();
        let mut file = std::fs::File::create(Path::new("assets/atlas.png")).unwrap();
        image.write_to(&mut file, image::ImageFormat::Png).unwrap();

        let mut file = std::fs::File::create(Path::new("assets/atlas.ron")).unwrap();
        let str = to_string_pretty(&packed_texture_data, PrettyConfig::default()).unwrap();
        let _ = file.write_all(str.as_bytes());
    }
}

fn main() {
    AsespritePacker::new();
}
