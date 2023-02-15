fn main() {
    if let Err(e) = rle::get_args().and_then(rle::run) {
        eprintln!("Finish with error:\n{}", e);
        std::process::exit(1);
    }
}
