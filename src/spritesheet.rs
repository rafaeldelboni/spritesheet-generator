use std::collections::HashMap;

use texture_packer;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Screen {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub screen: Screen,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spritesheet {
    pub frames: HashMap<String, Frame>,
}

pub fn to_atlas(
    frames: &HashMap<String, texture_packer::Frame>,
    image_width: u32,
    image_height: u32,
) -> Spritesheet {
    let frames_map = frames
        .iter()
        .map(|(name, frame)| (
                name.clone(),
                Frame {
                    x: frame.frame.x,
                    y: frame.frame.y,
                    w: frame.frame.w,
                    h: frame.frame.h,
                    screen: Screen {
                        x: 1. / (image_width as f32 / frame.frame.x as f32),
                        y: 1. / (image_height as f32 / frame.frame.y as f32),
                        w: 1. / (image_width as f32 / frame.frame.w as f32),
                        h: 1. / (image_height as f32 / frame.frame.h as f32),
                    }
                }
            )
        )
        .collect();

    return Spritesheet { frames: frames_map };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_atlas() {
        let mut converted_frames: HashMap<String, texture_packer::Frame> = HashMap::new();
        converted_frames.insert(
            "test1".to_string(),
            texture_packer::Frame {
                key: "test1".to_string(),
                frame: texture_packer::Rect{ x: 0, y: 0, w: 10, h: 50},
                source: texture_packer::Rect{ x: 0, y: 0, w: 10, h: 50},
                rotated: false,
                trimmed: false,
            }
        );
        let atlas = to_atlas(&converted_frames, 100, 100);

        let mut created_frames: HashMap<String, Frame> = HashMap::new();
        created_frames.insert(
            "test1".to_string(),
            Frame {
                x: 0, y: 0, w: 10, h: 50, screen: Screen { x: 0., y: 0., w: 0.1, h: 0.5 }
            }
        );
        created_frames.insert(
            "test2".to_string(),
            Frame {
                x: 1, y: 1, w: 1, h: 1, screen: Screen { x: 0., y: 0., w: 0., h: 0. }
            }
        );

        let converted = atlas.frames.get("test1").unwrap();
        let created = created_frames.get("test1").unwrap();

        assert_eq!(converted.x, created.x);
        assert_eq!(converted.y, created.y);
        assert_eq!(converted.w, created.w);
        assert_eq!(converted.h, created.h);
        assert_eq!(converted.screen.x, created.screen.x);
        assert_eq!(converted.screen.y, created.screen.y);
        assert_eq!(converted.screen.w, created.screen.w);
        assert_eq!(converted.screen.h, created.screen.h);
    }
}
