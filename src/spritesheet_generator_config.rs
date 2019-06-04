use std::default::Default;

#[derive(Clone, Debug)]
pub struct SpritesheetGeneratorConfig {
    pub max_width: u32,
    pub max_height: u32,
    pub border_padding: u32,
    pub input_folder: String,
    pub output_folder: String,
    pub output_file_name: String,
    pub allow_rotation: bool
}

impl Default for SpritesheetGeneratorConfig {
    fn default() -> SpritesheetGeneratorConfig {
        SpritesheetGeneratorConfig {
            max_width: 1024,
            max_height: 1024,
            border_padding: 2,
            input_folder: "assets/".to_string(),
            output_folder: "resources/".to_string(),
            output_file_name: "spritesheet".to_string(),
            allow_rotation: false
        }
    }
}
