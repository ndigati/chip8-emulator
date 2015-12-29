#![allow(dead_code)]

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
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,     
    vf: u8,      // used for the carry flag
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
                va: 0,
                vb: 0,
                vc: 0,
                vd: 0,
                ve: 0,
                vf: 0,
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
            0x0000  => {
                match opcode & 0x00FF {
                    0x00E0 => println!("Clears the screen."),
                    0x00EE => println!("Returns from a subroutine."),
                    _       => println!("Calls RCA 1802 program at address NNN. Not necessary for most ROMs.")
                }                              
            },
            0x1000  => println!("Jumps to address NNN."),
            0x2000  => println!("Calls subroutine at NNN."),
            0x3000  => println!("Skips the next instruction if VX equals NN."),
            0x4000  => println!("Skips the next instruction if VX doesn't equal NN."),
            0x5000  => println!("Skips the next instruction if VX equals VY."),
            0x6000  => println!("Sets VX to NN."),
            0x7000  => println!("Adds NN to VX."),
            0x8000  => {
                match opcode & 0x000F {
                    0x0000  => println!("Sets VX to the value of VY."),
                    0x0001  => println!("Sets VX to VX or VY."),
                    0x0002  => println!("Sets VX to VX and VY."),
                    0x0003  => println!("Sets VX to VX xor VY."),
                    0x0004  => println!("Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't."),
                    0x0005  => println!("VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't."),
                    0x0006  => println!("Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift."),
                    0x0007  => println!("Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't."),
                    0x000E  => println!("Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift."),
                    _       => println!("Non used opcode in the 0x8000 range")
                }
            },
            0x9000  => println!("Skips the next instruction if VX doesn't equal VY."),
            0xA000  => println!("Sets I to the address NNN."),
            0xB000  => println!("Jumps to the address NNN plus V0."),
            0xC000  => println!("Sets VX to the result of a bitwise and operation on a random number and NN."),
            0xD000  => println!("Drawing stuff instruction"),
            0xE000  => {
                match opcode & 0x00FF {
                    0x009E  => println!("Skips the next instruction if the key stored in VX is pressed."),
                    0x00A1  => println!("Skips the next instruction if the key stored in VX isn't pressed."),
                    _       => println!("Non used opcode in the 0xE000 range")
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => println!("Sets VX to the value of the delay timer."),
                    0x000A => println!("A key press is awaited, and then stored in VX."),
                    0x0015 => println!("Sets the delay timer to VX."),
                    0x0018 => println!("Sets the sound timer to VX."),
                    0x001E => println!("Adds VX to I."),
                    0x0029 => println!("Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font."),
                    0x0033 => println!("Stores the Binary-coded decimal representation of VX"),
                    0x0055 => println!("Stores V0 to VX in memory starting at address I."),
                    0x0065 => println!("Fills V0 to VX with values from memory starting at address I."),
                    _       => println!("Unused opcode in the 0xF000 range")
                }
            },
            _       => println!("Invalid/Unimplemented opcode")
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
