## Rust Compression CLI Tool

Welcome to the Rust Compression CLI tool! This project is a small yet powerful command-line tool built in Rust for basic file compression. It utilizes the `flate2` crate to perform compression using the gzip format.

### How to Use

To use the tool, simply run the following command:

```bash
cargo run <source> <target>
```

Replace `<source>` with the path to the file you want to compress, and `<target>` with the desired name for the compressed file.

### Code Overview

#### Dependencies

The project relies on the `flate2` crate, specifically using the `GzEncoder` and `Compression` for gzip compression. These dependencies are crucial for handling compression tasks.

#### Usage

- `args()` from the `env` module is used to accept the names of the source and target files as command-line arguments.
- `BufReader` is employed to read from the source file, ensuring efficient and buffered reading.
- `File::create` creates the target file where the compressed content will be written.
- `GzEncoder` initializes an encoder for writing compressed data into the target file.

#### Compression Process

- The tool measures the elapsed time using `Instant` to provide insights into the compression duration.
- The `copy` function seamlessly copies the contents of the source file into the encoder, performing the compression.
- The `finish` method finalizes the compression, and metadata information is retrieved for both source and target files.

### Example

```bash
cargo run input.txt compressed_output.gz
```

### Observations

- The tool prints the length of the source and target files, providing insights into the compression efficiency.
- Elapsed time indicates how quickly the compression process completed.

Feel free to explore and modify this Rust CLI Compression Tool. Happy compressing!
