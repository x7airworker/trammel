# trammel

trammel is a tool for converting C headers into implementable Rust stubs. It aims to simplify the process of integrating C libraries with Rust projects by generating Rust-compatible bindings.

## 🚧 Work in Progress

⚠️ **This project is a Work in Progress and is not yet production-ready.** Expect breaking changes and incomplete features as development continues.

## Features (Planned)

- Parse C header files (`.h`) and generate Rust stub files (`.rs`).
- Support for common C types, structures and functions.
- Basic macro and function conversion.
- CLI interface for easy usage.

## Installation

Currently, trammel is not available via package managers. To use it, you need to build it from source:

```sh
git clone https://github.com/x7airworker/trammel.git
cd trammel
cargo build --release
```

## Usage

Once built, you can run trammel with:

```sh
./target/release/trammel path/to/header.h output.rs
```

## Contributing

Contributions are welcome! Feel free to open issues and pull requests to help improve trammel.

## License

This project is licensed under the MIT License.
