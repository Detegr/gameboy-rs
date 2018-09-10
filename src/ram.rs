use std::ops::{Deref, DerefMut};
pub struct Ram {
    memory: Box<[u8]>,
}
impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: vec![0; 65536].into_boxed_slice(),
        }
    }

    #[cfg(test)]
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            self.memory[i] = *b;
        }
    }
}
impl Deref for Ram {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}
impl DerefMut for Ram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memory
    }
}
