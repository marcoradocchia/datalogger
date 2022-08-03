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

mod args;
mod error;

use args::{Args, Parser};
use chrono::{DateTime, Local};
use dht22_pi::{self, Reading, ReadingError};
use error::ErrorKind;
use signal_hook::{
    consts::SIGUSR1,
    flag::register,
};
use std::{
    fmt::{self, Display, Formatter},
    fs::OpenOptions,
    io::Write,
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

const MAX_RETRIES: u8 = 20;

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

    /// Format measurement as <hum,temp> to be used in unix pipelines.
    fn to_pipe(&self) -> String {
        format!("{},{}", self.reading.humidity, self.reading.temperature)
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date: {},  Time: {}, Humidity: {}%, Temperature: {}°C",
            self.datetime.date().format("%Y-%m-%d"),
            self.datetime.time().format("%H:%M:%S"),
            self.reading.humidity,
            self.reading.temperature
        )
    }
}

/// Retry DHT22 sensor reading and update the retries counter.
fn retry(retries: &mut u8) -> Result<(), ErrorKind> {
    // After 10 consecutive timeouts, exit process with error.
    if *retries >= MAX_RETRIES {
        return Err(ErrorKind::MaxRetries);
    };
    // If max retries is not reached, increase counter.
    *retries += 1;

    Ok(())
}

fn run(args: Args) -> Result<(), ErrorKind> {
    // Channel for message passing between main thread and output thread.
    let (tx, rx) = mpsc::channel::<Measure>();

    // Output thread.
    thread::spawn(move || -> Result<(), ErrorKind> {
        // Register signal hook for SIGUSR1 events: such events swap the current args.csv value
        // (this is used to enable/disable writing of measures to output file at runtime).
        let sig = Arc::new(AtomicBool::new(false));
        // Set `sig` to true when the program receives a SIGTERM kill signal.
        register(SIGUSR1, Arc::clone(&sig)).expect("unable to register SIGUSR1 event handler");

        // Local copy of args.csv which will be swapped every time SIGUSR1 signal is received,
        // allowing user to swap CSV file printing behaviour (start/stop printing measures to file
        // anytime at runtime).
        let mut csv = args.csv;

        for measure in rx {
            // If SIGUSR1 received (hence sig is true), swap csv and restore sig to false.
            if sig.load(Ordering::Relaxed) {
                csv = !csv;
                sig.store(false, Ordering::Relaxed);
            }

            if csv {
                let filename = Local::now().format(&args.format).to_string();
                let csv_file = &args.directory.join(filename).with_extension("csv");
                match OpenOptions::new().create(true).append(true).open(csv_file) {
                    Ok(mut file) => {
                        // If file is empty, then write headers.
                        if file
                            .metadata()
                            .expect("unable to get output file metadata")
                            .len()
                            == 0
                        {
                            file.write_all(b"DATE,TIME,HUMIDITY,TEMPERATURE\n").unwrap();
                        }

                        // Write Measure to csv file.
                        file.write_all(measure.to_csv().as_bytes()).unwrap();
                    }
                    Err(e) => return Err(ErrorKind::FileError(e.to_string())),
                }
            }

            if !args.quiet {
                // If `pipe` options is passed, print with "<hum>,<temp>" format to stdout, else
                // print human readable values.
                if args.pipe {
                    println!("{}", measure.to_pipe());
                } else {
                    println!("{}", measure);
                }
            }
        }

        Ok(())
    });

    // Start main loop: loop guard is 'received SIGINT'.
    loop {
        let start_measuring = Instant::now();
        let mut retries = 0;
        tx.send(Measure::new(
            // reading
            // Loop until valid result is obtained or max retries value is reached.
            loop {
                match dht22_pi::read(args.pin) {
                    Err(e) => {
                        // Handle all ReadingError variants: don't exit process on Timeout, retry
                        // read instead.
                        match e {
                            ReadingError::Timeout | ReadingError::Checksum => {
                                retry(&mut retries)?;
                                continue;
                            }
                            ReadingError::Gpio(e) => {
                                return Err(ErrorKind::GpioError(e.to_string()))
                            }
                        }
                    }
                    Ok(reading) => break reading,
                }
            },
            // datetime
            Local::now(),
        ))
        .expect("unable to send measure to 'ouput' thread");

        // Sleep for `args.interval` corrected by the time spent measuring.
        thread::sleep(Duration::from_secs(args.interval) - start_measuring.elapsed());
    }
}

fn main() {
    // Parse CLI arguments.
    let args = Args::parse();

    // Run the program and catch errors.
    if let Err(e) = run(args) {
        if e.colorize().is_err() {
            eprintln!("error: {e}");
        }
        process::exit(1);
    }
}
