use clap::{command, Arg};

fn main(){
    let test= command!().arg(
        Arg::new("count")
    );
}