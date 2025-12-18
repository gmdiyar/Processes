use sysinfo::*;
mod args;

fn main() {
    let cmd = args::args();
    let args = cmd.get_matches();
    let mut sys = System::new();

    if args.get_flag("count") {
        
    }

    if args.get_flag("list") {
        sys.refresh_all();
        for (pid, process) in sys.processes() {
            println!("{:?}", process.name())
        }
    }

    if args.get_flag("cpu") {
        // show cpu usage as percentage
    }
}
