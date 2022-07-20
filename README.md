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

Humidity & Temperature datalogger for DHT22 sensor on Raspberry Pi.

## Index

- [Install](#install)
  * [Master branch](#master-branch)
  * [Latest release from crates.io](#latest-release-from-crates.io)
- [Uninstall](#uninstall)
- [Usage](#usage)
- [Changelog](#changelog)
- [License](#license)

## Install

The following installation instructions assume a **Rust toolchain** installed
on the system. In order to install such toolchain you can use `rusutp`: see
https://www.rust-lang.org/tools/install for further installation
instructions and notes.

### Master branch

To build and install from master branch run:
```sh
cargo install --git https://github.com/marcoradocchia/datalogger --branch master
```

### Latest release from crates.io

To build and install the latest release from
[crates.io](https://crates.io/crates/datalogger) run:
```
cargo install datalogger
```

## Uninstall

To uninstall run:
```
cargo uninstall datalogger
```

## Usage

```
datalogger 0.1.0
Marco Radocchia <marco.radocchia@outlook.com>
Humidity & Temperature datalogger for DHT22 sensor on Raspberry Pi.

USAGE:
    datalogger [OPTIONS] --pin <PIN> [OUTPUT]

ARGS:
    <OUTPUT>    Output CSV data file

OPTIONS:
    -h, --help                   Print help information
    -i, --interval <INTERVAL>    Interval in seconds between consecutive measures [default: 120]
    -p, --pin <PIN>              GPIO pin for DHT22 data connection
    -V, --version                Print version information
```

## Changelog

Complete [CHANGELOG](CHANGELOG.md).

## License

[GPLv3](LICENSE)
