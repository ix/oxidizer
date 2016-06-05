use std::io::prelude::*;
use std::path::Path;
use std::fs::*;
use std::io::Result as IOResult;

pub trait Mutate {
    fn mutate_to<K: AsRef<Path>>(&mut self, out: K) -> IOResult<()>;
}

impl Mutate for File {
    fn mutate_to<K: AsRef<Path>>(&mut self, out: K) -> IOResult<()> {
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
