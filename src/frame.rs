#[derive(Debug, Clone)]
pub struct Frame(Vec<u8>);

impl Frame {
    pub fn new(data: Vec<u8>) -> Self {
        Frame(data)
    }
}

impl AsRef<[u8]> for Frame {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}