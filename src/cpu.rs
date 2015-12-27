// File to hold CPU functions and stuff
use mem::Mem;

/// 15 8-bit general purpose registers, an Index register, and program counter
struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    vA: u8,
    vB: u8,
    vC: u8,
    vD: u8,
    vE: u8,     // used for the carry flag
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
                v0: 0,
                v1: 0,
                v2: 0,
                v3: 0,
                v4: 0,
                v5: 0,
                v6: 0,
                v7: 0,
                v8: 0,
                v9: 0,
                vA: 0,
                vB: 0,
                vC: 0,
                vD: 0,
                vE: 0,
                pc: 0x200,
                i:  0,
            },
            opcode: 0,
            sp: 0,
            stack: [0; 16],
        }
    }
    
    /// Get the next opcode in memory (only after incrementing the program counter)
    fn get_opcode(&self) -> u16 {
        // opcode = memory[pc] << 8 | memory[pc + 1]
        // The left shift just makes it a 16 bit number (which is the full 2 byte opcode)
        let first: u8 = self.memory.get_memory_from_location(self.get_pc());
        let second: u8 = self.memory.get_memory_from_location(self.get_pc() + 1);
        // Since rust gives errors when shifting a number by >= number of bits it has
        // Cast first to a u16 to give leading zeros
        // Then shifting first into those leading 8 zeros
        return (first as u16) << 8 | (second as u16);
    }
    
    fn decode_opcode(&self, opcode: u16) {
        match opcode & 0xF000 {
            _ => println!("Doing nothing!")
        }
    }

    fn get_pc(&self) -> u16 {
        return self.registers.pc;
    }

    /// Update the PC register and return it
    fn inc_pc(&mut self) -> u16 {
        // Must increment pc by 2 since opcodes are 2 bytes
        // And each space in memory is only 1 byte
        self.registers.pc += 2;
        return self.registers.pc;
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        let opcode: u16 = self.get_opcode();

        // Decode opcode and execute opcode
        self.decode_opcode(opcode);
        
        // Update timers
        self.inc_pc();
    }
}
