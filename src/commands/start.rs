use std::{fs::File, io::Read};
use serde::{Serialize, Deserialize};
use crate::{log::{self, LogType}, srcds, FOLDER, SETTINGS_FILE};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config {
    srcds_path: String,
    srcds_args: Vec<String>
}

fn open_config() -> Result<File, std::io::Error> {
    match File::open(&format!("{}\\{}", FOLDER, SETTINGS_FILE)) {
        Ok(file) => Ok(file),
        Err(err) => Err(err),
    }
}

static CONSOLE_ARG: &str = "-console";

fn start(yml: Config) {
    let mut args = yml.srcds_args;
    let console_arg = CONSOLE_ARG.to_string();

    if !args.contains(&console_arg) {
        args.push(console_arg);
    }

    srcds::start(yml.srcds_path, args);
}

#[allow(unused_must_use)]
pub fn on_execute() {
    let mut config_file = match open_config() {
        Ok(file) => file,
        Err(err) => return log::print(LogType::Error, &format!("Unable to open {SETTINGS_FILE}: {}", err.to_string())),
    };

    let mut config = String::new();
    config_file.read_to_string(&mut config);

    match serde_yaml::from_str::<Config>(&config) {
        Ok(yml) => {
            start(yml);
        },
        Err(err) => return log::print(LogType::Error, &format!("Unable to scan {SETTINGS_FILE}: {}", err.to_string()))
    };
}