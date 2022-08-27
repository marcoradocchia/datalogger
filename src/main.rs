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
use signal_hook::{consts::SIGUSR1, flag::register};
use std::{
    fmt::{self, Display, Formatter},
    fs::{self, OpenOptions},
    io::Write,
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, ErrorKind>;

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
fn retry(retries: &mut u8) -> Result<()> {
    // After 10 consecutive timeouts, exit process with error.
    if *retries >= MAX_RETRIES {
        return Err(ErrorKind::MaxRetries);
    };
    // If max retries is not reached, increase counter.
    *retries += 1;

    Ok(())
}

fn run(args: Args) -> Result<()> {
    // Create directory (including parent directories if not present) if doesn't exist.
    if !args.directory.is_dir() {
        fs::create_dir_all(&args.directory)
            .map_err(|err| ErrorKind::MkDirErr(args.directory.to_owned(), err))?;
    }

    // Channel for message passing between main thread and output thread.
    let (tx, rx) = mpsc::channel::<Measure>();

    // Output thread.
    thread::spawn(move || -> Result<()> {
        // Register signal hook for SIGUSR1 events.
        let sigusr1 = Arc::new(AtomicBool::new(false));
        // Set `sigusr1` to `true` when the program receives a SIGUSR1 signal.
        register(SIGUSR1, Arc::clone(&sigusr1))
            .map_err(|_| "unable to register SIGUSR1 event handler")?;

        // Local copy of args.csv which will be swapped every time SIGUSR1 signal is received,
        // allowing user to swap CSV file printing behaviour (start/stop dumping measures to CSV
        // file anytime at runtime).
        let mut csv = args.csv;

        for measure in rx {
            // If SIGUSR1 received (hence `sigusr1` is `true`), swap csv and restore `sigusr1` to
            // false.
            if sigusr1.load(Ordering::Relaxed) {
                csv = !csv;
                sigusr1.store(false, Ordering::Relaxed);
            }

            // If `csv` status is true, write data to CSV file.
            if csv {
                let filename = Local::now().format(&args.format).to_string();
                let csv_file = &args.directory.join(filename).with_extension("csv");
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(csv_file)
                    .map_err(|err| ErrorKind::FileOpenErr(csv_file.to_owned(), err))?;

                // If file is empty, then write headers.
                if file
                    .metadata()
                    .map_err(|err| ErrorKind::FileMetadataErr(csv_file.to_owned(), err))?
                    .len()
                    == 0
                {
                    file.write_all(b"DATE,TIME,HUMIDITY,TEMPERATURE\n")
                        .map_err(|err| ErrorKind::FileWriteErr(csv_file.to_owned(), err))?;
                }

                // Write Measure to csv file.
                file.write_all(measure.to_csv().as_bytes())
                    .map_err(|err| ErrorKind::FileWriteErr(csv_file.to_owned(), err))?;
            }

            if !args.quiet {
                // If `pipe` options is passed, print with "<hum>,<temp>" format to stdout, else
                // print human readable values.
                match args.pipe {
                    true => println!("{}", measure.to_pipe()),
                    false => println!("{}", measure),
                }
            }
        }

        Ok(())
    });

    // Start main loop: loop guard is 'received SIGINT'.
    loop {
        let instant = Instant::now();
        let mut retries = 0;
        tx.send(Measure::new(
            // Loop until valid result is obtained or max retries value is reached.
            loop {
                match dht22_pi::read(args.pin) {
                    // Handle all ReadingError variants:
                    // don't exit process on Timeout or Checksum errors, retry read instead.
                    Err(err) => match err {
                        ReadingError::Timeout | ReadingError::Checksum => {
                            retry(&mut retries)?;
                            continue;
                        }
                        ReadingError::Gpio(err) => {
                            return Err(ErrorKind::GpioError(err));
                        }
                    },
                    Ok(reading) => break reading,
                }
            },
            // datetime
            Local::now(),
        ))
        .map_err(|_| ErrorKind::MsgPassingErr)?;

        // Sleep for `args.interval` corrected by the time spent measuring: if elapsed time is
        // grates than the specified interval, this means the measuring process took longer than
        // expected, so don't wait at all since we're already late.
        if let Some(delay) =
            Duration::from_secs(args.interval.into()).checked_sub(instant.elapsed())
        {
            thread::sleep(delay);
        }
    }
}

fn main() {
    // Parse CLI arguments.
    let args = Args::parse();

    // Run the program and catch errors.
    if let Err(err) = run(args) {
        if err.colorize().is_err() {
            eprintln!("error: {err}.");
        }
        process::exit(1);
    }
}
