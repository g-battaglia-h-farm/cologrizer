# Cologrizer

A simple colorizer for your logs written in Rust.

## Build

Standard Rust build:

```bash
$ cargo build --release
```

The binary will be in `target/release/cologrizer`.

## Usage

Move the `cologrizer` binary to your `/usr/local/bin` folder and then run it like this:

```bash
$ cat /var/log/system.log | cologrizer
```
