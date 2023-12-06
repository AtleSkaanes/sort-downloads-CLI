use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

const FILE_NAME: &str = "configure.ron";

use directories::UserDirs;
use serde::{Deserialize, Serialize};

use crate::data;

pub fn get_config() -> ConfigOpts {
    match open_config() {
        Ok(content) => {
            return ron::from_str::<ConfigOpts>(&content)
                .expect(&format!("\"{}\" should be in valid ron format", FILE_NAME))
        }
        Err(_) => {
            let _ = create_config();
            return get_default_config();
        }
    }
}

fn get_default_config() -> ConfigOpts {
    let binding = UserDirs::new();
    let dir = match &binding {
        Some(user_dirs) => user_dirs.picture_dir().unwrap(),
        None => &Path::new("./"),
    };
    ConfigOpts {
        sorting_locations: HashMap::from([(".png".to_string(), vec![dir.to_path_buf()])]),
        white_list: vec![],
        safe_mode: true,
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigOpts {
    // Use "." as a prefix to signify file extension, otherwise it is interpreted as a file-name prefix
    // example:
    //         ".png" matches "image.png"
    //         "d-" matches "d-text.txt"
    pub sorting_locations: HashMap<String, Vec<PathBuf>>,
    pub white_list: Vec<String>,
    // Safe mode asks before deleting any file
    pub safe_mode: bool,
}

pub fn open_config() -> std::io::Result<String> {
    let dir = data::get_data_path().expect("Should have a data path");
    fs::read_to_string(dir.join(FILE_NAME))
}

pub fn create_config() -> std::io::Result<()> {
    let dir = &data::get_data_path().expect("Should have a data path");

    fs::create_dir_all(dir)?;

    let mut f = fs::File::create(dir.join(FILE_NAME))?;

    let contents =
        ron::ser::to_string_pretty(&get_default_config(), ron::ser::PrettyConfig::default())
            .expect("Default sorting config should be valid .ron format");

    f.write_all(contents.as_bytes())?;

    Ok(())
}
