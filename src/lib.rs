mod error;
mod report;
pub mod rle_adv;
pub mod rle_naive;

use crate::error::MyResult;
use clap::{command, Arg};
use error::MyError;
use report::{Report, ReportBuilder};
use std::{
    ffi::OsString,
    fs::{metadata, File},
    io::{BufReader, BufWriter},
    path::Path,
};

pub fn run(config: Config) -> MyResult<Report> {
    println!("Take: {:?}", config.file_path);
    let mut report = ReportBuilder::new();
    let input = File::open(&config.file_path)?;
    let input = BufReader::new(input);
    let new_filename = make_filename(&config.file_path, config.action)?;
    let output = File::create(&new_filename)?;
    let output = BufWriter::new(output);
    let input_info = metadata(&config.file_path)?;
    match config.action {
        Action::Encode => {
            match config.method {
                Method::Naive => rle_naive::pack(input, output)?,
                Method::Advanced => rle_adv::encode(input, output)?,
            }
            let output_info = metadata(&new_filename)?;
            report.set_compressed(output_info.len() as _);
            report.set_origin(dbg!(input_info.len()) as _);
        }
        Action::Decode => {
            rle_naive::unpack(input, output)?;
            let output_info = metadata(&new_filename)?;
            report.set_compressed(input_info.len() as _);
            report.set_origin(output_info.len() as _);
        }
    }
    report.finalize()
}

fn make_filename(path: impl AsRef<Path>, action: Action) -> MyResult<OsString> {
    let filename = path
        .as_ref()
        .file_name()
        .ok_or(MyError::UnexpectedFilePath)?;
    let mut filename = filename.to_os_string();
    match action {
        Action::Encode => filename.push(".rle"),
        Action::Decode => {
            filename = OsString::from(
                filename
                    .to_str()
                    .unwrap()
                    .to_string()
                    .strip_suffix(".rle")
                    .unwrap_or_else(|| filename.to_str().unwrap()),
            );
        }
    };
    println!("and do the work to the file: {:?}", filename);
    Ok(filename)
}

#[derive(Debug)]
pub struct Config {
    file_path: String,
    action: Action,
    method: Method,
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
        .arg(
            Arg::new("naive")
                .help("Use 'naive' encoding")
                .num_args(0)
                .short('n'),
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
            (true, false) => Action::Encode,
            (false, true) => Action::Decode,
        },
        method: match matches.get_flag("naive") {
            true => Method::Naive,
            false => Method::Advanced,
        },
    })
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Encode,
    Decode,
}

#[derive(Debug, Clone, Copy)]
enum Method {
    Naive,
    Advanced,
}
