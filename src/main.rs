extern crate getopts;

mod corruption;

use getopts::Options;
use std::env;
use std::fs::*;
use corruption::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut corrupt = Corroptions::new();
    
    let mut options = Options::new();
    
    options.optflag("h", "help", "print this help menu");
    options.optopt("f", "file", "input filename", "FILE");
    
    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        let brief = format!("usage: {} [options]", program);
        print!("{}", options.usage(&brief));
        return;
    }

    if matches.opt_str("f").is_none() {
        println!("Input or output filename is missing!");
        return;
    }

    // It's safe to unwrap the option here because of the above check.
    let mut file = match File::open(matches.opt_str("f").unwrap()) {
        Ok(f) => f,
        Err(_) => {
            println!("Input file does not exist!");
            return;
        }
    };
    
    match file.mutate_to("mutated.bin", &corrupt) {
        Ok(_) => println!("Successfully corrupted the file."),
        Err(_) => println!("Failed to corrupt the file!")
    }
}
