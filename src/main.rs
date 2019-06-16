extern crate clap;

use std::error::Error;
use std::path::PathBuf;
use std::process;

use clap::{App, Arg};
use lindyndns::{expected_config_location, find_config, run};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("lindyndns")
        .version("0.1.0")
        .author("John Ramsden <johnramsden@riseup.net>")
        .about("Linode Dynamic DNS Client")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_file = match matches.value_of("config") {
        Some(c) => PathBuf::from(c),
        None => {
            let cfg = find_config();
            match cfg {
                Ok(c) => match c {
                    Some(c) => c,
                    None => {
                        let conf_expected = expected_config_location();
                        eprintln!("No config file found in system or user directories.");
                        eprintln!("{} or {}", conf_expected.0, conf_expected.1);
                        process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("Error occurred looking for config file");
                    return Err(e);
                }
            }
        }
    };
    let config_file = match config_file.to_str() {
        Some(c) => c,
        None => {
            eprintln!("Config file is invalid unicode.");
            process::exit(1);
        }
    };

    match run(config_file) {
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(1);
        }
        Ok(_e) => Ok(()),
    }
}
