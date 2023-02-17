use std::{error::Error, fs, io, path::Path};

use clap::{command, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum MyError {
    InvalidArguments,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                MyError::InvalidArguments => "Invalid flags combination.",
            }
        )
    }
}

impl Error for MyError {}

pub fn run(config: Config) -> MyResult<()> {
    let content = load_file(&config.file_path)?;
    println!("Content:\n{:?}\nLoaded {} bytes", content, content.len());
    let output = match config.action {
        Action::Zip => zip(&content),
        Action::Unzip => unzip(&content),
    };
    println!("Complete the file: {:?}", config.file_path);
    println!("Output = {:?}", output);
    Ok(())
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
        .about("Rust RLE compress/decompress")
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

enum State {
    Relax,
    Run(u8, u8),
}

fn zip(input: &[u8]) -> Vec<u8> {
    fn write_output(output: &mut Vec<u8>, byte: u8, run: u8) {
        match run {
            0 => unreachable!(),
            1 => output.push(byte),
            _ => {
                output.push(byte);
                output.push(byte);
                output.push(run);
            }
        }
    }
    use State::*;
    let mut output = vec![];
    let mut state = Relax;
    for byte in input {
        state = match state {
            Relax => Run(*byte, 1),
            Run(current, run) if run < u8::MAX && *byte == current => Run(current, run + 1),
            Run(current, run) => {
                write_output(&mut output, current, run);
                Run(*byte, 1)
            }
        }
    }
    if let Run(byte, run) = state {
        write_output(&mut output, byte, run);
    }
    output
}

fn unzip(input: &[u8]) -> Vec<u8> {
    println!("Hello, unzip!");
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zip_n_bytes() {
        assert_eq!(vec![4, 4, 4], zip(&[4, 4, 4, 4]));
    }

    #[test]
    fn compressed_longer_than_origin() {
        assert_eq!(
            vec![4, 4, 2, 11, 0, 0, 3, 21],
            zip(&[4, 4, 11, 0, 0, 0, 21])
        )
    }

    #[test]
    fn wiki_rle_example() {
        let example = b"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";
        let compressed = [
            b'W', b'W', 12, b'B', b'W', b'W', 12, b'B', b'B', 3, b'W', b'W', 24, b'B', b'W', b'W',
            14,
        ]; //"WW12BWW12BB3WW24BWW14"
        assert_eq!(compressed, zip(example).as_slice());
    }
}
