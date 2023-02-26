use clap::{arg, Arg, Command};
use config::Config;
use std::path::PathBuf;

mod archive;
mod config;
mod files;
mod logger;
mod remotes;

extern crate xdg;

fn cli() -> Command<'static> {
    Command::new("backup")
        .about("A backup CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(Arg::new("debug").long("debug").short('d').help("Run with debug output"))
        .arg(Arg::new("verbose").long("verbose").short('v').help("Run with verbose output"))
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
            Command::new("remotes")
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
    let matches = cli().get_matches();

    let logger = logger::Logger {
        verbose: matches.contains_id("verbose"),
        debug: matches.contains_id("debug"),
    };

    let config = Config::new(env!("CARGO_PKG_NAME"), &logger);
    let files = files::Files {
        config_path: config.files.clone(),
        logger: &logger,
    };
    let remotes = remotes::Remotes {
        config_path: config.remotes.clone(),
        logger: &logger,
    };

    logger.debug(&format!("matches: {:#?}", matches));
    logger.debug(&format!("config: {:#?}", config));

    match matches.subcommand() {
        Some(("run", _)) => {
            archive::create(
                files.get_only_existing().unwrap(),
                config.cache.clone(),
                &logger,
            )?;
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
        Some(("remotes", sub_matches)) => {
            let remote_command = sub_matches.subcommand().unwrap_or(("list", sub_matches));
            match remote_command {
                ("list", _sub_matches) => {
                    remotes.list();
                }
                ("add", sub_matches) => {
                    let r = sub_matches.get_one::<String>("String").unwrap();
                    remotes.add(r.to_owned()).unwrap();
                }
                ("remove", sub_matches) => {
                    let remote = sub_matches.get_one::<String>("REMOTE").expect("required");
                    // TODO: implement
                    logger.log(&format!("Removing remote {:?}", remote))
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
