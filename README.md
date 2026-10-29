# rgp

`rgp` is a small recursive text search program written in Rust. It searches files beneath the current working directory for a keyword, ignoring letter case, and prints matching lines with their file paths.

## Requirements

- Rust and Cargo

## Usage

From the directory you want to search, run:

```sh
cargo run -- <keyword>
```

For example:

```sh
cargo run -- lighthouse
```

The search includes files in nested directories. Matching line numbers start at `0`. Files that cannot be read as UTF-8 text are skipped.

If you run the program without a keyword, it prints a prompt:

```sh
cargo run
```
