# Tablify

A command-line application written in Rust that formats input text into table format. It reads from standard input and outputs a formatted table to standard output.

[日本語の説明はこちら (Japanese README)](README_ja.md)

## Features

- Reads text from standard input
- Formats text into a table structure
- Outputs formatted table to standard output
- Supports custom separators and regular expression patterns
- Handles full-width characters (Japanese, Chinese, Korean) correctly
- Support for header rows and custom column names

## Installation

```bash
git clone https://github.com/takeisa/tablify.git
cd tablify
cargo build --release
```

## Usage

### Basic Usage

By default, input text is split using TAB characters as separators:

```bash
$ echo -e "apple\t100\norange\t200" | cargo run
| apple  | 100 |
| orange | 200 |
```

### Command Line Options

#### Separator Options

**Custom Character Separator (`-s, --separator`)**
```bash
$ echo -e "apple 100\norange 200" | cargo run -- -s ' '
| apple  | 100 |
| orange | 200 |
```

**Regular Expression Pattern (`-p, --separator-pattern`)**
```bash
$ echo -e "apple   100\norange  200" | cargo run -- -p '\s+'
| apple  | 100 |
| orange | 200 |
```

#### Header Options

**Header Row (`--header`)**
```bash
$ echo -e "item\tprice\napple\t100\norange\t200" | cargo run -- --header
| item   | price |
+--------+-------+
| apple  | 100   |
| orange | 200   |
```

**Custom Column Names (`--columns`)**
```bash
$ echo -e "apple\t100\norange\t200" | cargo run -- --columns "fruit,price"
| fruit  | price |
+--------+-------+
| apple  | 100   |
| orange | 200   |
```

### Full-Width Character Support

Tablify correctly handles full-width characters commonly used in Asian languages:

```bash
$ echo -e "りんご\t100\nオレンジ\t200" | cargo run
| りんご   | 100 |
| オレンジ | 200 |
```

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [regex](https://crates.io/crates/regex) - Regular expression support
- [unicode-width](https://crates.io/crates/unicode-width) - Proper width calculation for Unicode characters

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is open source and available under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.