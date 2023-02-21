use clap::{arg, Command};
use config::Config;
use std::path::PathBuf;

mod archive;
mod config;
mod files;
mod remote;

extern crate xdg;

fn cli() -> Command<'static> {
    Command::new("backup")
        .about("A backup CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("run").about("Executes the backup"))
        .subcommand(
            Command::new("files")
                .args_conflicts_with_subcommands(true)
                .about("Subcommands for files")
                .subcommand(
                    Command::new("add")
                        .about("Add files")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(<PATH>  "Path of the file to add")
                                .value_parser(clap::value_parser!(PathBuf)),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove files")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(<PATH>  "Path of the file to remove")
                                .value_parser(clap::value_parser!(PathBuf)),
                        ),
                )
                .subcommand(Command::new("list").about("Lists all files"))
                .subcommand(Command::new("clean").about("Sort files and removes duplicates")),
        )
        .subcommand(
            Command::new("remote")
                .args_conflicts_with_subcommands(true)
                .about("Subcommands for remotes")
                .subcommand(
                    Command::new("add")
                        .about(
                            "Add a remote in an rsync compatible format i.e. `<user>@<host>:<path>`",
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(<String>  "Stuff to add")
                                .value_parser(clap::value_parser!(String)),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                    .about("Remove a remote")
                        .arg_required_else_help(true)
                        .arg(arg!(<REMOTE> "The remote to target")),
                )
                .subcommand(Command::new("list").about("List remotes")),
        )
}

fn main() -> Result<(), ()> {
    let config = Config::new(env!("CARGO_PKG_NAME"));

    let files = files::Files {
        config_path: config.files,
    };
    let remotes = remote::Remotes {
        config_path: config.remotes,
    };

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("run", _)) => {
            println!("Run backup");
            archive::create(files.get_only_existing().unwrap(), config.cache.clone())?;
            remotes.transfer(config.cache)?
        }
        Some(("files", sub_matches)) => {
            let files_command = sub_matches.subcommand().unwrap_or(("list", sub_matches));
            match files_command {
                ("list", _sub_matches) => {
                    files.list()?;
                }
                ("add", sub_matches) => {
                    let paths = sub_matches
                        .get_many::<PathBuf>("PATH")
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>();

                    for path in paths {
                        files.add(path)?;
                        println!("File {} added", path.display())
                    }

                    files.clean()?;
                }
                ("remove", sub_matches) => {
                    let paths = sub_matches
                        .get_many::<PathBuf>("PATH")
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>();

                    for path in paths {
                        files.remove(path)?;
                        println!("File {} removed", path.display())
                    }
                }
                ("clean", _sub_matches) => {
                    files.clean()?;
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some(("remote", sub_matches)) => {
            let remote_command = sub_matches.subcommand().unwrap_or(("list", sub_matches));
            match remote_command {
                ("list", _sub_matches) => {
                    remotes.list();
                }
                ("add", sub_matches) => {
                    let r = sub_matches.get_one::<String>("String").unwrap();

                    println!("Adding remote {:?}", r);
                    remotes.add(r.to_owned()).unwrap();
                }
                ("remove", sub_matches) => {
                    let remote = sub_matches.get_one::<String>("REMOTE").expect("required");
                    println!("Removing remote {:?}", remote);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
