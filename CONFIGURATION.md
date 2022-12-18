# Configuration

By default, `feofetch` reads the configuration file from
`CONFIG_PATH/config.toml`. On Linux and MacOS, `CONFIG_PATH` is at
`$HOME/.config/feofetch`, while on Windows, it is at
`%APPDATA%/ProspectPyxis/feofetch/config`. You can also force feofetch to use a
different `.toml` file as its config with the `--config-path` command line
option.

Any of the values may be omitted, in which case it will fall back to the
default.

You may also see an example default configuration file
[here](config.default.toml).

## Main Section

General options for `feofetch` that don't belong to any table.

### `data`

An array of data types that you want to display.

- **Possible values:** array of data (`os`, `version`, `uptime`, `packages`,
  `wm`, `shell`, `terminal`)
- **Default:** [`os`, `version`, `uptime`, `packages`, `wm`]

```toml
data = ["os", "version", "uptime", "packages", "wm"]
```

### `use_icons`

Whether to print out a Nerd Font icon for each data option, or to print a
text-based label.

- **Possible values:** boolean (`true`, `false`)
- **Default:** `false`

```toml
use_icons = false
```

### `label_color`

The color of the label text/icon for each data option.

- **Possible values:** Color (`normal`, `black`, `dark_grey`/`dark_gray`, `red`,
  `dark_red`, `green`, `dark_green`, `yellow`, `dark_yellow`, `blue`,
  `dark_blue`, `magenta`, `dark_magenta`, `cyan`, `dark_cyan`, `white`,
  `grey`/`gray`)
- **Default:** `cyan`

```toml
label_color = "cyan"
```

### `align_spaces`

How many spaces, at minimum, will be between the label text/icon and its data
value.

- **Possible values:** Any positive integer
- **Default:** 2

```toml
align_spaces = 2
```

### `offset`

Set the horizontal and vertical offset in terms of columns and lines when
printing, respectively.

- **Possible values:** Array of two positive integers
- **Default:** [0, 0]

```toml
offset = [0, 0]
```

### `padding_lines`

How many empty lines to print after the main body. Useful for if you want some
space between `feofetch`'s output and the terminal prompt.

- **Possible values:** Any positive integer
- **Default:** 1

```toml
padding_lines = 1
```

## `ascii`

Options related to printing ascii art.

### `print`

Whether to print out ascii art alongside the data or not.

- **Possible values:** boolean (`true`, `false`)
- **Default::** `false`

```toml
[ascii]
print = false
```

### `ascii_path`

The path to the file containing the ascii art to use.

- **Possible values:** Any path to a text file
- **Default:** `CONFIG_PATH/ascii.txt`

```toml
[ascii]
# CONFIG_PATH is a placeholder, please define a valid relative or absolute path
ascii_path = "CONFIG_PATH/ascii.txt"
```

### `color`

The color to print the ascii art in.

- **Possible values:** Color (`normal`, `black`, `dark_grey`/`dark_gray`, `red`,
  `dark_red`, `green`, `dark_green`, `yellow`, `dark_yellow`, `blue`,
  `dark_blue`, `magenta`, `dark_magenta`, `cyan`, `dark_cyan`, `white`,
  `grey`/`gray`)
- **Default:** `normal`

```toml
[ascii]
color = "normal"
```

### `align_spaces`

How many spaces, at minimum, will be between the ascii art and the data labels.

- **Possible values:** Any positive integer
- **Default:** 2

```toml
[ascii]
align_spaces = 2
```

## `packages`

Options related to the `packages` data option.

### `print_package_manager_names`

Whether to print out the package managers you have alongside the amount or not.

- **Possible values:** boolean (`true`, `false`)
- **Default:** `false`

```toml
[packages]
print_package_manager_names = false
```

## `wm`

Options related to the `wm` data option.

### `use_wmctrl`

Whether to try using `wmctrl` to detect your window manager. This has no effect
if `wmctrl` isn't installed.

- **Possible values:** boolean (`true`, `false`)
- **Default:** `false`

```toml
[wm]
use_wmctrl = false
```
