use chrono::{DateTime, Local};
use clap::Parser;
use dht22_pi::{self, Reading, ReadingError};
use std::{
    fmt::{self, Display, Formatter},
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    process,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const MAX_RETRIES: u8 = 20;

/// Enum representing handled runtime errors.
enum ErrorKind {
    /// Occurs when sensor reading results in GPIO error.
    GpioError(String),
    /// Occurs when max retries value is reached while reading DHT22 sensor.
    MaxRetries,
    /// Occurs when theres is an issue with file output.
    FileError(String)
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GpioError(msg) => format!("GPIO error `{}`", msg),
            Self::MaxRetries => "reached max retries while reading DHT22 sensor".to_string(),
            Self::FileError(msg) => format!("unable to write data to file `{}`", msg),
        }
        .fmt(f)
    }
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

    /// Interval in seconds between consecutive measures.
    #[clap(short, long, default_value_t = 120, value_parser = parse_interval)]
    interval: u64,

    /// Print output to stdin to be used in scripts [<hum,temp>].
    #[clap(short, value_parser)]
    script: bool,

    /// Output CSV data file.
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

    /// Format measurement as <hum,temp> to be used in scripts or piped in unix pipeline.
    fn to_script_format(&self) -> String {
        format!("{},{}", self.reading.humidity, self.reading.temperature)
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

fn run() -> Result<(), ErrorKind> {
    // Parse CLI arguments.
    let args = Args::parse();

    // Channel for message passing between main thread and output thread.
    let (tx, rx) = mpsc::channel::<Measure>();

    thread::spawn(move || -> Result<(), ErrorKind> {
        for reading in rx {
            // If output is file, write measure values to file, otherwhise print them to stdout.
            if let Some(output) = &args.output {
                match OpenOptions::new().create(true).append(true).open(output) {
                    Ok(mut file) => {
                        // If file is empty write headers.
                        if file
                            .metadata()
                            .expect("unable to get output file metadata")
                            .len()
                            == 0
                        {
                            file.write_all(b"DATE,TIME,HUMIDITY,TEMPERATURE\n").unwrap();
                        }

                        // Write Measure to csv file.
                        file.write_all(reading.to_csv().as_bytes()).unwrap();
                    }
                    Err(e) => return Err(ErrorKind::FileError(e.to_string())),
                }
            } else {
                if args.script {
                    println!("{}", reading.to_script_format());
                } else {
                    println!("{reading}");
                }
            }
        }

        Ok(())
    });

    // Main loop.
    loop {
        let start_measuring = Instant::now();
        let mut retries = 0;
        tx.send(Measure::new(
            // reading
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

        // Sleep for `args.interval` corrected by the time spend measuring.
        thread::sleep(Duration::from_secs(args.interval) - start_measuring.elapsed());
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
