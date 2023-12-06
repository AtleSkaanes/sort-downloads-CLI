use std::{collections::HashMap, ffi::OsStr, fs, path::PathBuf};

use byte_unit::Byte;
use filesize::file_real_size;

use crate::{command, config, data, log};

pub fn start_sort(cmd: command::CommandOpts) {
    log::info("Starting sort...");

    let mut sort_count = 0;
    let mut del_count = 0;

    let mut sort_size = 0;
    let mut del_size = 0;

    for file in data::get_dir_files(cmd.dir) {
        let filename = file
            .file_name()
            .unwrap_or(&OsStr::new("[NO NAME]"))
            .to_str()
            .unwrap();

        let filesize = file_real_size(file.clone()).unwrap_or(0);

        // if file is matched by keep, don't do anything to it :)
        if file_matches_vec(file.clone(), &cmd.keep_prefixes)
            || file_matches_vec(file.clone(), &cmd.keep_extensions)
        {
            log::warn(&format!(
                "File \"{}\" is set as protected, so can't be moved",
                file.file_name()
                    .unwrap_or(&OsStr::new("[NO NAME]"))
                    .to_str()
                    .unwrap()
            ));
            continue;
        }

        // Remove
        if !cmd.no_del {
            if file_matches_vec(file.clone(), &cmd.del_prefixes)
                || file_matches_vec(file.clone(), &cmd.del_extensions)
            {
                if cmd.safe_mode {
                    let mut got_answer: bool = false;
                    while !got_answer {
                        let answer =
                            log::get_yn(&format!("Do you want to delete file: {}", filename));
                        match answer {
                            Some(ans) => {
                                if !ans {
                                    log::info(&format!("Aborting deletion of \"{}\"", filename));
                                    got_answer = true;
                                    continue;
                                } else {
                                    log::info(&format!("Continuing deletion of \"{}\"", filename));
                                    got_answer = true;
                                }
                            }
                            None => log::error("Not valid answer"),
                        }
                    }
                }
                match fs::remove_file(file.clone()) {
                    Ok(()) => {
                        log::info(&format!("Succesfully removed file \"{}\"", filename));
                        del_count += 1;
                        del_size += filesize;
                        continue;
                    }
                    Err(e) => {
                        log::error(&format!(
                            "While removing file \"{}\" : {}",
                            filename,
                            e.to_string()
                        ));
                        continue;
                    }
                }
            }
        }

        // Sort
        if !cmd.no_sort {
            if let Some(locations) = file_matches_hashmap(file.clone(), &cmd.sort_table) {
                let mut dir = locations[0].clone();
                if locations.len() > 1 {
                    let mut got_answer = false;
                    println!("\n {} has multiple end-locations, pick one.", filename);
                    for (i, dir) in locations.iter().enumerate() {
                        println!("{} - {}", i, dir.to_str().unwrap_or("[NO NAME]"));
                    }
                    while !got_answer {
                        let answer =
                            log::get_input(&format!("Select location [0-{}]", locations.len() - 1));
                        if answer.parse::<usize>().unwrap_or(locations.len()) < locations.len() {
                            got_answer = true;
                            dir = locations[answer.parse::<usize>().unwrap()].clone();
                        } else {
                            log::error("Not valid answer");
                        }
                    }
                }
                // If given end-location exists
                if !dir.exists() {
                    log::error(&format!(
                        "Location doesn't exist: {}, trying to create...",
                        dir.to_str().unwrap()
                    ));
                    match fs::create_dir_all(dir.clone()) {
                        Ok(()) => log::info("Successfully created location!"),
                        Err(e) => {
                            log::error(&format!("While creating location : {}", e));
                            continue;
                        }
                    }
                }

                // Move file to new location
                match fs::rename(file.clone(), dir.join(file.clone().file_name().unwrap())) {
                    Ok(()) => {
                        log::info(&format!(
                            "Moved {} to {}",
                            file.clone().file_name().unwrap().to_str().unwrap(),
                            dir.to_str().unwrap()
                        ));
                        sort_count += 1;
                        sort_size += filesize;
                        sort_size += filesize;
                    }
                    Err(e) => log::error(&format!(
                        "While moving {} to {}: {}",
                        file.clone().file_name().unwrap().to_str().unwrap(),
                        dir.to_str().unwrap(),
                        e.to_string()
                    )),
                }
            }
        } else {
            log::debug("Flag \"dont-sort\" is set");
        }
    }

    log::info("Completed sorting\n");
    if !cmd.no_sort {
        log::info(&format!(
            "Sorted {} file(s) - {:#}",
            sort_count,
            Byte::from_u64(sort_size),
        ));
    }
    if !cmd.no_del {
        log::info(&format!(
            "Removed {} file(s) - {:#}",
            del_count,
            Byte::from_u64(del_size),
        ));
    }
}

fn file_matches_vec(file: PathBuf, check: &Vec<String>) -> bool {
    let filename = file.file_name().unwrap().to_str().unwrap();
    let fileext = file
        .extension()
        .unwrap_or(&OsStr::new("[NO EXT]"))
        .to_str()
        .unwrap();
    if check.contains(&(".".to_string() + fileext)) {
        return true;
    }
    for prefix in check {
        if filename.starts_with(prefix) {
            return true;
        }
    }
    false
}

fn file_matches_hashmap(
    file: PathBuf,
    check: &HashMap<String, Vec<PathBuf>>,
) -> Option<Vec<PathBuf>> {
    let filename = file.file_name().unwrap().to_str().unwrap();
    let fileext = file
        .extension()
        .unwrap_or(&OsStr::new("[NO EXT]"))
        .to_str()
        .unwrap();
    for prefix in check.keys() {
        if filename.starts_with(prefix) {
            return check.get(prefix).cloned();
        }
    }
    if let Some(dir) = check.get(&(".".to_string() + fileext)) {
        return Some(dir.to_vec());
    }
    None
}
