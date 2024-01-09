# Disk-Gym
## Video Example
https://github.com/mewmix/disk-gym/assets/42463809/57f91ed5-00be-442d-841d-c24571811dec

## Introduction
This is a cross-platform disk benchmarking tool written in Rust because I wanted things fast and smooth. Rather than selecting a disk persay, you select a path including the name of the testfile. This design pattern may change, but for now is a simple way of remaining cross platform and modular. 

## Features
- **Sequential Read/Write Tests**: Assess the performance of your disk with sequential data operations.
- **Random Read/Write Tests**: Measure how your disk handles operations at random locations.
- **IOPS Measurement [COMING SOON]**: Evaluate the number of individual read/write operations your disk can handle per second.
- **Cross-Platform Compatibility**: Tested on Windows, MacOS & Linux
- **Customizable Test Parameters**: Allows users to specify file paths, sizes, and other test parameters.
- **Progress Tracking**: Real-time progress bars during tests for better user experience.

## Installation
To install Disk-Gym, follow these steps:
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone the repository: 
```bash
git clone https://github.com/mewmix/disk-gym
```

3. Navigate to the directory

```bash
cd disk-gym
```
4. Build
```rust
cargo build --release

```

5. Usage 

```bash
cargo run -- "path/to/testfile" -s 10 -t sequential -b 4096


```

or

```bash
cargo run -- "path/to/testfile" -s 10 -t random -b 4096 -n 1000

```

Replace `"path/to/testfile"` with the path to the file you want to test, `-s` for file size in MB, `-b` for buffer size, and `-n` for the number of operations (for random tests).

## Limitations
- Disk performance may vary based on the system's hardware and the specific filesystem used.
- Ensure you have the necessary permissions to read/write to the specified file path.

## Contributing
Contributions to Disk-Gym are welcome! 

