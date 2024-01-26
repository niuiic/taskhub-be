use std::path::PathBuf;

use clap::{arg, command, value_parser};

use crate::config::read_config;

mod config;

fn main() {
    let args = command!()
        .arg(
            arg!(-c --config <FILE> "select config file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let config = match read_config(args.get_one::<PathBuf>("config")) {
        Ok(config_v) => config_v,
        Err(err) => {
            eprintln!("unable to get config: {:?}", err);
            return;
        }
    };
}
