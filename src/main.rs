use clap::Parser;
use dht22_pi::{self, ReadingError};
use std::{path::PathBuf, process};

/// Humidity & Temperature datalogger for DHT22 sensor on Raspberry Pi.
#[derive(Parser, Debug)]
#[clap(
    author = "Marco Radocchia <marco.radocchia@outlook.com>",
    version,
    about,
    long_about = None
)]
struct Args {
    /// GPIO pin for DHT22 data connection.
    #[clap(short, long, value_parser)]
    pin: u8,

    /// Interval in seconds between consequent measures.
    #[clap(short, long, value_parser)]
    interval: u32,

    /// Output data file.
    #[clap(value_parser)]
    output: PathBuf,
}

fn main() {
    // Parse CLI arguments.
    let args = Args::parse();
    dbg!(&args);

    let measure = while let Err(e) = dht22_pi::read(args.pin) {
        match e {
            ReadingError::Timeout => {
                eprintln!("warning: timeout reached while reading sensor");
                continue;
            }
            ReadingError::Checksum => {
                eprintln!("error: incorrect checksum value");
                process::exit(1);
            }
            ReadingError::Gpio(e) => {
                eprintln!("error: {}", e);
                process::exit(1);
            }
        }
    };

    dbg!(measure);
}
