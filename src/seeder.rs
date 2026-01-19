use sha2::{Digest, Sha256};

pub struct Seeder {
    hasher: Sha256,
}

impl Seeder {
    pub fn new() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }

    pub fn feed(&mut self, data: &[u8]) -> &mut Self {
        self.hasher.update(data);
        self
    }

    pub fn finalize(self) -> [u8; 32] {
        self.hasher.finalize().into()
    }
}