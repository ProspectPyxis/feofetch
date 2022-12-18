# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking Changes

- Restructured config file - split options specific to data types to its own
  tables.

### Added

- Command line flags: `--help`, `--version`.
- `--config-path`/`-c` command line flag to set an alternate config path for one
  execution.
- `--x-offset`/`-x` and `--y-offset`/`-y` command line flags fo override offsets
  for this run.
- `--padding-lines`/`-p` command line flag to override padding lines for this
  run.
- Highly improved error handling - program should no longer panic other than in
  catastrophic cases, thanks to implementing the `anyhow` crate
- New data options: `shell`, `terminal`.
- `label_color` config option that supports all ANSI colors

### Changed

- Better icons for `os` data type - now shows the appropriate logo for Windows,
  Mac, and Linux for its icons

## [0.1.0] - 2022-07-03

### Added

- Data options: `os`, `version`, `uptime`, `packages`, `wm`.
- `use_icons` config option.
- Ability to display ASCII art alongside data.
