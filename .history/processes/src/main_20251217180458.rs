use clap::{command, Arg};

fn main(){
    let test: Command = command!().Arg(
        Arg::new("count")
    );
}