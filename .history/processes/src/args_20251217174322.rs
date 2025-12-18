use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(version, about)]

pub struct Arguments{
    ///Displays the number of currently running processes.
    pub count: String,
    ///Lists all running processes by process name only.
    pub names: String
}