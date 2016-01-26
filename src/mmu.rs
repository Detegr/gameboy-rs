use std::ops::{Deref, DerefMut};
pub struct Mmu {
    memory: Box<[u8]>
}
impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            memory: vec![0; 65536].into_boxed_slice()
        }
    }

    #[cfg(test)]
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            self.memory[i] = *b;
        }
    }
}
impl Deref for Mmu {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}
impl DerefMut for Mmu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memory
    }
}
