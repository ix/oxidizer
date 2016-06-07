extern crate getopts;
extern crate rustc_serialize;

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
    options.optopt("i", "increment", "increment by", "N");
    options.optopt("d", "decrement", "decrement by", "N");
    options.optopt("m", "multiply", "multiply by", "N");
    options.optopt("l", "shift-left", "shift left by", "N");
    options.optopt("r", "shift-right", "shift right by", "N");
    options.optopt("s", "skip", "corrupt every n bytes", "N");
    options.optopt("b", "begin", "offset to begin at", "OFFSET");
    options.optopt("e", "end", "offset to end at", "OFFSET");
    
    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        let brief = format!("usage: {} [options]", program);
        print!("{}", options.usage(&brief));
        println!("Offsets are hex (prefixed with '0x')");
        return;
    }

    if matches.opt_str("f").is_none() {
        println!("Input or output filename is missing!");
        return;
    }

    if let Some(inc) = matches.opt_str("i") {
        corrupt.action = CorruptMethod::Increment(inc.parse::<u8>().unwrap());
    }

    if let Some(dec) = matches.opt_str("d") {
        corrupt.action = CorruptMethod::Decrement(dec.parse::<u8>().unwrap());
    }

    if let Some(mul) = matches.opt_str("m") {
        corrupt.action = CorruptMethod::Multiply(mul.parse::<u8>().unwrap());
    }

    if let Some(shl) = matches.opt_str("l") {
        corrupt.action = CorruptMethod::Left(shl.parse::<u32>().unwrap());
    }

    if let Some(shr) = matches.opt_str("r") {
        corrupt.action = CorruptMethod::Right(shr.parse::<u32>().unwrap());
    }

    if let Some(skip) = matches.opt_str("s") {
        corrupt.skip = Some(skip.parse::<u64>().unwrap());
    }

    if let Some(start) = matches.opt_str("b") {
        corrupt.start = Some(u64::from_str_radix(&start[2..], 16).unwrap());
    }

    if let Some(end) = matches.opt_str("e") {
        corrupt.end = Some(u64::from_str_radix(&end[2..], 16).unwrap());
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
