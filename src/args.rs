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

pub use clap::Parser;
use clap::{
    value_parser,
    ArgAction::{Set, SetTrue},
};
use std::path::PathBuf;

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
    /// Interval between consecutive measures in seconds.
    #[clap(
        short,
        long,
        default_value_t = 120,
        value_parser = value_parser!(u16).range(2..)
    )]
    pub interval: u16,
    /// Print output as `<hum,temp>` to stdout (for use in unix pipeline).
    #[clap(short = 'P', long, action = SetTrue)]
    pub pipe: bool,
    /// Output CSV directory.
    #[clap(short, long, default_value = "~", value_parser)]
    pub directory: PathBuf,
    /// Output CSV filename format (see
    /// https://docs.rs/chrono/latest/chrono/format/strftime/index.html for valid specifiers).
    #[clap(short, long, default_value = "%Y%m%d")]
    pub format: String,
    /// Dumps data to CSV file (can be swapped at runtime signalling `datalogger` process with
    /// SIGUSR1).
    #[clap(long, action = SetTrue)]
    pub csv: bool,
    /// Mute standard output.
    #[clap(short, long, action = SetTrue)]
    pub quiet: bool,
}
