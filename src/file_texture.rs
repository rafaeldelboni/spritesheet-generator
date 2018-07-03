use file_data;
use image;

use std::fs;
use texture_packer::importer::ImageImporter;

pub struct FileTexture {
    pub file: file_data::FileData,
    pub texture: image::DynamicImage,
}

pub fn find_all (folder: String) -> Vec<FileTexture> {
    let mut file_list = Vec::new();
    if let Ok(entries) = fs::read_dir(&folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let file_name = entry.file_name().into_string().unwrap();
                        let child_folder = format!("{}{}/", &folder, file_name);
                        file_list.extend(find_all(child_folder));
                    } else {
                        let file = file_data::extract(entry.path());
                        let texture = ImageImporter::import_from_file(&entry.path())
                            .unwrap();
                        file_list.push(FileTexture {file, texture});
                    }
                }
            }
        }
    }
    file_list
}
