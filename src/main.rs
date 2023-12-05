mod command;
mod config;
mod data;
mod log;
mod sort;

use clap::Parser;

use crate::log::LogLevel;

fn main() {
    let args = command::CliArgs::parse();
    match args.log_level {
        Some(level) => log::set_log_level(level),
        None => log::set_log_level(LogLevel::Info),
    }
    if args.dont_sort {
        log::debug("Flag \"dont-sort\" is set. No sorting will happen");
    }
    if args.dont_remove {
        log::debug("Flag \"dont-remove\" is set. No removing will happen");
    }
    if args.dont_remove && args.dont_sort {
        log::error("Both \"dont-sort\" and \"dont-remove\" are set. Nothing can happen");
        return;
    }
    let cmd = command::fill_command_opts(args);

    sort::start_sort(cmd);
}
