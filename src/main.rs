extern crate clap;

use clap::{Arg, App};

use lindyndns::run;


fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("lindyndns")
                    .version("0.0.0")
                    .author("John Ramsden <johnramsden@riseup.net>")
                    .about("Linode Dynamic DNS Client")
                    .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file")
                        .takes_value(true))
                    .get_matches();

    let config_file = matches.value_of("config").unwrap_or("config.toml");

    run(config_file)
}
