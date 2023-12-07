use clap::Parser;
use std::{collections::HashMap, path::PathBuf};

use crate::{config, data, log};

#[macro_export]
macro_rules! clap_enum_variants {
    ($e: ty) => {{
        use clap::builder::TypedValueParser;
        clap::builder::PossibleValuesParser::new(<$e>::variants().iter())
            .map(|s| s.parse::<$e>().unwrap())
    }};
}

#[derive(Default)]
pub struct CommandOpts {
    pub dir: PathBuf,
    pub keep_extensions: Vec<String>,
    pub del_extensions: Vec<String>,
    pub keep_prefixes: Vec<String>,
    pub del_prefixes: Vec<String>,
    pub safe_mode: bool,
    pub remove_unknown: bool,
    pub no_del: bool,
    pub no_sort: bool,
    pub sort_table: HashMap<String, Vec<PathBuf>>,
}

/// A program that sorts or deletes files in your downloads folder, or a custom folder
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Custom directory to sort, defaults to downloads directory
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    /// File patterns to keep (prefix with . for file extension, don't for file prefixes)
    #[arg(short, long, num_args = 1..)]
    pub keep: Option<Vec<String>>,

    /// File patterns to remove (prefix with . for file extension, don't for file prefixes)
    #[arg(short, long, num_args = 1..)]
    pub remove: Option<Vec<String>>,

    /// If set to true, don't sort, only keep or remove files
    #[arg(long)]
    pub dont_sort: bool,

    /// If set to true, don't delete, only sort relevant files
    #[arg(long)]
    pub dont_remove: bool,

    /// Remove all files not specified in --keep (or in config.ron if set to sort)
    #[arg(long)]
    pub remove_unknown: bool,

    #[arg(short, long, ignore_case = true, value_parser = clap_enum_variants!(log::LogLevel))]
    pub log_level: Option<log::LogLevel>,
}

pub fn fill_command_opts(args: CliArgs) -> CommandOpts {
    let mut cmd = CommandOpts::default();

    let cfg = config::get_config();

    cmd.dir = args.directory.unwrap_or_else(|| {
        data::get_downloads_path().expect("Should have a valid downloads directory")
    });

    cmd.keep_prefixes = vec![];
    cmd.keep_extensions = vec![];

    cmd.del_prefixes = vec![];
    cmd.del_extensions = vec![];

    for item in args.keep.unwrap_or(vec![]) {
        if item.starts_with(".") {
            cmd.keep_extensions.push(item);
        } else {
            cmd.keep_prefixes.push(item);
        }
    }
    for item in cfg.keep {
        if item.starts_with(".") {
            cmd.keep_extensions.push(item);
        } else {
            cmd.keep_prefixes.push(item);
        }
    }
    for item in cfg.remove {
        if item.starts_with(".") {
            cmd.del_extensions.push(item);
        } else {
            cmd.del_prefixes.push(item);
        }
    }
    for item in args.remove.unwrap_or(vec![]) {
        if item.starts_with(".") {
            cmd.del_extensions.push(item);
        } else {
            cmd.del_prefixes.push(item);
        }
    }

    cmd.safe_mode = cfg.safe_mode;

    cmd.remove_unknown = args.remove_unknown;

    cmd.no_del = args.dont_remove;
    cmd.no_sort = args.dont_sort;

    cmd.sort_table = cfg.sorting_locations;
    cmd
}
