use std::path::PathBuf;
use regex::{Regex};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub path: PathBuf,
    pub name: String,
    pub ext: String,
}

pub fn extract(path: PathBuf) -> FileData {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<name>[^:\\/]*?)(?:\.(?P<ext>[^ :\\/.]*))?$"
        ).unwrap();
    }
    let file = RE.captures(&path.to_str().unwrap()).unwrap();

    FileData {
        path: path.clone(),
        name: String::from(&file["name"]),
        ext: String::from(&file["ext"]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_file_name() {
        let path = PathBuf::from(r"C:\windows\filename.ext");
        let file = extract(path.clone());
        assert_eq!(file.path, path);
        assert_eq!(file.name, "filename");
    }

    #[test]
    fn should_extract_file_extension() {
        let path = PathBuf::from(r"./folder/something.ext");
        let file = extract(path.clone());
        assert_eq!(file.path, path);
        assert_eq!(file.ext, "ext");
    }
}
