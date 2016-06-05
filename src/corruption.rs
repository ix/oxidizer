use std::io::prelude::*;
use std::path::Path;
use std::fs::*;
use std::io::Result as IOResult;

#[allow(dead_code)]
pub struct Corroptions {
    pub start:  Option<u64>,
    pub end:    Option<u64>,
    pub skip:   Option<u64>,
    pub action: CorruptMethod
}

#[allow(dead_code)]
pub enum CorruptMethod {
    Increment(u64),
    Decrement(u64),
    Multiply(u64),
    Left(u64),
    Right(u64),
    None
}

#[allow(dead_code)]
impl Corroptions {
    pub fn new() -> Corroptions {
        Corroptions {
            start: None,
            end: None,
            skip: None,
            action: CorruptMethod::None
        }
    }
}

pub trait Mutate {
    fn mutate_to<K: AsRef<Path>>(&mut self, out: K, options: &Corroptions) -> IOResult<()>;
}

impl Mutate for File {
    fn mutate_to<K: AsRef<Path>>(&mut self, out: K, options: &Corroptions) -> IOResult<()> {
        let mut buf = Vec::new();

        for result_byte in self.bytes() {
            let byte = try!(result_byte);
          
            buf.push(byte);
        }
        
        let mut file = try!(File::create(&out)); {
            try!(file.write(&buf));
        }
        
        Ok(())
    }
}
