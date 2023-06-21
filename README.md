# grepw 🔍

`grepw` is a command-line utility for searching text files for lines that match a given pattern. It is written in Rust and is designed to work on Windows.

## Features ✨

- **Case-insensitive search**: Add an option to ignore case distinctions in the pattern and input data, so that characters that differ only in case match each other.
- **Invert match**: Add an option to invert the sense of matching, to select non-matching lines.
- **File search**: Search a specified file or standard input for lines that match the given pattern.

## Installation 💾

To install `grepw`, just [Download the latest release](https://github.com/yourusername/grepw/releases) from GitHub, and then run the setup executable.

## Usage 💻

The basic syntax for `grepw` is:

grepw [options] PATTERN [FILE]


- `options`: Optional flags that modify the behavior of the command.
- `PATTERN`: The pattern to search for.
- `FILE`: The file to search. If no file is specified, `grepw` searches the standard input.

## Contributing 🤝

Contributions to `grepw` are welcome! Please feel free to open an issue or submit a pull request on GitHub.

## License 📜

`grepw` is licensed under the MIT license. See the [LICENSE](./LICENSE) file for more information.
