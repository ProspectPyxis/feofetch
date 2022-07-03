# feofetch

Yet another (x)fetch tool, written in rust with cross-platform in mind.

## Note on cross-platform

This program was written specifically with cross-platform libraries, so in theory, 
this should compile and run on Windows, MacOS, and Linux. However, this program 
so far has only been tested on Linux, and a lot of functionality will be missing
on other OSes.

Any feature requests, bug reports, or pull requests regarding this are very welcome!

## Installation

### With cargo

`feofetch` can be installed with [cargo](https:#www.rust-lang.org/tools/install).

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

## Configuration

This program reads configuration from `CONFIG_PATH/config.toml`. On MacOS and Linux, 
`CONFIG_PATH` is at `$HOME/.config/feofetch`, while on Windows, it is at 
`%APPDATA%/ProspectPyxis/feofetch/config`.

Any option can be omitted, in which case it will fall back to the default as described below.

```toml
# If this option is set to `true`, the data names will be replaced with a Nerd Font icon.
# Valid options: true, false
use_icons = false

# How many spaces, at minimum, will be in between the data names and values.
# Valid options: positive integers or 0
align_spaces = 2

# Sets the horizontal and vertical offset to print at in terms of
# characters and lines, respectively.
# Valid options: An array of two positive integers or 0
offset = [0, 0]

# Sets the amount of blank lines to print after the output.
# Useful if you want the output to feel less crowded with the shell prompt.
# Valid options: Any positive integers or 0
padding_lines = 1

# Sets the data that will be displayed, in order.
# Valid options: An array of any combination of the following:
# os, version, uptime, packages, wm
data = ["os", "version", "uptime", "packages", "wm"]

# If displaying `packages`, sets whether to display the package managers used.
# Valid options: true, false
display_package_manager = false

# If displaying `wm`, sets whether to use `wmctrl` to check the window manager,
# or to simply try to read from environment variables instead.
# This requires wmctrl to be installed on your machine.
# Valid options: true, false
use_wmctrl = false

[ascii]
# Sets whether to print an ascii file or not.
# Valid options: true, falsee
print = false

# Sets the file to read the ascii art from.
# Note that if you're specifying the path to the config directory yourself,
# you must provide the full path, as the `CONFIG_PATH` shown here is only a placeholder.
# Valid options: any path to a text file
ascii_path = "CONFIG_PATH/ascii.txt"

# How many spaces, at minimum, will be in between the ascii art and the data names.
# Valid options: positive integers or 0
align_spaces = 2
```

## Contributing

Any pull requests, issues, or feature requests are welcome!

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE.md) 
or [MIT License](LICENSE-MIT.md), at your option.
