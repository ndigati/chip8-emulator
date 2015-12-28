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
    
    pub fn store(&mut self, location: u16, data: u8) {
        self.memory[location as usize] = data;
    }
}

#[cfg(test)]
mod test {
    use mem::Mem;
    
    #[test]
    fn test_memory() {
        let mut test_mem = Mem::new();
        
        // Make sure it's not set before
        assert_eq!(test_mem.get_memory_from_location(253), 0);
        test_mem.store(253 as u16, 25 as u8);
        // Make sure it's set after
        assert_eq!(test_mem.get_memory_from_location(253), 25);
    }
}
