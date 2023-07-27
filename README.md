# Cologrizer

A simple colorizer for your logs written in Rust.

## Build

Standard Rust build, check to have rust installed on your system, go to the directory of the project and then:

```bash
$ cargo build --release
```

The binary will be in `target/release/cologrizer`.

## Usage

Move the `cologrizer` binary to your `/usr/local/bin` folder and then run it like this:

```bash
$ cat /var/log/system.log | cologrizer
```
