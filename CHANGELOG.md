# Changelog

## [0.2.0] - 2022-08-03

### Added

- `directory` CLI option to specify output CSV file directory.
- `format` CLI option to specify output CSV filename format.
- `quiet` CLI option to mute standard output.
- `csv` CLI option to enable CSV file output at launch.
- Signal handler listening for `SIGUSR1` signals to toggle (*enable*/*disable*)
  CSV file output at runtime (e.g. `pkill --signal=SIGUSR1 datalogger`).
- [build.rs](build.rs) build script to generate manpage & shell completions.
- [Makefile](Makefile) to compile/install/uninstall `datalogger` alongside
  manpage & shell completions.

## [0.1.0] - 2022-07-09

Initial release.
