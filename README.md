# SecurePurge

## Introduction

SecurePurge is a Rust-based command-line utility designed for securely deleting files from a filesystem. It employs a method where file data is overwritten several times before the file is finally deleted, ensuring that the data is not easily recoverable. This tool is particularly useful for situations where sensitive data needs to be securely erased from storage devices.

## How to Use

SecurePurge is straightforward to use with its command-line interface. Here are some examples of how to use SecurePurge:

### Removing a File

To remove a file securely, you can use the `remove` subcommand with the `-p` flag to specify the file path and the `-t` flag to indicate the number of times the file data should be overwritten. You can also use the `-r` flag for recursive file deletion.

Example:
```shell
secure_purge remove -p /path/to/file -t 3 -r
```

This command will overwrite the file located at `/path/to/file` three times before deleting it, and it will also delete directories recursively.

### Formatting a Disk (Upcoming Feature)

An upcoming feature for SecurePurge is the ability to format a disk securely. This will include overwriting the disk data several times before the actual formatting process.

## Dependencies

SecurePurge relies on several dependencies to function properly:

- **clap**: A powerful Rust library used for parsing command-line arguments and parameters. SecurePurge uses clap to handle its command-line interface, making it user-friendly and robust. (Current Version: 4.4.18)

- **indicatif**: A Rust crate for creating progress bars. SecurePurge uses indicatif to display progress when securely deleting files. (Current Version: 0.17.7)

- **rand**: This is a Rust crate that provides functionalities for generating random numbers. In SecurePurge, it is used to generate random data to overwrite files during the secure deletion process. (Current Version: 0.8.5)

- **colored**: A Rust crate for coloring terminal output. SecurePurge uses colored to enhance the readability of its output. (Current Version: 2.1.0)

## Contributors

SecurePurge is an open-source project, and contributions are welcome. If you're interested in contributing, please feel free to fork the repository, make your changes, and submit a pull request.

## Installation

Currently, SecurePurge can be compiled from source. Clone the repository and use Cargo, Rust's package manager and build system, to build and run the application.

```shell
git clone https://github.com/yourusername/securepurge.git
cd securepurge
cargo install --path .

### or alternativly build them with:
cargo build --release
cargo run --release
```

## Future Enhancements

- The addition of the disk formatting feature, which will allow for secure erasing of entire storage devices.
- Further optimization and testing on various filesystems and platforms.

## License

SecurePurge is licensed under [MIT License](LICENSE.md). Feel free to use, modify, and distribute the code as per the license terms.

---

SecurePurge is still in active development, and new features and improvements are continually being added. Stay tuned for more updates and enhancements.
