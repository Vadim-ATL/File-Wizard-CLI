# File Wizard

File Wizard is a powerful CLI tool built on rust for smart file organization, search, and manipulation.

# Dependencies

- **Rust**: `1.84.1`

| Dependency      | Version       | Features           |
|-----------------|---------------|--------------------|
| `anyhow`       | `1.0.95`       | -                  |
| `chrono`       | `0.4.39`       | -                  |
| `clap`         | `4.5.27`       | `"derive"`         |
| `log`          | `0.4.25`       | -                  |
| `serde`        | `1.0.217`      | -                  |
| `serde_derive` | `1.0.217`      | -                  |
| `serde_json`   | `1.0.137`      | -                  |
| `simplelog`    | `0.12.2`       | -                  |

## Installation

Use Rust (up to version 1.84.1) and the Cargo package manager to build and run the project.

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

//Run file organization by files types (Images, Videos, Music, Other)
cargo run -- --types

//Revert changes from moving files, all movements from log moved_files wil be moved back
cargo run -- --revert

```

## Contributing

Pull requests are always welcome 🤗, I am only starting this project, so feel free to suggest, criticize (hope constructively). 

For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT LICENSE](LICENSE)
