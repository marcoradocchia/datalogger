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

use rppal::gpio::Error as RppalGpioError;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::{self, Error as IoError, Write},
    path::PathBuf,
};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

/// Enum representing handled runtime errors.
#[derive(Debug)]
pub enum ErrorKind {
    /// Occurs when sensor reading results in GPIO error.
    GpioError(RppalGpioError),
    /// Occurs when max retries value is reached while reading DHT22 sensor.
    MaxRetries,
    /// Occurs when unable to open file.
    FileOpenErr(PathBuf, IoError),
    /// Occurs when unable to write to file.
    FileWriteErr(PathBuf, IoError),
    /// Occurs when unable to access file metadata.
    FileMetadataErr(PathBuf, IoError),
    /// Occurs when mpsc message passing between threads fails.
    MsgPassingErr,
    /// Occurs when unable to create directory.
    MkDirErr(PathBuf, IoError),
    /// Any other error.
    Other(String),
}

impl ErrorKind {
    /// Colorize error output.
    pub fn colorize(&self) -> io::Result<()> {
        let color_choice = match atty::is(atty::Stream::Stderr) {
            true => ColorChoice::Auto,
            false => ColorChoice::Never,
        };

        let writer = BufferWriter::stderr(color_choice);
        let mut buffer = writer.buffer();

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
        write!(&mut buffer, "error:")?;
        buffer.reset()?;
        writeln!(&mut buffer, " {}.", self)?;

        writer.print(&buffer)
    }
}

impl Display for ErrorKind {
    /// Display error message.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GpioError(err) => write!(f, "unable to access GPIO pins {}", err),
            Self::MaxRetries => write!(f, "reached max retries while reading DHT22 sensor"),
            Self::FileOpenErr(path, err) => {
                write!(f, "unable to open '{}': {}", path.display(), err)
            }
            Self::FileWriteErr(path, err) => {
                write!(f, "unable to write to '{}': {}", path.display(), err)
            }
            Self::FileMetadataErr(path, err) => {
                write!(f, "unable to access '{}' metadata: {}", path.display(), err)
            }
            Self::MsgPassingErr => write!(f, "unable to send messages between threads"),
            Self::MkDirErr(path, err) => write!(
                f,
                "unable to create directory '{}': {}",
                path.display(),
                err
            ),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ErrorKind {}

impl From<&str> for ErrorKind {
    fn from(msg: &str) -> Self {
        ErrorKind::Other(msg.to_string())
    }
}
