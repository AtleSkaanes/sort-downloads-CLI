use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use directories::{ProjectDirs, UserDirs};

use crate::log;

pub fn get_downloads_path() -> Option<PathBuf> {
    let binding = UserDirs::new();
    match &binding {
        Some(user_dirs) => Some(user_dirs.download_dir().unwrap().to_path_buf()),
        None => None,
    }
}

pub fn get_data_path() -> Option<PathBuf> {
    let binding = ProjectDirs::from("com", "atle", "sort_downloads");
    match &binding {
        Some(project_dirs) => Some(project_dirs.config_local_dir().to_path_buf()),
        None => None,
    }
}

pub fn get_dir_content_raw(path: PathBuf) -> Vec<std::io::Result<PathBuf>> {
    //let contents = fs::read_dir(path).expect("Should be able to read assigned directory");
    let contents_result = match fs::read_dir(path.clone()) {
        Ok(path) => Ok(path),
        Err(e) => {
            log::error(&format!(
                "While reading \"{}\" : {}",
                path.clone().to_str().unwrap_or("[NO NAME]"),
                e.to_string()
            ));
            Err(())
        }
    };
    if let Ok(contents) = contents_result {
        return contents
            .map(
                |entry_result: std::io::Result<DirEntry>| -> std::io::Result<PathBuf> {
                    entry_result.map(|entry: DirEntry| -> PathBuf { entry.path() })
                },
            )
            .collect();
    };
    vec![]
}

pub fn get_dir_files(path: PathBuf) -> Vec<PathBuf> {
    let contents = get_dir_content_raw(path);
    contents
        .iter()
        .filter_map(|item| match item {
            Ok(file) => {
                if fs::metadata(file.clone())
                    .expect(&format!(
                        "{} should have valid metadata",
                        file.to_str().unwrap()
                    ))
                    .is_file()
                {
                    Some(file.clone())
                    // match file.extension() {
                    //     Some(_) => Some(file.clone()),
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect::<Vec<PathBuf>>()
}
