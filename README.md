# backup.rs

A CLI tool to manage and run your backups.

# How it works

Most of the backup solutions out there are way to complex, this is why I've created this simple tool.
It has a configuration file which contains all files or directories you want to back up and a different configuration file
which contains the destinations where the backup should be placed in an rsync compatible format.
When you start the backup there is a gzipped tarball created which then gets transferred to all remotes one by one.
And that's it. No magic, no surprises.

The configuration files are placed in `$XDG_CONFIG_HOME` and the tarball is placed in `$XDG_CACHE_HOME`

Here is a table showing the different commands

| command | subcommand | description                                                          | example                                                |
| ------- | ---------- | -------------------------------------------------------------------- | ------------------------------------------------------ |
| files   | list       | list all files                                                       | `sback files list`                                     |
| files   | add        | add a file                                                           | `sback files add ./README.md`                          |
| files   | remove     | remove a file                                                        | `sback files remove $PWD/README.md`                    |
| files   | clean      | sorts the file list and removes duplicates in case of manual editing | `sback files clean`                                    |
| remotes | list       | list all remote                                                      | `sback remotes list`                                   |
| remotes | add        | add a remote                                                         | `sback remote add backup-user@12.98.34.76:~/backup`    |
| remotes | remove     | remove a remote                                                      | `sback remote remove backup-user@12.98.34.76:~/backup` |
| run     | -          | executes the backup process                                          | `sback run`                                            |

# Dependencies

This tool uses rsync to transfer these files so you need rsync installed.

# Installation

## Crates.io

`cargo install sback`

## Raw

Clone the repository and run `cargo build --release` and you should find the binary in `./target/release/sback`.

## Release Page

Or grab a binary from the [release page](https://github.com/mstruebing/backup.rs/releases)

# Contribution

- Fork this project
- Create a branch
- Provide a pull request

The CI will lint your commit message with [commitlint](https://commitlint.js.org/#/).
