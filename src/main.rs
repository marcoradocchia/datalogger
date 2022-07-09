use chrono::{DateTime, Local};
use clap::Parser;
use dht22_pi::{self, Reading, ReadingError};
use std::{
    fmt::Display, fs::OpenOptions, io::Write, path::PathBuf, process, sync::mpsc, thread,
    time::Duration,
};

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
    #[clap(short, long, default_value_t = 2, value_parser = parse_interval)]
    interval: u64,

    /// Output data file.
    #[clap(value_parser)]
    output: Option<PathBuf>,
}

/// Sensor Reading and Date/Time.
///
/// # Fields
/// - reading: DHT22 sensor Reading
/// - datetime: date & time of the measurement
struct Measure {
    reading: Reading,
    datetime: DateTime<Local>,
}

impl Measure {
    fn new(reading: Reading, datetime: DateTime<Local>) -> Self {
        Self { reading, datetime }
    }

    // Format measurement data to csv.
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{}\n",
            self.datetime.date().format("%Y-%m-%d"),
            self.datetime.time().format("%H:%M:%S"),
            self.reading.humidity,
            self.reading.temperature
        )
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} -> Humidity: {}%, Temperature: {}°C",
            self.datetime.date().format("%Y-%m-%d"),
            self.datetime.time().format("%H:%M:%S"),
            self.reading.humidity,
            self.reading.temperature
        )
    }
}

fn main() {
    // Parse CLI arguments.
    let args = Args::parse();

    // Channel for message passing between main thread and output thread.
    let (tx, rx) = mpsc::channel::<Measure>();

    thread::spawn(move || {
        for reading in rx {
            // If output is file, write measure values to file, otherwhise print them to stdout.
            if let Some(output) = &args.output {
                match OpenOptions::new().append(true).open(output) {
                    Ok(mut file) => {
                        file.write_all(reading.to_csv().as_bytes())
                            .unwrap_or_else(|e| {
                                eprintln!("error: {e}");
                                process::exit(1);
                            })
                    }
                    Err(e) => {
                        eprintln!("error: {e}");
                        process::exit(1);
                    }
                }
            } else {
                println!("{}", reading);
            }
        }
    });

    // Main loop.
    loop {
        let mut retries = 0;
        tx.send(Measure::new(
            // reading
            loop {
                match dht22_pi::read(args.pin) {
                    Err(e) => {
                        // Handle all ReadingError variants: don't exit process on Timeout, retry
                        // read instead.
                        match e {
                            ReadingError::Timeout => {
                                if retries < 10 {
                                    eprintln!("warning: timeout reached while reading sensor");
                                    retries += 1;
                                    continue;
                                } else {
                                    // After 10 consecutive timeouts, exit process with error.
                                    eprintln!("error: reached max retries");
                                    process::exit(1);
                                }
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
                    }
                    Ok(reading) => break reading,
                }
            },
            // datetime
            Local::now(),
        ))
        .expect("unable to send reading to 'ouput' thread");

        thread::sleep(Duration::from_secs(args.interval));
    }
}
