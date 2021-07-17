use asefile::{AsepriteFile };
use std::{fs, io::Write, path::PathBuf};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::{collections::HashMap,  path::Path};
use texture_packer::{exporter::ImageExporter, TexturePacker, TexturePackerConfig };
use serde::{Serialize, Deserialize};
use ron::{ser::{PrettyConfig, to_string_pretty}};

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
    name: String
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
    
        let mut packer: TexturePacker<ImageBuffer<Rgba<u8>, Vec<u8>>, String> = TexturePacker::new_skyline(texture_packer_config);
    
        let mut packed_texture_data: HashMap<String, AseTextureData> = HashMap::default();

        let path = Path::new("./ase_files");
        let ase_files: Vec<AseFile> = {
            println!("{}", path.display());
            let paths = fs::read_dir(path).unwrap();
            paths.map(|p| {
                
                let path_buff = p.unwrap();
                
                let name =  path_buff.path().file_stem().unwrap().to_str().unwrap().to_string();
                println!("{}", name);
                AseFile {
                    path: path_buff.path().as_path().to_owned(),
                    name
                }
            }).collect()
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
                        let texture =   image::imageops::flip_vertical(&frame.image());
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

        fs::read_dir(Path::new("./assets")).unwrap().for_each(|p| {
                
            let path_buff = p.unwrap();
            
            let name =  path_buff.path().file_stem().unwrap().to_str().unwrap().to_string();
            println!("{}", name);
        
        });

        let mut file = std::fs::File::create(Path::new("assets/atlas.png")).unwrap();
        image
            .write_to(&mut file, image::ImageFormat::Png)
            .unwrap();
        println!("Output texture stored in {:?}", file);


        let mut file = std::fs::File::create(Path::new("assets/atlas.ron")).unwrap();
        let str = to_string_pretty(&packed_texture_data, PrettyConfig::default()).unwrap();
        let _ = file.write_all(str.as_bytes());
        println!("Output texture stored in {:?}", file);

    }
  
}


 
fn main() {
    AsespritePacker::new();
}