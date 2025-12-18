use clap::{command, Arg};

fn main(){
    let test: Command = command().arg(
        Arg::new("count")
    );
}