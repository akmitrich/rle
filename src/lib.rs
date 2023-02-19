mod error;
mod report;
pub mod rle_naive;

use crate::error::MyResult;
use clap::{command, Arg};
use error::MyError;
use report::{Report, ReportBuilder};
use std::{fs, io, path::Path};

pub fn run(config: Config) -> MyResult<Report> {
    let content = load_file(&config.file_path)?;
    let mut report = ReportBuilder::new();
    match config.action {
        Action::Zip => {
            report.set_compressed(rle_naive::encode(&content));
            report.set_origin(content);
        }
        Action::Unzip => {
            report.set_origin(rle_naive::decode(&content));
            report.set_compressed(content);
        }
    };
    report.finalize()
}

fn load_file(file_path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    fs::read(file_path)
}

#[derive(Debug)]
pub struct Config {
    file_path: String,
    action: Action,
}

pub fn get_args() -> MyResult<Config> {
    let matches = command!()
        .version("0.1.0")
        .author("Alexander Kalashnikov <ak.mitrich@mail.ru>")
        .about("Rust byte RLE compress/decompress")
        .arg(
            Arg::new("filename")
                .value_name("PATH")
                .help("Input file name")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("compress")
                .help("Compress 'path' into current directory")
                .num_args(0)
                .short('z'),
        )
        .arg(
            Arg::new("decompress")
                .help("Decompress 'path' into current directory")
                .num_args(0)
                .short('u'),
        )
        .get_matches();
    let file_path = matches
        .get_raw("filename")
        .unwrap()
        .next()
        .unwrap()
        .to_string_lossy()
        .to_string();
    Ok(Config {
        file_path,
        action: match (matches.get_flag("compress"), matches.get_flag("decompress")) {
            (true, true) | (false, false) => return Err(Box::new(MyError::InvalidArguments)),
            (true, false) => Action::Zip,
            (false, true) => Action::Unzip,
        },
    })
}

#[derive(Debug)]
enum Action {
    Zip,
    Unzip,
}
