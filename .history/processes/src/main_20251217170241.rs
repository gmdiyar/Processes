use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool
}

fn main() {
    let args = Cli::parse();

    if args.pattern == "count" {
        println!("{}", Cli.verbose)
    }

    if args.pattern == "show" {
        println!("Processes: ")
    }
}
