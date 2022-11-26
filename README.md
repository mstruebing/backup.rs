# backup.rs

A CLI tool to manage and run your backups.

!!! This repository is in an early WIP state right now.

# Dependencies

This tool uses rsync to transfer these files so you need rsync installed.

# Installation

## Crates.io

`cargo install back`

## Raw

Clone the repository and run `cargo build --release` and you should find the binary in `./target/release/back`.

## Release Page

Or grab a binary from the [release page](https://github.com/mstruebing/backup.rs/releases)

# Contribution

- Fork this project
- Create a branch
- Provide a pull request

The CI will lint your commit message with [commitlint](https://commitlint.js.org/#/).
