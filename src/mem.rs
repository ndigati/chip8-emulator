// File to hold memory function and stuff

pub struct Mem {
    memory: [u8; 4096],
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            memory: [0; 4096],
        }
    }
    
    pub fn get_memory_from_location(&self, location: u16) -> u8 {
        return self.memory[location as usize];
    }
}
