#[derive(Debug, Clone)]
pub struct Frame(Vec<u8>);

impl Frame {
    pub fn new(data: Vec<u8>) -> Self {
        Frame(data)
    }

    pub fn diff(&self, other: &Frame) -> Vec<u8> {
        self.as_ref()
            .iter()
            .zip(other.as_ref())
            .map(|(a, b)| a.abs_diff(*b))
            .collect()
    }
}

impl AsRef<[u8]> for Frame {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
