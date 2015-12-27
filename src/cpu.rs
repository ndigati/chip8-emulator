// File to hold CPU functions and stuff
use mem::Mem;

/// 15 8-bit general purpose registers, an Index register, and program counter
struct Registers {
    V0: u8,
    V1: u8,
    V2: u8,
    V3: u8,
    V4: u8,
    V5: u8,
    V6: u8,
    V7: u8,
    V8: u8,
    V9: u8,
    VA: u8,
    VB: u8,
    VC: u8,
    VD: u8,
    VE: u8,     // used for the carry flag
    pc: u16,
    i:  u16
}

pub struct Cpu {
    memory: Mem,
    registers: Registers,
    opcode: u8,
    sp: u16,
    stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: Mem::new(),
            registers: Registers {
                V0: 0,
                V1: 0,
                V2: 0,
                V3: 0,
                V4: 0,
                V5: 0,
                V6: 0,
                V7: 0,
                V8: 0,
                V9: 0,
                VA: 0,
                VB: 0,
                VC: 0,
                VD: 0,
                VE: 0,
                pc: 0x200,
                i:  0,
            },
            opcode: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn emulate_cycle(&self) {
        // Fetch opcode
        let opcode: u16 = self.get_opcode();

        // Decode opcode

        // Execute opcode

        // Update timers
    }
    
    fn get_opcode(&self) -> u16 {
        // opcode = memory[pc] << 8 | memory[pc + 1]
        // The left shift just makes it a 16 bit number (which is the full 2 byte opcode)
        let first: u8 = self.memory.get_memory_from_location(self.registers.pc);
        let second: u8 = self.memory.get_memory_from_location(self.registers.pc + 1);
        // Since rust gives errors when shifting a number by >= number of bits it has
        // Cast first to a u16 to give leading zeros
        // Then shifting first into those leading 8 zeros
        return (first as u16) << 8 | (second as u16);
    }

    fn get_pc(&self) -> u16 {
        return self.registers.pc;
    }

    /// Update the PC register
    fn inc_pc(&mut self) -> u16{
        self.registers.pc += 1;
        return self.registers.pc;
    }
}
