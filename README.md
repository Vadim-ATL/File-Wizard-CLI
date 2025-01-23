# File Wizard

File Wizard is a powerful CLI tool built on rust for smart file organization, search, and manipulation.

## Installation

Use Rust (up to version 1.82.0) and the Cargo package manager to build and run the project.

```bash
cargo build
cargo run
```

## Usage

```rust
//Organize files in current directory
cargo run

//Specify a different directory
cargo run -- --path /path/to/directory

//Dry run (preview changes without moving files)
cargo run -- --dry-run
```

## Contributing

Pull requests are always welcome 🤗, I am only starting this project, so feel free to suggest, criticize (hope constructively). 

For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)