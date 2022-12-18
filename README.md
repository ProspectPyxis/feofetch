# feofetch

Yet another (x)fetch tool, written in rust with cross-platform in mind.

## Note on cross-platform

This program was written specifically with cross-platform libraries, so in
theory, this should compile and run on Windows, MacOS, and Linux. However, this
program so far has only been tested on Linux, and a lot of functionality will be
missing on other OSes.

Any feature requests, bug reports, or pull requests regarding this are very
welcome!

## Installation

### With cargo

`feofetch` can be installed with
[cargo](https:#www.rust-lang.org/tools/install).

```sh
cargo install feofetch
```

### Build from source

If you want the latest build, you can build directly from source. This requires
[rust](https:#www.rust-lang.org/tools/install) to be installed on your system.

```sh
git clone https:#github.com/ProspectPyxis/feofetch.git
# Or download the source code zip file directly and extract

cd feofetch/
cargo install --path .
```

## Usage

Just run `feofetch` in the terminal:

```sh
feofetch
```

The program also supports various command line flags.

```
Usage: feofetch [OPTIONS]

Options:
  -c, --config-path <CONFIG_PATH>      Override default config location with the specified path
  -x, --x-offset <X_OFFSET>            Set x-offset to this number, overriding config
  -y, --y-offset <Y_OFFSET>            Set y-offset to this number, overriding config
  -p, --padding-lines <PADDING_LINES>  Set padding lines to this number, overriding config
  -h, --help                           Print help information
  -V, --version                        Print version information
```

## Configuration

For how to configure this program, see [CONFIGURATION.md](CONFIGURATION.md).

## Contributing

Any pull requests, issues, or feature requests are welcome!

## License

This project is licensed under either of
[Apache License, Version 2.0](LICENSE-APACHE.md) or
[MIT License](LICENSE-MIT.md), at your option.
