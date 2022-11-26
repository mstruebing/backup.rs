use std::fs;
use std::fs::OpenOptions;
use std::process::Command;

use std::io::Write;
use std::path::PathBuf;

pub struct Remotes {
    pub config_path: PathBuf,
}

impl Remotes {
    pub fn add(&self, remote_string: String) -> Result<(), ()> {
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
            println!("{}", remote);
        }
    }

    pub fn transfer(&self, file: PathBuf) -> Result<(), ()> {
        let remotes = self.get_remotes();
        let string_file = &file.into_os_string().into_string().unwrap();

        for remote in remotes {
            println!("Copying to remote: {}", remote);
            let cmd = Command::new("rsync")
                .args(["-azvhP", string_file, &remote])
                .spawn()
                .unwrap();

            let output = cmd.wait_with_output();

            match output {
                Ok(output) => {
                    println!("output: {:?}", output);
                }
                Err(error) => {
                    println!("error: {:?}", error);
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
