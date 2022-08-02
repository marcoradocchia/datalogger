// datalogger: Humidity & Temperature CLI datalogger for DHT22 sensor on Raspberry Pi.
// Copyright (C) 2022 Marco Radocchia
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see https://www.gnu.org/licenses/.

use clap::ArgAction::{Set, SetTrue};
pub use clap::Parser;
use directories::BaseDirs;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Expands `~` in `path` to absolute HOME path.
pub fn expand_home(path: &Path) -> PathBuf {
    if let Ok(path) = path.strip_prefix("~") {
        // Generate the absolute path for HOME.
        BaseDirs::new()
            .expect("unable to find home directory")
            .home_dir()
            .to_path_buf()
            .join(path)
    } else {
        path.to_path_buf()
    }
}

/// Custom parser for `directory` field.
/// Automatically expands ~ and creates directory if doesn't exist.
pub fn parse_directory(directory: &str) -> Result<PathBuf, String> {
    let path = expand_home(&PathBuf::from(directory));
    if !path.is_dir() && fs::create_dir_all(&path).is_err() {
        return Err(String::from("unable to create specified directory"));
    }

    Ok(path)
}

/// Parse interval value as `u32` grater than 2.
fn parse_interval(interval: &str) -> Result<u64, String> {
    match interval.parse::<u64>() {
        Ok(val) => {
            if val < 2 {
                Err("interval must be >=2".to_string())
            } else {
                Ok(val)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Humidity & Temperature CLI datalogger for DHT22 sensor on Raspberry Pi.
#[derive(Parser, Debug)]
#[clap(
    author = "Marco Radocchia <marco.radocchia@outlook.com>",
    version,
    about,
    long_about = None
)]
pub struct Args {
    /// GPIO pin for DHT22 data connection.
    #[clap(short, long, action = Set)]
    pub pin: u8,

    /// Interval in seconds between consecutive measures.
    #[clap(short, long, default_value_t = 120, value_parser = parse_interval)]
    pub interval: u64,

    /// Print output as <hum,temp> to stdout (for use in unix pipeline).
    #[clap(short = 'P', long, action = SetTrue)]
    pub pipe: bool,

    /// Output CSV directory.
    #[clap(short, long, default_value = "~", value_parser = parse_directory, requires = "csv")]
    pub directory: PathBuf,

    /// Output CSV filename format (see
    /// https://docs.rs/chrono/latest/chrono/format/strftime/index.html for valid specifiers).
    #[clap(short, long, default_value = "%Y%m%d")]
    pub format: String,

    /// Dumps data to CSV file.
    #[clap(long, action = SetTrue)]
    pub csv: bool,
}
