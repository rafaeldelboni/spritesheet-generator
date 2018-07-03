use std::fs::File;

use serde_json;
use image;
use texture_packer::{TexturePacker, TexturePackerConfig};
use texture_packer::exporter::ImageExporter;

use file_texture;
use spritesheet;
use spritesheet_generator_config;

pub fn generate(config: spritesheet_generator_config::SpritesheetGeneratorConfig) {
    // Initial setup
    let input_folder = config.input_folder;
    let output_folder = config.output_folder;
    let output_file_name = config.output_file_name;

    // Perform texture packing
    let config = TexturePackerConfig {
        max_width: config.max_width,
        max_height: config.max_height,
        border_padding: config.border_padding,
        allow_rotation: false,
        texture_outlines: false,
        ..Default::default()
    };
    let mut packer = TexturePacker::new_skyline(config);
    for file_textures in file_texture::find_all(input_folder) {
        packer.pack_own(file_textures.file.name, file_textures.texture);
    }

    // Save Json
    let atlas = spritesheet::to_atlas(packer.get_frames());
    let json_path = format!("{}{}.json", output_folder, output_file_name);
    let json_file = File::create(json_path).unwrap();
    serde_json::to_writer_pretty(json_file, &atlas).unwrap();

    // Save Image
    let exporter = ImageExporter::export(&packer).unwrap();
    let image_path = format!("{}{}.png", output_folder, output_file_name);
    let mut image_file = File::create(image_path).unwrap();
    exporter.write_to(&mut image_file, image::PNG).unwrap();
}
