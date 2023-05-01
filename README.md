# PortScanner

PortScanner is a command-line tool for scanning a target IP address for open ports. It is written in Rust and uses the `tokio` crate for async I/O operations, as well as the `clap` and `chrono` crates for argument parsing and time formatting, respectively.

## Installation

To install the tool, you will need to have Rust and Cargo installed on your system. You can then clone the repository and build the tool using Cargo:

```
git clone https://github.com/TitoMitto/portscanner.git
cd portscanner
cargo build --release
```

This will build an optimized release build of the tool in the `target/release` directory. You can then run the tool by running the `portscanner` binary in the `target/release` directory.

## Usage

To use the tool, simply run the `portscanner` binary with the following arguments:

```
./target/release/portscanner [IP_ADDRESS] [-m MAX_PORT]
```

where `IP_ADDRESS` is the IP address to scan, and `MAX_PORT` is the maximum port number to scan (defaults to 1000 if not specified).

The tool will then scan the target IP address for open ports and print a message for each open port it finds.

## License

This tool is licensed under the [MIT License](https://opensource.org/licenses/MIT). See the `LICENSE` file for more information.