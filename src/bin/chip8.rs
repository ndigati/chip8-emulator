// Chip-8 Emulator
// Specs:
//  - 35 opcodes each 2 bytes (16 bits) long
//      - Each opcode will be represented as an u16 (16-bit unsigned integer)
//  - 4K memory
//      - Array of size 4096 1 byte integers (u8, 8-bit unsigned integer)
//  - 15 8-bit registers [V0-VE]
//      - Array of size 16 1 byte integers (u8, 8-bit unsigned interger)
//      - VE (last register) is used for the 'carry flag'
//  - Index register (I) (u16)
//  - Program counter (PC) (u16)
//  - Memory Map
//      - 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
//      - 0x050-0x0A0 - Used for the build in 4x5 pixel font set (0-F)
//      - 0x200-0xFFF - Program ROM and work RAM
//  - VF register used for drawing
//  - 2048 pixels used for drawing (64x32)
//      - Use an array to hold either a 0 or 1
//      - Array of size 64 * 32 1 byte integers (u8, 8-bit unsigned interger)
//  - Two timer registers that count at 60 Hz
//      - 1 byte delay timer (u8)
//      - 1 byte sound timer (u8)
//          - Buzzer sounds when sound timer reaches 0
//  - 16 level stack with stack pointer
//      - Array of size 16 2 byte integers (u16 16-bit integer)
//      - Two byte stack pointer (u16)
//  - Hex based keypad (0x0-0xF)
//      - Array of size 16 1 byte (u8) to store current state of the key
extern crate chip8;

use chip8::cpu::Cpu;

fn main() {
    // Set up render system

    // Initialize the Chip-8 System and load the game into memory

    //Emulation loop
    loop {
        //Emulate one cycle

        // If the draw flag is set, update the screen

        // Store the key press state (Press and Release)

        // Putting break here for now so it doesn't do nothing in an infinite loop
        break;
    }
    let mut cpu = Cpu::new();
    println!("Finished running!");
}
