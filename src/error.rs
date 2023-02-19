use std::error::Error;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum MyError {
    InvalidArguments,
    InvalidReport,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                MyError::InvalidArguments => "Invalid flags combination.",
                MyError::InvalidReport => "Invalid report.",
            }
        )
    }
}

impl Error for MyError {}
