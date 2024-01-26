use std::path::PathBuf;

use bucket::get_bucket;
use clap::{arg, command, value_parser};

use crate::config::read_config;

mod bucket;
mod config;

#[tokio::main]
async fn main() {
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

    let bucket = match get_bucket(&config.minio) {
        Ok(bucket_v) => bucket_v,
        Err(err) => {
            eprintln!("unable to get minio bucket: {:?}", err);
            return;
        }
    };
}
