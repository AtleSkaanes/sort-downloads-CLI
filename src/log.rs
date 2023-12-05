use std::str::FromStr;

use clap::Parser;
use colored::Colorize;

#[derive(Clone, Copy, Parser, Debug, PartialEq)]
pub enum LogLevel {
    Error,
    Info,
    Warning,
    Debug,
}

impl LogLevel {
    pub fn variants() -> Vec<&'static str> {
        vec!["error", "log", "warning", "debug"]
    }
}

impl FromStr for LogLevel {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, std::io::Error> {
        match s.to_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "debug" => Ok(Self::Debug),
            &_ => Ok(Self::Error),
        }
    }
}

static mut LOG_LEVEL: LogLevel = LogLevel::Error;

pub fn set_log_level(level: LogLevel) {
    unsafe {
        LOG_LEVEL = level;
    }
}

pub fn get_log_level() -> LogLevel {
    unsafe { return LOG_LEVEL }
}

pub fn error(msg: &str) {
    if get_log_level() as u8 >= LogLevel::Error as u8 {
        println!("{} {}", "[ERROR]:".red().bold(), msg.red().bold());
    }
}
pub fn info(msg: &str) {
    if get_log_level() as u8 >= LogLevel::Info as u8 {
        println!("{} {}", "[INFO]:", msg);
    }
}
pub fn warn(msg: &str) {
    if get_log_level() as u8 >= LogLevel::Warning as u8 {
        println!("{} {}", "[WARNING]:".yellow(), msg.yellow());
    }
}
pub fn debug(msg: &str) {
    if get_log_level() as u8 >= LogLevel::Debug as u8 {
        println!("{} {}", "[DEBUG]:".blue(), msg.blue());
    }
}
pub fn get_input(prompt: &str) -> String {
    let mut line = String::new();
    println!("\n{}", prompt.blue().italic());
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}
pub fn get_yn(prompt: &str) -> Option<bool> {
    let input = get_input(format!("{} [Y/N]", prompt).as_str())
        .trim()
        .to_lowercase();
    if input == "y" || input == "yes" {
        return Some(true);
    } else if input == "n" || input == "no" {
        return Some(false);
    } else {
        return None;
    }
}
