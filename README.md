<div align="center">
  <h1 align="center">Datalogger</h1>

  ![GitHub releases](https://img.shields.io/github/downloads/marcoradocchia/datalogger/total?color=%23a9b665&logo=github)
  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/datalogger?color=ea6962&logo=github)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/datalogger?color=%23d8a657&logo=github)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/datalogger?color=%2389b482&logo=github)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=%23d3869b&logo=github)
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/datalogger?color=%23e78a4e)
  <!-- ![Crates.io downloads](https://img.shields.io/crates/d/datalogger?label=crates.io%20downloads&color=%23a9b665&logo=rust) -->
  <!-- ![Crates.io version](https://img.shields.io/crates/v/datalogger?logo=rust&color=%23d8a657) -->
</div>

Humidity & Temperature CLI datalogger for DHT22 sensor on Raspberry Pi.

## Index

- [Install](#install)
  - [Git](#git)
  - [Cargo](#cargo)
    - [Master branch](#master-branch)
    - [Latest release from crates.io](#latest-release-from-crates.io)
- [Uninstall](#uninstall)
- [Usage](#usage)
- [Changelog](#changelog)
- [License](#license)

## Install

The following installation instructions assume a **Rust toolchain** installed
on the system. In order to install such toolchain you can use `rusutp`: see
https://www.rust-lang.org/tools/install for further installation
instructions and notes.

### Git

If you want to install `datalogger`, including **manpage** and shell
**completions** (Bash, Zsh, Fish), clone this repository and compile/install
using `make`:
```sh
git clone https://github.com/marcoradocchia/datalogger
cd datalogger
make
sudo make install
```

### Cargo

#### Master branch

To build and install from master branch run:
```sh
cargo install --git https://github.com/marcoradocchia/datalogger --branch master
```

#### Latest release from crates.io

To build and install the latest release from
[crates.io](https://crates.io/crates/datalogger) run:
```
cargo install datalogger
```

## Usage

```
datalogger 0.2.0
Marco Radocchia <marco.radocchia@outlook.com>
Humidity & Temperature CLI datalogger for DHT22 sensor on Raspberry Pi.

USAGE:
    datalogger [OPTIONS] --pin <PIN>

OPTIONS:
        --csv                      Dumps data to CSV file (can be swapped at runtime signalling
                                   `datalogger` process with SIGUSR1)
    -d, --directory <DIRECTORY>    Output CSV directory [default: ~]
    -f, --format <FORMAT>          Output CSV filename format (see
                                   https://docs.rs/chrono/latest/chrono/format/strftime/index.html
                                   for valid specifiers) [default: %Y%m%d]
    -h, --help                     Print help information
    -i, --interval <INTERVAL>      Interval in seconds between consecutive measures [default: 120]
    -p, --pin <PIN>                GPIO pin for DHT22 data connection
    -P, --pipe                     Print output as `<hum,temp>` to stdout (for use in unix pipeline)
    -q, --quiet                    Mute standard output
    -V, --version                  Print version information
```

## Changelog

Complete [CHANGELOG](CHANGELOG.md).

## License

[GPLv3](LICENSE)
