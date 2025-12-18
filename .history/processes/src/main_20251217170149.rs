use clap::Parser;

#[derive(Parser)]
#[command(hello)]
struct Cli {
    #[arg(short, long)]
    verbose: bool
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
