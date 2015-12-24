// File to hold CPU functions and stuff
//
#[derive(Debug)]
pub struct Cpu {
    // memory: Memory,
    registers: [u8; 16],
    pc: u16,
    opcode: u8,
    I: u16,
    sp: u16,
    stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            pc: 0x200,              // Program counter starts at 0x200
            opcode: 0,
            I: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn emulate_cycle() {
        // Fetch opcode

        // Decode opcode

        // Execute opcode

        // Update timers
    }
}
