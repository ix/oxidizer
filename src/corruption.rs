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
    Increment(u8),
    Decrement(u8),
    Multiply(u8),
    Left(u32),
    Right(u32),
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

pub trait Corrupt {
    fn corrupt(self, method: &CorruptMethod) -> Self;
}

impl Corrupt for u8 {
    fn corrupt(self, method: &CorruptMethod) -> Self {
        match *method {
            CorruptMethod::Increment(i) => self.wrapping_add(i),
            CorruptMethod::Decrement(i) => self.wrapping_sub(i),
            CorruptMethod::Multiply(i)  => self.wrapping_mul(i),
            CorruptMethod::Left(i)      => self.wrapping_shl(i),
            CorruptMethod::Right(i)     => self.wrapping_shr(i),
            _ => self
        }
    }        
}

pub trait Mutate {
    fn mutate_to<K: AsRef<Path>>(&mut self, out: K, options: &Corroptions) -> IOResult<()>;
}

impl Mutate for File {
    fn mutate_to<K: AsRef<Path>>(&mut self, out: K, options: &Corroptions) -> IOResult<()> {
        let mut buf = Vec::new();
        let mut i: u64 = 0; // track iterations

        for result_byte in self.bytes() {
            let byte = try!(result_byte);

            // skip if we're not range
            if let Some(start) = options.start {
                if i < start {
                    buf.push(byte);
                    i += 1;
                    continue;
                }
            }

            if let Some(end) = options.end {
                if i > end {
                    buf.push(byte);
                    i += 1;
                    continue;
                }
            }

            // skip if we're skipping
            if let Some(skip) = options.skip {
                if i % skip != 0 {
                    buf.push(byte);
                    i += 1;
                    continue;
                }
            }

            // nothing made us skip, ok
            // push the corrupted byte instead
            buf.push(byte.corrupt(&options.action));
            i += 1;
        }
        
        let mut file = try!(File::create(&out)); {
            try!(file.write(&buf));
        }
        
        Ok(())
    }
}
