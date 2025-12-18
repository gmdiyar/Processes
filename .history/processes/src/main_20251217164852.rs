use clap::Parser;

#[derive(Parser)]

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    if args.pattern == "count" {
        println!("Number of running processes: ")
    }

    println!("{:?}, {:?}", args.pattern, args.path)
}
