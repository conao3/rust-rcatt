# rcatt

A simple, high-performance `cat` command implementation in Rust.

## Overview

rcatt is a minimal implementation of the Unix `cat` utility, designed to efficiently copy standard input to standard output. It leverages Rust's `io::copy` with locked stdin/stdout handles for maximum throughput.

## Installation

```sh
cargo install --path .
```

Or build from source:

```sh
cargo build --release
```

## Usage

```sh
# Pipe content through rcatt
echo "Hello, World" | rcatt

# Copy file contents to stdout
rcatt < input.txt
```

## Performance

Benchmarks measured using `pv` on the output stream:

| Implementation | Throughput |
|----------------|------------|
| Theoretical max (`yes \| pv`) | 5.14 GiB/s |
| **rcatt** | 4.52 GiB/s |
| GNU cat 9.1 | 2.78 GiB/s |

rcatt achieves approximately 88% of the theoretical maximum throughput, outperforming GNU cat by over 60%.

### Reproducing Benchmarks

```sh
# Theoretical maximum
yes | pv -F '  %a' > /dev/null

# rcatt
yes | cargo run --release | pv -F '  %a' > /dev/null

# GNU cat (baseline)
yes | cat | pv -F '  %a' > /dev/null
```

## License

Apache-2.0
