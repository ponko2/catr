fn main() {
    if let Err(err) = catr::run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
