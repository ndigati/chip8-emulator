// File to hold CPU functions and stuff
//
#[derive(Debug)]
pub struct Cpu {
    // memory: Memory,
    registers: [u8; 16],
    pc: u16,
    I: u16,
    sp: u16,
    stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            pc: 0,
            I: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn print_stuff(&self) {
        println!("{:?}", &self);
    }
}
