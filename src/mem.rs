// File to hold memory function and stuff

use std::fmt;

pub struct Mem {
    memory: [u8; 4096],
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            memory: [0; 4096],
        }
    }
}
