use clap::{Arg, ArgAction, Command};

pub fn args() -> Command {
    Command::new("processes")
        .about("CLI to show processs and process details.")
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .help("Counts the number of processes currently running.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .help("lists the processes currently running.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("cpu")
                .long("cpu")
                .help("Shows CPU usage")
                .action(ArgAction::SetTrue),
        )
}