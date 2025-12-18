use clap::Parser;

#[derive(Parser)]

struct Cli {
    pattern: String,
}

fn main() {
    let args = Cli::parse();

    if args.pattern == "count" {
        println!("Number of running processes: ")
    }

    if args.pattern == "show" {
        println!("Processes: ")
    }
}
