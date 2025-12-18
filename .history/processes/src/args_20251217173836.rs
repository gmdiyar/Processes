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
    ///Displays the currently running processes by name only.
    pub show: String
}