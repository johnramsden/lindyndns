extern crate clap;

use std::error::Error;
use std::path::PathBuf;
use std::process;

use clap::{App, Arg};
use lindyndns::{run, MyError, find_config};

fn main() -> Result<(), Box<Error>> {
    let matches = App::new("lindyndns")
        .version("0.0.0")
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
                    None => return Err(Box::new(
                        MyError(String::from("No config file found")))),
                },
                Err(e) => return Err(Box::new(
                    MyError(String::from("Error occurred looking for config file")))),
            }
        },
    };
    let config_file = match config_file.to_str() {
        Some(c) => c,
        None => return Err(Box::new(
            MyError(String::from("Config file is invalid unicode.")))),
    };
    run(config_file)
}
