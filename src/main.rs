fn main() {
    struct Cli {
        pattern: String,
        path: std::path::PathBuf,
    }

    let pattern = std::env::args().nth(0).expect("no pattern given");
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
        pattern: pattern,
    };
}