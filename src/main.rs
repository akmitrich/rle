fn main() {
    match rle::get_args() {
        Ok(config) => match rle::run(config) {
            Ok(report) => println!("REPORT:\n{}", report.finalize()),
            Err(ref e) => {
                eprintln!("Finish with error: {}", e);
                std::process::exit(2);
            }
        },
        Err(ref e) => {
            eprintln!("Invalid arguments:\n{}", e);
            std::process::exit(1);
        }
    }
}
