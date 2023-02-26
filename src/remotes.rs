use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::logger::Logger;

pub struct Remotes<'a> {
    pub config_path: PathBuf,
    pub logger: &'a Logger,
}

impl Remotes<'_> {
    pub fn add(&self, remote_string: String) -> Result<(), ()> {
        self.logger.log(&format!("Adding remote {}", remote_string));
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.config_path)
            .unwrap();

        writeln!(file, "{}", remote_string).unwrap();

        Ok(())
    }

    pub fn list(&self) {
        let remotes = self.get_remotes();
        for remote in remotes {
            self.logger.log(&remote);
        }
    }

    pub fn transfer(&self, file: PathBuf) -> Result<(), ()> {
        let remotes = self.get_remotes();
        let string_file = &file.into_os_string().into_string().unwrap();

        for remote in remotes {
            self.logger
                .log(&format!("Transfering to remote: {}", remote));
            let cmd = Command::new("rsync")
                .args(["-azvhP", string_file, &remote])
                .spawn()
                .unwrap();

            let output = cmd.wait_with_output();

            match output {
                Ok(output) => {
                    self.logger.log(&format!("Rsync output: {:#?} ", output));
                    self.logger
                        .log(&format!("Transer to remote: {} successful", remote));
                }
                Err(error) => {
                    self.logger.debug(&format!("Rsync error: {:#?} ", error));
                    self.logger
                        .log(&format!("Transer to remote: {} failed", remote));
                }
            }
        }

        Ok(())
    }

    fn get_remotes(&self) -> Vec<String> {
        let config_string = fs::read_to_string(&self.config_path).unwrap_or_else(|_| {
            panic!(
                "Can not read config file: {}",
                self.config_path.to_str().unwrap()
            )
        });

        let remotes: Vec<String> = config_string.lines().map(String::from).collect();
        remotes
    }
}
