extern crate getopts;

mod corruption;

use getopts::Options;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
    
    options.optflag("h", "help", "print this help menu");
    options.optopt("f", "file", "input filename", "FILE");
    options.optopt("o", "output", "output filename", "OUTPUT");
    
    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        let brief = format!("usage: {} [options]", program);
        print!("{}", options.usage(&brief));
        return;
    }

    if matches.opt_str("f").is_none() || matches.opt_str("o").is_none() {
        println!("Both an input filename and output filename must be provided!");
        return;
    }
}
